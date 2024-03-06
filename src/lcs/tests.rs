use crate::lcs::wunsch::diff;

struct TestCase {
    x: &'static str,
    y: &'static str,
    expected: &'static str,
}

struct TestCaseText {
    x: &'static [&'static str],
    y: &'static [&'static str],
    expected: &'static [&'static str],
}

const TEST_CASES: [TestCase; 4] = [
    TestCase {
        x: "ABCDEF",
        y: "ABCDEF",
        expected: "ABCDEF",
    },
    TestCase {
        x: "ABC",
        y: "XYZ",
        expected: "",
    },
    TestCase {
        x: "AABCXY",
        y: "XYZ",
        expected: "XY",
    },
    TestCase {
        x: "",
        y: "",
        expected: "",
    },
];

const TEST_CASES_TEXT: [TestCaseText; 1] = [
    TestCaseText {
        x: &[
            "Coding Challenges helps you become a better software engineer through that build real applications.",
            "I share a weekly coding challenge aimed at helping software engineers level up their skills through deliberate practice.",
            "I've used or am using these coding challenges as exercise to learn a new programming language or technology.",
            "Each challenge will have you writing a full application or tool. Most of which will be based on real world tools and utilities."
        ],
        y: &[
            "Helping you become a better software engineer through coding challenges that build real applications.",
            "I share a weekly coding challenge aimed at helping software engineers level up their skills through deliberate practice.",
            "These are challenges that I've used or am using as exercises to learn a new programming language or technology.",
            "Each challenge will have you writing a full application or tool. Most of which will be based on real world tools and utilities."
        ],
        expected: &[
            "I share a weekly coding challenge aimed at helping software engineers level up their skills through deliberate practice.",
            "Each challenge will have you writing a full application or tool. Most of which will be based on real world tools and utilities."
        ],
    }
];

fn _base_test<T>(seq_1: &Vec<T>, seq_2: &Vec<T>, expected: &Vec<T>) -> Result<(), String>
where
    T: std::fmt::Debug + Ord + Copy,
{
    let diff_res = diff::<T>(&seq_1, &seq_2);
    assert_eq!(diff_res, *expected);
    Ok(())
}

#[test]
fn test_chars() {
    for case in TEST_CASES {
        let _ = _base_test::<char>(
            &case.x.chars().collect(),
            &case.y.chars().collect(),
            &case.expected.chars().collect(),
        );
    }
}

#[test]
fn test_text() {
    for case in TEST_CASES_TEXT {
        let _ = _base_test::<&str>(
            &Vec::from(case.x),
            &Vec::from(case.y),
            &Vec::from(case.expected),
        );
    }
}
