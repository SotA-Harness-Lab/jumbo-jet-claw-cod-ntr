// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The TUI's translations, with Mozilla's Fluent through i18n-embed.
//!
//! Messages live in `i18n/<language>/codex-tui.ftl`; `en-US` is the source and the fallback,
//! so a message missing from a translation shows in English. [`tr!`] looks a message up by id
//! and is checked at compile time against `en-US` (the id exists, the arguments match).
//!
//! The language is the one `[ui] language` of `ccn-config.toml` names, else the system's
//! (`LANG`, the desktop's preferred languages); [`init`] selects it once at startup. Until then,
//! and always in tests, messages are English.

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;

use fluent::FluentValue;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::LanguageLoader;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::fluent::fluent_language_loader;
use i18n_embed::unic_langid::LanguageIdentifier;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

/// The loader [`tr!`] reads from: English until [`init`] selects another language.
pub(crate) static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    if let Err(err) = loader.load_fallback_language(&Localizations) {
        tracing::error!("failed to load the TUI's English messages: {err}");
    }
    // Unicode bidi isolation marks around arguments would show up in a terminal.
    loader.set_use_isolating(false);
    loader
});

/// Selects the TUI's language: `configured` (`[ui] language` of `ccn-config.toml`) when given,
/// else the system's. A language without a translation, or a tag that does not parse, leaves
/// the TUI in English. Tests never call it.
pub(crate) fn init(configured: Option<&str>) {
    let requested: Vec<LanguageIdentifier> = match configured {
        Some(tag) => match tag.parse() {
            Ok(language) => vec![language],
            Err(err) => {
                tracing::warn!("ignoring [ui] language = {tag:?}: {err}");
                DesktopLanguageRequester::requested_languages()
            }
        },
        None => DesktopLanguageRequester::requested_languages(),
    };
    if let Err(err) = i18n_embed::select(&*LOADER, &Localizations, &requested) {
        tracing::warn!("failed to select the TUI's language: {err}");
    }
    // Loading a language rebuilds the bundles, which isolate arguments again by default.
    LOADER.set_use_isolating(false);
}

/// The message `id` in the selected language as a `&'static str`, for the many places that
/// hand out static text. Each message is formatted once; [`init`] runs before any is shown.
pub(crate) fn static_message(id: &'static str, message: impl FnOnce() -> String) -> &'static str {
    static CACHE: LazyLock<Mutex<HashMap<&'static str, &'static str>>> =
        LazyLock::new(Mutex::default);
    let mut cache = CACHE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    cache
        .entry(id)
        .or_insert_with(|| Box::leak(message().into_boxed_str()))
}

/// A message argument as a Fluent value, its type fixed before `fl!` converts it: numbers stay
/// numbers (plural selection needs them), text becomes a string.
pub(crate) fn arg<'a>(value: impl Into<FluentValue<'a>>) -> FluentValue<'a> {
    value.into()
}

/// A translated message: `tr!("id")` is a `&'static str`; `tr!("id", name = value, ...)` a
/// `String` with the arguments filled in.
macro_rules! tr {
    ($id:literal) => {
        $crate::i18n::static_message($id, || {
            i18n_embed_fl::fl!($crate::i18n::LOADER, $id)
        })
    };
    ($id:literal, $($name:ident = $value:expr),+ $(,)?) => {
        i18n_embed_fl::fl!(
            $crate::i18n::LOADER,
            $id,
            $($name = $crate::i18n::arg($value)),+
        )
    };
}
pub(crate) use tr;

#[cfg(test)]
#[path = "i18n_tests.rs"]
mod tests;
