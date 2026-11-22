// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! English UI strings are the lookup keys and the fallback. Add a translation to
//! `i18n/zh-CN.json` under `messages` using the English literal or format string as
//! the key. Catalog values are Fluent patterns, so an English `{name}` becomes
//! `{ $name }` in the translation and arguments can be reordered or omitted.
//! Use `contexts` for distinct senses of the same English text and for labels
//! translated by a shared component before measuring or wrapping.
//! A context entry set to `null` uses the matching translation in `messages`.

use std::collections::HashMap;
use std::fmt::Write;
use std::sync::OnceLock;

use fluent::FluentArgs;
use fluent::FluentResource;
use fluent::concurrent::FluentBundle;
use fluent_langneg::NegotiationStrategy;
use fluent_langneg::negotiate_languages;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::unic_langid::LanguageIdentifier;

const ZH_CN: &str = include_str!("../i18n/zh-CN.json");

#[derive(serde::Deserialize)]
struct CatalogSource {
    messages: HashMap<String, String>,
    #[serde(default)]
    contexts: HashMap<String, HashMap<String, Option<String>>>,
}

struct Message {
    id: String,
    plain: Option<String>,
}

struct Catalog {
    messages: HashMap<String, Message>,
    contexts: HashMap<String, HashMap<String, Message>>,
    bundle: FluentBundle<FluentResource>,
}

impl Catalog {
    fn load(language: LanguageIdentifier, json: &str) -> anyhow::Result<Self> {
        let source: CatalogSource = serde_json::from_str(json)?;
        let mut fluent_source = String::new();
        let mut count = 0;
        let mut index = |messages: HashMap<String, String>| {
            messages
                .into_iter()
                .map(|(english, translation)| {
                    // Fluent needs identifiers. These are private to this bundle.
                    let id = format!("message-{count}");
                    count += 1;
                    writeln!(fluent_source, "{id} = {translation}").expect("writing to a String");
                    (english, Message { id, plain: None })
                })
                .collect::<HashMap<_, _>>()
        };
        let contexts = source
            .contexts
            .into_iter()
            .map(|(context, translations)| {
                let messages = translations
                    .into_iter()
                    .filter_map(|(english, translation)| {
                        let translation =
                            translation.or_else(|| source.messages.get(&english).cloned())?;
                        Some((english, translation))
                    })
                    .collect();
                (context, index(messages))
            })
            .collect::<HashMap<_, _>>();
        let messages = index(source.messages);
        let resource = FluentResource::try_new(fluent_source)
            .map_err(|(_, errors)| anyhow::anyhow!("invalid Fluent translations: {errors:?}"))?;
        let mut bundle = FluentBundle::new_concurrent(vec![language]);
        // Bidi isolation marks would be visible in a terminal.
        bundle.set_use_isolating(false);
        bundle
            .add_resource(resource)
            .map_err(|errors| anyhow::anyhow!("invalid translation bundle: {errors:?}"))?;
        let mut catalog = Self {
            messages,
            contexts,
            bundle,
        };
        for message in catalog
            .messages
            .values_mut()
            .chain(catalog.contexts.values_mut().flat_map(HashMap::values_mut))
        {
            message.plain = Self::format(&catalog.bundle, message, /*args*/ None);
        }
        Ok(catalog)
    }

    fn format(
        bundle: &FluentBundle<FluentResource>,
        message: &Message,
        args: Option<&FluentArgs<'_>>,
    ) -> Option<String> {
        let pattern = bundle.get_message(&message.id)?.value()?;
        let mut errors = Vec::new();
        let text = bundle.format_pattern(pattern, args, &mut errors);
        errors.is_empty().then(|| text.into_owned())
    }

    fn contextual(&self, context: &str, source: &str) -> Option<&Message> {
        self.contexts.get(context)?.get(source)
    }

    fn lookup(&self, context: Option<&str>, source: &str) -> Option<&Message> {
        context
            .and_then(|context| self.contextual(context, source))
            .or_else(|| self.messages.get(source))
    }
}

static CATALOG: OnceLock<Option<Catalog>> = OnceLock::new();

