use crate::lcs::wunsch::{diff, lcs};

struct TestCase {
    x: &'static str,
    y: &'static str,
    lcs: &'static str,
    diff: &'static [&'static str],
}

struct TestCaseText {
    x: &'static [&'static str],
    y: &'static [&'static str],
    lcs: &'static [&'static str],
    diff: &'static [&'static str],
}

const TEST_CASES: [TestCase; 4] = [
    TestCase {
        x: "ABCDEF",
        y: "ABCDEF",
        lcs: "ABCDEF",
        diff: &["< 'A'", "< 'B'", "< 'C'", "< 'D'", "< 'E'", "< 'F'"],
    },
    TestCase {
        x: "ABC",
        y: "XYZ",
        lcs: "",
        diff: &["> 'A'", "> 'B'", "> 'C'",],
    },
    TestCase {
        x: "AABCXY",
        y: "XYZ",
        lcs: "XY",
        diff: &["> 'A'", "> 'A'", "> 'B'", "> 'C'", "< 'X'", "< 'Y'"],
    },
    TestCase {
        x: "",
        y: "",
        lcs: "",
        diff: &[],
    },
];

const TEST_CASES_TEXT: &[TestCaseText] = &[
    TestCaseText {
        x: &[
            "Line 1 for compare",
            "Line 2 for compare with change",
            "Line 3 for compare",
        ],
        y: &[
            "Line 1 for compare",
            "Line 2 for compare",
            "Line 3 for compare",
        ],
        lcs: &["Line 1 for compare", "Line 3 for compare"],
        diff: &[
            "< \"Line 1 for compare\"",
            "> \"Line 2 for compare with change\"",
            "< \"Line 3 for compare\"",
        ],
    },
    TestCaseText {
        x: &[
            "Line 1 for compare",
            "Line 2 for compare with change",
            "Line 3 for comparing",
        ],
        y: &[
            "Line 1 for compare",
            "Line 2 for compare",
            "Line 3 for compare",
        ],
        lcs: &["Line 1 for compare"],
        diff: &[
            "< \"Line 1 for compare\"",
            "> \"Line 2 for compare with change\"",
            "> \"Line 3 for comparing\"",
        ],
    },
];

fn _base_test_lcs<T>(seq_1: &Vec<T>, seq_2: &Vec<T>, expected: &Vec<T>) -> Result<(), String>
where
    T: std::fmt::Debug + Ord + Copy,
{
    let diff_res = lcs::<T>(&seq_1, &seq_2);
    assert_eq!(diff_res, *expected);
    Ok(())
}

fn _base_test_diff<T>(seq_1: &Vec<T>, seq_2: &Vec<T>, expected: &Vec<&str>) -> Result<(), String>
where
    T: std::fmt::Debug + Ord + Copy,
{
    let diff_res = diff::<T>(&seq_1, &seq_2);
    assert_eq!(diff_res, *expected);
    Ok(())
}

#[test]
fn test_chars_lcs() {
    for case in TEST_CASES {
        let _ = _base_test_lcs::<char>(
            &case.x.chars().collect(),
            &case.y.chars().collect(),
            &case.lcs.chars().collect(),
        );
    }
}

#[test]
fn test_chars_diff() {
    for case in TEST_CASES {
        let _ = _base_test_diff::<char>(
            &case.x.chars().collect(),
            &case.y.chars().collect(),
            &Vec::from(case.diff),
        );
    }
}

#[test]
fn test_text_lcs() {
    for case in TEST_CASES_TEXT {
        let _ =
            _base_test_lcs::<&str>(&Vec::from(case.x), &Vec::from(case.y), &Vec::from(case.lcs));
    }
}

#[test]
fn test_text_diff() {
    for case in TEST_CASES_TEXT {
        let _ = _base_test_diff::<&str>(
            &Vec::from(case.x),
            &Vec::from(case.y),
            &Vec::from(case.diff),
        );
    }
}
