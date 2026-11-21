// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::TextSlice;
use super::binary_extension;
use super::description;
use super::exceeds_token_limit;
use super::is_image;
use super::is_notebook;
use super::is_pdf;
use super::parse_page_range;
use super::pdfinfo_page_count;
use super::pdftoppm_failed_message;
use super::render;
use super::rendered_page_number;
use super::slice_text;
use super::too_large_message;
use super::validate_pages;

#[test]
fn whole_file_is_numbered_from_one() {
    let slice = slice_text("\u{feff}fn main() {\r\n}\n", None, None);
    assert_eq!(
        slice,
        TextSlice {
            content: "fn main() {\n}".to_string(),
            start_line: 1,
            num_lines: 2,
            total_lines: 2,
        }
    );
    assert_eq!(render(&slice), "1\tfn main() {\n2\t}");
}

#[test]
fn a_final_line_break_starts_no_line() {
    assert_eq!(slice_text("a\nb", None, None).total_lines, 2);
    assert_eq!(
        render(&slice_text("a\nb\n\n", None, None)),
        "1\ta\n2\tb\n3\t"
    );
    assert_eq!(render(&slice_text("\n", None, None)), "1\t");
    assert_eq!(
        render(&slice_text("a\nb\n", Some(3), None)),
        "<system-reminder>Warning: the file exists but is shorter than the provided offset (3). The file has 2 lines.</system-reminder>"
    );
}

#[test]
fn offset_and_limit_select_lines() {
    let text = "a\nb\nc\nd\ne";
    let slice = slice_text(text, Some(2), Some(2));
    assert_eq!(slice.content, "b\nc");
    assert_eq!(slice.total_lines, 5);
    assert_eq!(render(&slice), "2\tb\n3\tc");
    assert_eq!(slice_text(text, Some(0), Some(1)).content, "a");
}

#[test]
fn empty_file_and_short_file_get_warnings() {
    assert_eq!(
        render(&slice_text("", None, None)),
        "<system-reminder>Warning: the file exists but the contents are empty.</system-reminder>"
    );
    assert_eq!(
        render(&slice_text("a\nb", Some(10), None)),
        "<system-reminder>Warning: the file exists but is shorter than the provided offset (10). The file has 2 lines.</system-reminder>"
    );
}

#[test]
fn file_kinds_follow_the_extension() {
    assert!(is_image("/tmp/Shot.PNG"));
    assert!(!is_image("/tmp/png"));
    assert_eq!(binary_extension("/a/b.tar.gz"), Some(".gz".to_string()));
    assert_eq!(binary_extension("/a/doc.pdf"), None);
    assert_eq!(binary_extension("/a/.bashrc"), None);
    assert_eq!(binary_extension("/a/main.rs"), None);
    for name in ["t.parquet", "m.XLSM", "b.epub", "p.heic", "w.npz", "d.pkl"] {
        assert!(binary_extension(name).is_some(), "{name}");
    }
}

#[test]
fn limits_and_messages() {
    assert!(!exceeds_token_limit(&"x".repeat(100_000)));
    assert!(exceeds_token_limit(&"x".repeat(100_001)));
    assert!(
        too_large_message(300 * 1024)
            .starts_with("File content (300KB) exceeds maximum allowed size (256KB).")
    );
    assert!(description(true).contains("read images"));
    assert!(!description(false).contains("read images"));
}

#[test]
fn pdfs_and_notebooks_are_recognised_by_extension() {
    assert!(is_pdf("/a/Paper.PDF"));
    assert!(!is_pdf("/a/.pdf"));
    assert!(is_notebook("/a/analysis.ipynb"));
    assert!(!is_notebook("/a/analysis.py"));
}

#[test]
fn page_ranges_are_parsed_as_claude_code_parses_them() {
    assert_eq!(parse_page_range("5"), Some((5, Some(5))));
    assert_eq!(parse_page_range(" 1-10 "), Some((1, Some(10))));
    assert_eq!(parse_page_range("3-"), Some((3, None)));
    assert_eq!(parse_page_range("0"), None);
    assert_eq!(parse_page_range("5-2"), None);
    assert_eq!(parse_page_range("-5"), None);
    assert_eq!(parse_page_range("x"), None);
    assert_eq!(parse_page_range(""), None);
    assert_eq!(parse_page_range(" 3 - 5 "), Some((3, Some(5))));
}

#[test]
fn a_list_of_pages_is_refused_not_cut_to_its_first_entry() {
    assert_eq!(parse_page_range("6,9,15"), None);
    assert_eq!(parse_page_range("3-5,8"), None);
    assert_eq!(parse_page_range("2abc"), None);
    assert_eq!(
        validate_pages("6,9,15"),
        Err("Invalid pages parameter: \"6,9,15\". Use formats like \"1-5\", \"3\", or \"10-20\". Pages are 1-indexed. To read several pages or ranges, read each one in a call of its own.".to_string())
    );
}

#[test]
fn a_read_takes_at_most_twenty_pages() {
    assert_eq!(validate_pages("1-20"), Ok((1, Some(20))));
    assert_eq!(
        validate_pages("1-21"),
        Err("Page range \"1-21\" exceeds maximum of 20 pages per request. Please use a smaller range.".to_string())
    );
    assert!(validate_pages("3-").is_err());
    assert!(
        validate_pages("a")
            .unwrap_err()
            .starts_with("Invalid pages parameter: \"a\".")
    );
}

#[test]
fn poppler_output_is_read() {
    assert_eq!(
        pdfinfo_page_count("Title: x\nPages:          12\nEncrypted: no\n"),
        Some(12)
    );
    assert_eq!(pdfinfo_page_count("Title: x\n"), None);
    assert_eq!(rendered_page_number("page-07.jpg"), Some(7));
    assert_eq!(rendered_page_number("page-120.jpg"), Some(120));
    assert_eq!(rendered_page_number("page-1.png"), None);
    assert_eq!(
        pdftoppm_failed_message("Command Line Error: Incorrect password"),
        "PDF is password-protected. Please provide an unprotected version."
    );
    assert_eq!(
        pdftoppm_failed_message("Syntax Error: Couldn't find trailer dictionary; damaged"),
        "PDF file is corrupted or invalid."
    );
}
