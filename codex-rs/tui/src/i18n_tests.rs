// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use fluent::FluentArgs;

use super::Catalog;
use super::CatalogSource;
use super::ZH_CN;
use super::select_catalog;

#[test]
fn translated_patterns_use_arguments_from_the_english_key() {
    let source: CatalogSource = serde_json::from_str(ZH_CN).expect("valid catalog");
    for (english, translation) in source.messages.iter().chain(
        source
            .contexts
            .values()
            .flatten()
            .filter_map(|(english, translation)| Some((english, translation.as_ref()?))),
    ) {
        for reference in translation.split("{ $").skip(1) {
            let (name, _) = reference
                .split_once([' ', '}'])
                .expect("end of named argument");
            assert!(
                english.contains(&format!("{{{name}}}")),
                "{translation:?} uses {name:?} absent from {english:?}"
            );
        }
    }
}

#[test]
fn english_is_the_fallback_for_unavailable_translations() {
    for tag in ["en-US", "fr-FR"] {
        let requested = [tag.parse().expect("valid language")];
        assert!(select_catalog(&requested).expect("select locale").is_none());
    }
    assert_eq!(tr!("Press enter to continue"), "Press enter to continue");
    assert_eq!(tr!("Welcome, {name}!", name = "Ada"), "Welcome, Ada!");
}

#[test]
fn english_plural_uses_the_count_when_other_arguments_precede_it_in_the_text() {
    for (count, expected) in [
        (0, "Ada has 0 tasks"),
        (1, "Ada has 1 task"),
        (2, "Ada has 2 tasks"),
    ] {
        assert_eq!(
            tr!(
                "{name} has 1 task" | "{name} has {count} tasks",
                count,
                name = "Ada"
            ),
            expected
        );
    }
}

#[test]
fn captured_and_explicit_arguments_can_be_mixed() {
    let name = "Ada";
    let mut evaluations = 0;
    let message = tr!(
        "Welcome, {name}. You have {count} tasks.",
        name,
        count = {
            evaluations += 1;
            2
        }
    );
    assert_eq!(message, "Welcome, Ada. You have 2 tasks.");
    assert_eq!(evaluations, 1);
}

#[test]
fn chinese_catalog_looks_up_english_and_interpolates_arguments() {
    let catalog = select_catalog(&["zh".parse().expect("valid language")])
        .expect("valid catalog")
        .expect("Chinese");
    assert_eq!(
        catalog
            .lookup(/*context*/ None, "configure Codex preferences")
            .and_then(|message| message.plain.as_deref()),
        Some("配置 Codex 偏好设置")
    );
    assert!(catalog.lookup(/*context*/ None, "Added upstream").is_none());

    let message = catalog
        .lookup(/*context*/ None, "{current} of {total}")
        .expect("progress message");
    let args = FluentArgs::from_iter([("current", 2), ("total", 3)]);
    assert_eq!(
        Catalog::format(&catalog.bundle, message, Some(&args)).as_deref(),
        Some("2 / 3")
    );
    assert!(Catalog::format(&catalog.bundle, message, /*args*/ None).is_none());
}

#[test]
fn shared_labels_only_match_their_own_context() {
    let catalog = select_catalog(&["zh-CN".parse().expect("valid language")])
        .expect("valid catalog")
        .expect("Chinese");
    assert_eq!(
        catalog
            .contextual("status-field", "Model")
            .and_then(|message| message.plain.as_deref()),
        Some("模型")
    );
    assert!(catalog.contextual("selection-title", "Model").is_none());
    assert_eq!(
        catalog
            .lookup(Some("actions"), "List")
            .and_then(|message| message.plain.as_deref()),
        Some("列表")
    );
}

