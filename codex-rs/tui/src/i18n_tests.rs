// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use i18n_embed::LanguageLoader;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::fluent::fluent_language_loader;
use pretty_assertions::assert_eq;

use super::Localizations;

/// Each message's id and the variables it uses, read from an embedded .ftl file.
fn messages(language: &str) -> BTreeMap<String, BTreeSet<String>> {
    let file = Localizations::get(&format!("{language}/codex-tui.ftl")).expect("embedded .ftl");
    let text = String::from_utf8(file.data.into_owned()).expect("UTF-8 .ftl");
    let mut messages = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        // A message starts at the line's beginning; a multiline one ends its first line with " =".
        if let Some((id, _)) = line.split_once(" =")
            && !line.starts_with([' ', '#'])
        {
            current = Some(id.to_string());
            messages.insert(id.to_string(), BTreeSet::new());
        }
        if let Some(id) = &current {
            let variables = messages.get_mut(id).expect("current message");
            for (index, _) in line.match_indices("{ $") {
                let rest = &line[index + 3..];
                let end = rest.find([' ', '}']).unwrap_or(rest.len());
                variables.insert(rest[..end].to_string());
            }
        }
    }
    messages
}

#[test]
fn every_translation_has_an_english_message_with_its_variables() {
    let english = messages("en-US");
    for (id, variables) in messages("zh-CN") {
        // A translation may leave out a variable (the count of an English plural), never add one.
        let english_variables = english.get(&id);
        assert!(
            english_variables.is_some_and(|english| variables.is_subset(english)),
            "zh-CN message {id} uses {variables:?}, English has {english_variables:?}"
        );
    }
}

#[test]
fn chinese_is_selected_and_tests_otherwise_read_english() {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    loader
        .load_languages(&Localizations, &["zh-CN".parse().expect("tag")])
        .expect("load zh-CN");
    assert_eq!(loader.get("slash-config"), "配置 Codex 偏好设置");
    assert_eq!(super::tr!("slash-config"), "configure Codex preferences");
}
