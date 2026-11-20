// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::net::IpAddr;

use pretty_assertions::assert_eq;

use super::interpolate_env_vars;
use super::is_blocked_address;
use super::url_matches_pattern;
use super::validate_body;

fn blocked(address: &str) -> bool {
    is_blocked_address(address.parse::<IpAddr>().expect("address"))
}

#[test]
fn url_patterns_match_whole_urls_with_wildcards() {
    for (url, pattern, expected) in [
        ("https://hooks.example.com/a", "https://hooks.example.com/a", true),
        ("https://hooks.example.com/a/b", "https://hooks.example.com/a", false),
        ("https://hooks.example.com/a/b", "https://hooks.example.com/*", true),
        ("https://api.example.com/v1/hook", "https://*.example.com/*/hook", true),
        ("https://api.example.org/v1/hook", "https://*.example.com/*/hook", false),
        ("https://example.com/", "*", true),
        ("http://example.com/x", "https://*", false),
    ] {
        assert_eq!(url_matches_pattern(url, pattern), expected, "{url} {pattern}");
    }
}

#[test]
fn only_released_variables_reach_a_header() {
    let lookup = |name: &str| match name {
        "TOKEN" => Some("abc".to_string()),
        "SECRET" => Some("hidden".to_string()),
        "BROKEN" => Some("a\r\nX-Evil: 1".to_string()),
        _ => None,
    };
    assert_eq!(
        interpolate_env_vars("Bearer $TOKEN", &["TOKEN"], lookup),
        "Bearer abc"
    );
    assert_eq!(
        interpolate_env_vars("Bearer ${TOKEN}/$SECRET", &["TOKEN"], lookup),
        "Bearer abc/"
    );
    assert_eq!(interpolate_env_vars("$MISSING", &["MISSING"], lookup), "");
    assert_eq!(
        interpolate_env_vars("$BROKEN", &["BROKEN"], lookup),
        "aX-Evil: 1"
    );
    assert_eq!(interpolate_env_vars("plain", &[], lookup), "plain");
}

#[test]
fn private_addresses_are_refused_and_loopback_is_not() {
    for address in [
        "10.0.0.1",
        "172.16.5.4",
        "192.168.1.1",
        "169.254.169.254",
        "100.64.0.1",
        "0.0.0.0",
        "fd00::1",
        "fe80::1",
        "::ffff:10.0.0.1",
    ] {
        assert_eq!(blocked(address), true, "{address}");
    }
    for address in ["127.0.0.1", "::1", "8.8.8.8", "2606:4700::1111", "100.128.0.1"] {
        assert_eq!(blocked(address), false, "{address}");
    }
}

#[test]
fn a_response_is_a_json_object_or_empty() {
    assert_eq!(validate_body(""), Ok(()));
    assert_eq!(validate_body("  \n"), Ok(()));
    assert_eq!(validate_body(r#"{"decision":"block","reason":"no"}"#), Ok(()));
    assert!(validate_body("ok").is_err());
    assert!(validate_body("{not json").is_err());
}
