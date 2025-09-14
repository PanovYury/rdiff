use std::{cmp::{max, Ord}, collections::LinkedList};

pub type Matrix = Vec<Vec<i32>>;

fn fill_matrix<T: Ord>(x: &Vec<T>, y: &Vec<T>) -> Matrix {
    let mut matrix = vec![vec![0; y.len() + 1]; x.len() + 1];
    for i in 1..x.len() + 1 {
        for j in 1..y.len() + 1 {
            if x[i - 1] == y[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1] + 1
            } else {
                matrix[i][j] = max(matrix[i - 1][j], matrix[i][j - 1])
            }
        }
    }
    return matrix;
}

pub fn lcs<'a, T: Ord>(x: &'a Vec<T>, y: &Vec<T>) -> Vec<&'a T> {
    let matrix = fill_matrix(&x, &y);
    let mut lcs = LinkedList::<&T>::new();
    let mut i = x.len();
    let mut j = y.len();
    while i > 0 && j > 0 {
        if x[i - 1] == y[j - 1] {
            lcs.push_front(&x[i - 1]);
            i -= 1;
            j -= 1;
        } else if matrix[i - 1][j] > matrix[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    return lcs.into_iter().collect();
}

pub fn diff<'a, T: Ord>(x: &'a Vec<T>, y: &Vec<T>) -> Vec<(bool, &'a T)> {
    let mut _lcs = lcs(&x, &y);
    x.iter()
        .map(|line| (_lcs.contains(&line), line))
        .collect::<Vec<(bool, &T)>>()
}

pub fn print_diff<T: std::fmt::Debug>(lines: &Vec<(bool, &T)>) {
    for (is_include, line) in lines {
        let _diff_icon = if *is_include { ">" } else { "<" };
        println!("{} {:?}", _diff_icon, line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a, T> {
        x: &'a [T],
        y: &'a [T],
        lcs: &'a [&'a T],
        diff: &'a [(bool, &'a T)],
    }

    const TEST_CASES_CHARS: &[TestCase<char>] = &[
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['A', 'B', 'C'],
            lcs: &[&'A', &'B', &'C'],
            diff: &[(true, &'A'), (true, &'B'), (true, &'C')],
        },
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['X', 'Y', 'Z'],
            lcs: &[],
            diff: &[(false, &'A'), (false, &'B'), (false, &'C')],
        },
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['A', 'Y', 'C'],
            lcs: &[&'A', &'C'],
            diff: &[(true, &'A'), (false, &'B'), (true, &'C')],
        },
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['B', 'A', 'C'],
            lcs: &[&'B', &'C'],
            diff: &[(false, &'A'), (true, &'B'), (true, &'C')],
        },
    ];

    const TEST_CASES_NUMBER: &[TestCase<i32>] = &[
        TestCase {
            x: &[1, 2, 3],
            y: &[1, 2, 3],
            lcs: &[&1, &2, &3],
            diff: &[(true, &1), (true, &2), (true, &3)],
        },
        TestCase {
            x: &[1, 2, 3],
            y: &[4, 5, 6],
            lcs: &[],
            diff: &[(false, &1), (false, &2), (false, &3)],
        },
        TestCase {
            x: &[1, 2, 3],
            y: &[1, 5, 3],
            lcs: &[&1, &3],
            diff: &[(true, &1), (false, &2), (true, &3)],
        },
    ];

    const TEST_CASES_TEXT: &[TestCase<&str>] = &[
        TestCase {
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
            lcs: &[&"Line 1 for compare", &"Line 3 for compare"],
            diff: &[
                (true, &"Line 1 for compare"),
                (false, &"Line 2 for compare with change"),
                (true, &"Line 3 for compare"),
            ],
        },
        TestCase {
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
            lcs: &[&"Line 1 for compare"],
            diff: &[
                (true, &"Line 1 for compare"),
                (false, &"Line 2 for compare with change"),
                (false, &"Line 3 for comparing"),
            ],
        },
    ];

    fn _base_test_types<T: Clone + Ord + std::fmt::Debug>(test_cases: &[TestCase<T>]) {
        for case in test_cases {
            let x = case.x.to_vec();
            let y = case.y.to_vec();
            assert_eq!(&lcs(&x, &y), &case.lcs.to_vec());
            assert_eq!(&diff(&x, &y), &case.diff.to_vec());
        }
    }

    #[test]
    fn test_text() {
        _base_test_types(TEST_CASES_TEXT);
    }

    #[test]
    fn test_chars() {
        _base_test_types(TEST_CASES_CHARS);
    }

    #[test]
    fn test_numbers() {
        _base_test_types(TEST_CASES_NUMBER);
    }
}
