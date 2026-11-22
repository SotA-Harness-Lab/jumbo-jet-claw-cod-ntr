// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

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
