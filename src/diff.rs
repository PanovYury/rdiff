use std::{cmp::{max, Ord}, fmt::Display};

fn lcs_len<T: Ord>(x: &[T], y: &[T]) -> Vec<i32> {
    let mut matrix = vec![0; y.len() + 1];
    for i in 1..x.len() + 1 {
        for j in 1..y.len() + 1 {
            if x[i - 1] == y[j - 1] {
                matrix[j] = matrix[j] + 1
            } else {
                matrix[j] = max(matrix[j - 1], matrix[j])
            }
        }
    }
    matrix
}

fn lcs_len_reversed<T: Ord>(x: &[T], y: &[T]) -> Vec<i32> {
    let rev_x = x.iter().rev().collect::<Vec<&T>>();
    let rev_y = y.iter().rev().collect::<Vec<&T>>();
    let mut matrix = lcs_len(&rev_x, &rev_y);
    matrix.reverse();
    matrix
}

fn lcs_sum(lcs_1: &Vec<i32>, lcs_2: &Vec<i32>) -> Vec<i32> {
    lcs_1
        .iter()
        .zip(lcs_2.iter())
        .map(|(a, b)| a + b)
        .collect::<Vec<i32>>()
}

fn max_index<T: Ord>(array: &[T]) -> usize {
    let mut max = (0, &array[0]);
    for (idx, item) in array.iter().enumerate() {
        if *item > *max.1 {
            max = (idx, item);
        }
    }
    max.0
}

pub fn lcs<'a, T: Ord>(x: &'a [T], y: &[T]) -> Vec<&'a T> {
    match x.len() {
        0 => vec![],
        1 => {
            if y.contains(&x[0]) {
                vec![&x[0]]
            } else {
                vec![]
            }
        }
        _ => {
            // Split x vector
            let mid_x = x.len() / 2;
            let (xb, xe) = (&x[..mid_x], &x[mid_x..]);

            let matrix_l1 = lcs_len(xb, y);
            let matrix_l2 = lcs_len_reversed(&xe, &y);

            let sum = lcs_sum(&matrix_l1, &matrix_l2);

            // Split y vector
            let mid_y = max_index(&sum);
            let (yb, ye) = (&y[..mid_y], &y[mid_y..]);

            let lcs_b = lcs::<T>(xb, yb);
            let lcs_e = lcs::<T>(xe, ye);

            [lcs_b.as_slice(), lcs_e.as_slice()].concat()
        }
    }
}

pub fn diff<'a, T: Ord>(x: &'a Vec<T>, y: &Vec<T>) -> Vec<(bool, &'a T)> {
    let mut _lcs = lcs(&x, &y);
    x.iter()
        .map(|line| (_lcs.contains(&line), line))
        .collect::<Vec<(bool, &T)>>()
}

pub fn print_diff<T: Display>(lines: &Vec<(bool, &T)>) {
    for (is_include, line) in lines {
        let _diff_icon = if *is_include { ">" } else { "<" };
        println!("{} {}", _diff_icon, line);
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
