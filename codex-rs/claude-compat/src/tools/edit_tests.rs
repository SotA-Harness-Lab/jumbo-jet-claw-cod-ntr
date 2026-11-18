use pretty_assertions::assert_eq;

use super::EditError;
use super::apply_edit;
use super::success_message;

#[test]
fn replaces_a_unique_string() {
    assert_eq!(
        apply_edit(Some("let a = 1;\nlet b = 2;\n"), "a = 1", "a = 3", false),
        Ok("let a = 3;\nlet b = 2;\n".to_string())
    );
}

#[test]
fn refuses_a_string_found_more_than_once_unless_replace_all() {
    let content = "x\nx\n";
    assert_eq!(
        apply_edit(Some(content), "x", "y", false),
        Err(EditError::NotUnique {
            matches: 2,
            old_string: "x".to_string()
        })
    );
    assert_eq!(
        apply_edit(Some(content), "x", "y", true),
        Ok("y\ny\n".to_string())
    );
}

#[test]
fn refuses_missing_and_identical_strings() {
    assert_eq!(
        apply_edit(Some("abc"), "zzz", "y", false),
        Err(EditError::NotFound {
            old_string: "zzz".to_string()
        })
    );
    assert_eq!(
        apply_edit(Some("abc"), "a", "a", false),
        Err(EditError::NoChange)
    );
    assert_eq!(
        EditError::NotFound {
            old_string: "zzz".to_string()
        }
        .message(),
        "String to replace not found in file.\nString: zzz"
    );
}

#[test]
fn empty_old_string_creates_or_fills_a_file() {
    assert_eq!(apply_edit(None, "", "new", false), Ok("new".to_string()));
    assert_eq!(
        apply_edit(Some("  \n"), "", "new", false),
        Ok("new".to_string())
    );
    assert_eq!(
        apply_edit(Some("content"), "", "new", false),
        Err(EditError::FileExists)
    );
    assert_eq!(
        apply_edit(None, "a", "b", false),
        Err(EditError::FileMissing)
    );
}

#[test]
fn deleting_a_line_takes_its_line_break() {
    assert_eq!(
        apply_edit(Some("keep\ndrop\nkeep\n"), "drop", "", false),
        Ok("keep\nkeep\n".to_string())
    );
}

#[test]
fn replacement_text_is_literal() {
    assert_eq!(
        apply_edit(Some("a"), "a", "$& $1", false),
        Ok("$& $1".to_string())
    );
}

#[test]
fn success_messages() {
    assert_eq!(
        success_message("/p/f.rs", false),
        "The file /p/f.rs has been updated successfully."
    );
    assert_eq!(
        success_message("/p/f.rs", true),
        "The file /p/f.rs has been updated. All occurrences were successfully replaced."
    );
}