/// A string literal of a Rust source file: the bytes it spans and its value.
struct Literal {
    start: usize,
    end: usize,
    value: String,
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The string literals of a Rust source file, in order. Comments, character literals and
/// lifetimes are skipped.
fn string_literals(source: &str) -> Vec<Literal> {
    let bytes = source.as_bytes();
    let mut literals = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &bytes[i..];
        if rest.starts_with(b"//") {
            i += rest
                .iter()
                .position(|byte| *byte == b'\n')
                .unwrap_or(rest.len());
        } else if rest.starts_with(b"/*") {
            let mut depth = 1;
            i += 2;
            while depth > 0 && i < bytes.len() {
                if bytes[i..].starts_with(b"/*") {
                    depth += 1;
                    i += 2;
                } else if bytes[i..].starts_with(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
        } else if let Some(hashes) = raw_string_hashes(bytes, i) {
            let start = i;
            let body = i + 1 + hashes + 1;
            let mut close = vec![b'"'];
            close.resize(1 + hashes, b'#');
            let length = bytes[body..]
                .windows(close.len())
                .position(|window| window == close)
                .expect("end of raw string");
            i = body + length + close.len();
            literals.push(Literal {
                start,
                end: i,
                value: source[body..body + length].to_string(),
            });
        } else if bytes[i] == b'"' {
            let start = i;
            let mut value = Vec::new();
            i += 1;
            while bytes[i] != b'"' {
                if bytes[i] != b'\\' {
                    value.push(bytes[i]);
                    i += 1;
                    continue;
                }
                match bytes[i + 1] {
                    b'\n' => {
                        i += 2;
                        while bytes[i].is_ascii_whitespace() {
                            i += 1;
                        }
                    }
                    b'u' => {
                        let close = i + source[i..].find('}').expect("end of unicode escape");
                        let code = u32::from_str_radix(&source[i + 3..close], 16)
                            .expect("unicode escape");
                        let character = char::from_u32(code).expect("unicode scalar");
                        value.extend_from_slice(character.encode_utf8(&mut [0; 4]).as_bytes());
                        i = close + 1;
                    }
                    b'x' => {
                        value.push(
                            u8::from_str_radix(&source[i + 2..i + 4], 16).expect("byte escape"),
                        );
                        i += 4;
                    }
                    escaped => {
                        value.push(match escaped {
                            b'n' => b'\n',
                            b't' => b'\t',
                            b'r' => b'\r',
                            b'0' => 0,
                            other => other,
                        });
                        i += 2;
                    }
                }
            }
            i += 1;
            literals.push(Literal {
                start,
                end: i,
                value: String::from_utf8_lossy(&value).into_owned(),
            });
        } else if bytes[i] == b'\'' {
            // A character literal, or else a lifetime, which has no closing quote.
            i += match bytes.get(i + 1) {
                Some(b'\\') => {
                    3 + bytes[i + 3..]
                        .iter()
                        .position(|byte| *byte == b'\'')
                        .expect("end of character literal")
                        + 1
                }
                Some(first) => {
                    let width = match *first {
                        0xf0.. => 4,
                        0xe0.. => 3,
                        0xc0.. => 2,
                        _ => 1,
                    };
                    if bytes.get(i + 1 + width) == Some(&b'\'') {
                        width + 2
                    } else {
                        1
                    }
                }
                None => 1,
            };
        } else {
            i += 1;
        }
    }
    literals
}

/// The number of `#` of a raw string literal starting at `i`, if one starts there.
fn raw_string_hashes(bytes: &[u8], i: usize) -> Option<usize> {
    if bytes[i] != b'r' {
        return None;
    }
    let after_identifier = match i {
        0 => false,
        1 => is_identifier_byte(bytes[0]),
        _ => {
            is_identifier_byte(bytes[i - 1])
                && !(bytes[i - 1] == b'b' && !is_identifier_byte(bytes[i - 2]))
        }
    };
    let hashes = bytes[i + 1..]
        .iter()
        .take_while(|byte| **byte == b'#')
        .count();
    (!after_identifier && bytes.get(i + 1 + hashes) == Some(&b'"')).then_some(hashes)
}

/// The context and catalog key of every `tr!` call in `source` that passes its text as a literal.
fn translated_texts<'a>(source: &str, literals: &'a [Literal]) -> Vec<(Option<&'a str>, &'a str)> {
    let bytes = source.as_bytes();
    let skip_blank = |mut i: usize| {
        while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
            i += 1;
        }
        i
    };
    let literal_at = |i: usize| {
        literals
            .binary_search_by_key(&i, |literal| literal.start)
            .ok()
            .map(|index| &literals[index])
    };
    let mut texts = Vec::new();
    for (call, _) in source.match_indices("tr!(") {
        // Calls inside comments and strings are not calls.
        if (call > 0 && is_identifier_byte(bytes[call - 1]))
            || literals
                .iter()
                .any(|literal| literal.start < call && call < literal.end)
        {
            continue;
        }
        let mut i = skip_blank(call + "tr!(".len());
        let mut context = None;
        if bytes[i..].starts_with(b"in") && bytes.get(i + 2).is_some_and(u8::is_ascii_whitespace) {
            let Some(literal) = literal_at(skip_blank(i + 2)) else {
                continue;
            };
            context = Some(literal.value.as_str());
            i = skip_blank(literal.end);
            if bytes.get(i) != Some(&b',') {
                continue;
            }
            i = skip_blank(i + 1);
        }
        let Some(mut text) = literal_at(i) else {
            continue;
        };
        // The plural form `"one" | "other"` is looked up by its second text.
        let after = skip_blank(text.end);
        if bytes.get(after) == Some(&b'|')
            && let Some(other) = literal_at(skip_blank(after + 1))
        {
            text = other;
        }
        texts.push((context, text.value.as_str()));
    }
    texts
}

