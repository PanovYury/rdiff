use std::cmp::max;

type Chars = Vec<char>;
type Matrix = Vec<Vec<i32>>;

fn create_matrix(rows: usize, cols: usize) -> Matrix {
    vec![vec![0; cols]; rows]
}

pub fn get_chars(text: &str) -> Chars {
    text.chars().collect()
}

fn fill_dyn_matrix(seq_1: &Chars, seq_2: &Chars) -> Matrix {
    let mut matrix = create_matrix(seq_1.len() + 1, seq_2.len() + 1);
    for (i, x) in seq_1.iter().enumerate() {
        for (j, y) in seq_2.iter().enumerate() {
            if x == y {
                matrix[i + 1][j + 1] = matrix[i][j] + 1
            } else {
                matrix[i + 1][j + 1] = max(matrix[i + 1][j], matrix[i][j + 1])
            }
        }
    }
    return matrix;
}

pub fn diff(seq_1: Chars, seq_2: Chars) -> Chars {
    let matrix = fill_dyn_matrix(&seq_1, &seq_2);
    let mut lcs: Chars = vec![];
    let mut i = seq_1.len() as i32 - 1;
    let mut j = seq_2.len() as i32 - 1;
    while i >= 0 && j >= 0 {
        let ui = i as usize;
        let uj = j as usize;
        if seq_1[ui] == seq_2[uj] {
            lcs.push(seq_1[ui]);
            i -= 1;
            j -= 1;
        } else if matrix[ui][uj + 1] > matrix[ui + 1][uj] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.reverse();
    return lcs;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_test(seq_1: &str, seq_2: &str, expected: &str) -> Result<(), String> {
        let diff = diff(get_chars(seq_1), get_chars(seq_2));
        assert_eq!(diff, get_chars(expected));
        Ok(())
    }

    #[test]
    fn check_equals() -> Result<(), String> {
        base_test("ABCDEF", "ABCDEF", "ABCDEF")
    }

    #[test]
    fn check_not_equals() -> Result<(), String> {
        base_test("ABCD", "XYZW", "")
    }

    #[test]
    fn check_partical() -> Result<(), String> {
        base_test("AABCXY", "XYZ", "XY")
    }

    #[test]
    fn check_empty() -> Result<(), String> {
        base_test("", "", "")
    }
}
