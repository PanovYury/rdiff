use std::{cmp::max, fmt};

#[derive(Debug, PartialEq)]
pub enum DiffOp<'a, T> {
    Unchanged(&'a T),
    Removed(&'a T),
    Added(&'a T),
}

impl<'a, T: fmt::Debug> fmt::Display for DiffOp<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffOp::Unchanged(line) => write!(f, "  {:?}", line),
            DiffOp::Removed(line) => write!(f, "- {:?}", line),
            DiffOp::Added(line) => write!(f, "+ {:?}", line),
        }
    }
}

pub type Matrix = Vec<Vec<i32>>;

fn fill_matrix<T: PartialEq>(x: &[T], y: &[T]) -> Matrix {
    let mut matrix = vec![vec![0; y.len() + 1]; x.len() + 1];
    for i in 1..=x.len() {
        for j in 1..=y.len() {
            if x[i - 1] == y[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1] + 1
            } else {
                matrix[i][j] = max(matrix[i - 1][j], matrix[i][j - 1])
            }
        }
    }
    matrix
}

pub fn diff<'a, T: PartialEq>(x: &'a [T], y: &'a [T]) -> Vec<DiffOp<'a, T>> {
    let matrix = fill_matrix(x, y);
    let mut result = Vec::new();
    let mut i = x.len();
    let mut j = y.len();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
            result.push(DiffOp::Unchanged(&x[i - 1]));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || matrix[i][j - 1] >= matrix[i - 1][j]) {
            // Элемент есть в Y, но нет в X (добавлен)
            result.push(DiffOp::Added(&y[j - 1]));
            j -= 1;
        } else if i > 0 && (j == 0 || matrix[i][j - 1] < matrix[i - 1][j]) {
            // Элемент есть в X, но нет в Y (удален)
            result.push(DiffOp::Removed(&x[i - 1]));
            i -= 1;
        }
    }

    result.reverse();
    result
}

pub fn print_diff<T: std::fmt::Debug>(diff_ops: &[DiffOp<T>]) {
    for op in diff_ops {
        println!("{op}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a, T> {
        x: &'a [T],
        y: &'a [T],
        diff: &'a [DiffOp<'a, T>],
    }

    const TEST_CASES_CHARS: &[TestCase<char>] = &[
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['A', 'B', 'C'],
            diff: &[
                DiffOp::Unchanged(&'A'),
                DiffOp::Unchanged(&'B'),
                DiffOp::Unchanged(&'C'),
            ],
        },
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['X', 'Y', 'Z'],
            diff: &[
                DiffOp::Removed(&'A'),
                DiffOp::Removed(&'B'),
                DiffOp::Removed(&'C'),
                DiffOp::Added(&'X'),
                DiffOp::Added(&'Y'),
                DiffOp::Added(&'Z'),
            ],
        },
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['A', 'Y', 'C'],
            diff: &[
                DiffOp::Unchanged(&'A'),
                DiffOp::Removed(&'B'),
                DiffOp::Added(&'Y'),
                DiffOp::Unchanged(&'C'),
            ],
        },
        TestCase {
            x: &['A', 'B', 'C'],
            y: &['B', 'A', 'C'],
            diff: &[
                DiffOp::Removed(&'A'),
                DiffOp::Unchanged(&'B'),
                DiffOp::Added(&'A'),
                DiffOp::Unchanged(&'C'),
            ],
        },
    ];

    const TEST_CASES_NUMBER: &[TestCase<i32>] = &[
        TestCase {
            x: &[1, 2, 3],
            y: &[1, 2, 3],
            diff: &[
                DiffOp::Unchanged(&1),
                DiffOp::Unchanged(&2),
                DiffOp::Unchanged(&3),
            ],
        },
        TestCase {
            x: &[1, 2, 3],
            y: &[4, 5, 6],
            diff: &[
                DiffOp::Removed(&1),
                DiffOp::Removed(&2),
                DiffOp::Removed(&3),
                DiffOp::Added(&4),
                DiffOp::Added(&5),
                DiffOp::Added(&6),
            ],
        },
        TestCase {
            x: &[1, 2, 3],
            y: &[1, 5, 3],
            diff: &[
                DiffOp::Unchanged(&1),
                DiffOp::Removed(&2),
                DiffOp::Added(&5),
                DiffOp::Unchanged(&3),
            ],
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
            diff: &[
                DiffOp::Unchanged(&"Line 1 for compare"),
                DiffOp::Removed(&"Line 2 for compare with change"),
                DiffOp::Added(&"Line 2 for compare"),
                DiffOp::Unchanged(&"Line 3 for compare"),
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
            diff: &[
                DiffOp::Unchanged(&"Line 1 for compare"),
                DiffOp::Removed(&"Line 2 for compare with change"),
                DiffOp::Removed(&"Line 3 for comparing"),
                DiffOp::Added(&"Line 2 for compare"),
                DiffOp::Added(&"Line 3 for compare"),
            ],
        },
    ];

    fn _base_test_types<T: Clone + Ord + std::fmt::Debug>(test_cases: &[TestCase<T>]) {
        for case in test_cases {
            let x = case.x.to_vec();
            let y = case.y.to_vec();
            assert_eq!(&diff(&x, &y), &case.diff);
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