/// The Rust files under `directory` that are not tests, with their contents.
fn production_sources(directory: &std::path::Path, sources: &mut Vec<(String, String)>) {
    let mut entries = std::fs::read_dir(directory)
        .expect("read source directory")
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if path.is_dir() {
            if name != "tests" {
                production_sources(&path, sources);
            }
        } else if name.ends_with(".rs") && name != "tests.rs" && !name.ends_with("_tests.rs") {
            let text = std::fs::read_to_string(&path).expect("read source file");
            sources.push((path.display().to_string(), text));
        }
    }
}

#[test]
fn string_literals_are_read_as_rust_reads_them() {
    let source = r##"// "comment" tr!("no")
        let a = "one \"two\" \u{4e2d}\n"; /* "block" */
        let b = r#"raw "three""#; let c = '"'; let d: &'static str = "four \
            five";
        tr!(in "menu", "Open"); tr!("{count} file" | "{count} files", count);
        tr!($label); "tr!(\"quoted\")";
    "##;
    let literals = string_literals(source);
    let values = literals
        .iter()
        .map(|literal| literal.value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        values,
        [
            "one \"two\" 中\n",
            "raw \"three\"",
            "four five",
            "menu",
            "Open",
            "{count} file",
            "{count} files",
            "tr!(\"quoted\")",
        ]
    );
    assert_eq!(
        translated_texts(source, &literals),
        [(Some("menu"), "Open"), (None, "{count} files")]
    );
}

/// The catalog is keyed by the English text, so rewording a text in the code silently drops its
/// translation and leaves the old entry behind. This fails on both until the catalog follows.
#[test]
fn chinese_catalog_matches_the_english_texts_in_the_code() {
    let catalog: CatalogSource = serde_json::from_str(ZH_CN).expect("valid catalog");
    let mut sources = Vec::new();
    production_sources(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut sources,
    );
    let mut in_code = std::collections::HashSet::new();
    let mut untranslated = Vec::new();
    for (path, source) in &sources {
        let literals = string_literals(source);
        for (context, text) in translated_texts(source, &literals) {
            let in_context = context
                .and_then(|context| catalog.contexts.get(context))
                .is_some_and(|messages| messages.contains_key(text));
            if !in_context && !catalog.messages.contains_key(text) {
                untranslated.push(format!("{path}: {text:?}"));
            }
        }
        in_code.extend(literals.into_iter().map(|literal| literal.value));
    }
    let mut stale = catalog
        .messages
        .keys()
        .chain(catalog.contexts.values().flat_map(HashMap::keys))
        .filter(|english| !in_code.contains(*english))
        .collect::<Vec<_>>();
    stale.sort();
    stale.dedup();
    assert!(
        untranslated.is_empty() && stale.is_empty(),
        "i18n/zh-CN.json does not match the code.\n\
         Texts passed to tr! with no entry (add one, keyed by the English text):\n{untranslated:#?}\n\
         Entries whose English text is no longer in the code (rename or remove them):\n{stale:#?}"
    );
}