fn select_catalog(requested: &[LanguageIdentifier]) -> anyhow::Result<Option<Catalog>> {
    let english: LanguageIdentifier = "en-US".parse()?;
    let chinese: LanguageIdentifier = "zh-CN".parse()?;
    let available = [english, chinese.clone()];
    let selected = negotiate_languages(
        requested,
        &available,
        /*default*/ None,
        NegotiationStrategy::Filtering,
    );
    if selected.first() == Some(&&chinese) {
        return Catalog::load(chinese, ZH_CN).map(Some);
    }
    Ok(None)
}

/// Selects the configured or desktop language before the first frame.
pub(crate) fn init(configured: Option<&str>) {
    CATALOG.get_or_init(|| {
        let requested = match configured.map(str::parse) {
            Some(Ok(language)) => vec![language],
            Some(Err(err)) => {
                tracing::warn!("ignoring [ui] language = {configured:?}: {err}");
                DesktopLanguageRequester::requested_languages()
            }
            None => DesktopLanguageRequester::requested_languages(),
        };
        select_catalog(&requested).unwrap_or_else(|err| {
            tracing::warn!("failed to load TUI translations: {err}");
            None
        })
    });
}

fn catalog() -> Option<&'static Catalog> {
    CATALOG.get()?.as_ref()
}

pub(crate) fn translate<'a>(source: &'a str, context: Option<&str>) -> &'a str {
    catalog()
        .and_then(|catalog| catalog.lookup(context, source))
        .and_then(|message| message.plain.as_deref())
        .unwrap_or(source)
}

/// Looks up labels supplied to a shared component, where other text may also
/// pass through the same field. Only that component's catalog entries apply.
pub(crate) fn translate_in<'a>(context: &str, source: &'a str) -> &'a str {
    scoped_translation(context, source).unwrap_or(source)
}

pub(crate) fn scoped_translation(context: &str, source: &str) -> Option<&'static str> {
    catalog()?.contextual(context, source)?.plain.as_deref()
}

pub(crate) fn translate_owned_in(context: &str, source: String) -> String {
    scoped_translation(context, &source).map_or(source, str::to_owned)
}

pub(crate) fn scoped_translator(context: &str) -> Option<impl Fn(&str) -> Option<&'static str>> {
    let messages = catalog()?.contexts.get(context)?;
    Some(move |source: &str| messages.get(source)?.plain.as_deref())
}

pub(crate) fn translate_format<'a>(
    source: &str,
    context: Option<&str>,
    args: impl FnOnce() -> FluentArgs<'a>,
) -> Option<String> {
    let catalog = catalog()?;
    let message = catalog.lookup(context, source)?;
    Catalog::format(&catalog.bundle, message, Some(&args()))
}

/// Looks up a literal or a named Rust format string. Pass captured variables by
/// name, and use `name = expression` for other arguments.
///
/// Plural messages use the plural English template as the catalog key and keep
/// the singular template at the call site.
macro_rules! tr {
    (in $context:literal, $($message:tt)+) => {
        $crate::i18n::tr!(@translate Some($context), $($message)+)
    };
    (@translate $context:expr, $source:literal $(,)?) => {
        $crate::i18n::translate($source, $context)
    };
    (@translate $context:expr, $one:literal | $other:literal,
     $count:ident $(= $count_value:expr)? $(, $name:ident $(= $value:expr)?)* $(,)?) => {
        $crate::i18n::tr!(
            @translate $context, $other, $count $(= $count_value)? $(, $name $(= $value)?)*;
            if $count == 1 { format!($one) } else { format!($other) }
        )
    };
    (@translate $context:expr, $source:literal, $($name:ident $(= $value:expr)?),+; $fallback:expr $(,)?) => {{
        let ($($name,)+) = ($($crate::i18n::tr!(@value $name $(= $value)?),)+);
        let fallback = $fallback;
        $crate::i18n::translate_format($source, $context, || {
            fluent::FluentArgs::from_iter([$((
                stringify!($name),
                fluent::FluentValue::from($name)
            )),+])
        })
        .unwrap_or(fallback)
    }};
    (@translate $context:expr, $source:literal, $($name:ident $(= $value:expr)?),+ $(,)?) => {
        $crate::i18n::tr!(@translate $context, $source, $($name $(= $value)?),+; format!($source))
    };
    (@value $name:ident = $value:expr) => {
        $value
    };
    (@value $name:ident) => {
        $name
    };
    ($($message:tt)+) => {
        $crate::i18n::tr!(@translate None, $($message)+)
    };
}
pub(crate) use tr;

#[cfg(test)]
#[path = "i18n_tests.rs"]
mod tests;
