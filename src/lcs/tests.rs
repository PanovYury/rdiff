use crate::lcs::wunsch::{diff, get_chars, lcs_hirshberg};

fn _base_test(seq_1: &str, seq_2: &str, expected: &str) -> Result<(), String> {
    let diff = diff(&get_chars(seq_1), &get_chars(seq_2));
    assert_eq!(diff, get_chars(expected));
    Ok(())
}

#[test]
fn check_equals() -> Result<(), String> {
    _base_test("ABCDEF", "ABCDEF", "ABCDEF")
}

#[test]
fn check_not_equals() -> Result<(), String> {
    _base_test("ABC", "XYZ", "")
}

#[test]
fn check_partical() -> Result<(), String> {
    _base_test("AABCXY", "XYZ", "XY")
}

#[test]
fn check_empty() -> Result<(), String> {
    _base_test("", "", "")
}
