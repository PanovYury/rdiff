use std::cmp::max;
use super::{Chars, Matrix};

fn create_matrix(rows: usize, cols: usize) -> Matrix {
    vec![vec![0; cols]; rows]
}

pub fn get_chars(text: &str) -> Chars {
    text.chars().collect()
}

fn fill_dyn_matrix(x: &Chars, y: &Chars) -> Matrix {
    let mut matrix = create_matrix(x.len() + 1, y.len() + 1);
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

pub fn diff(x: &Chars, y: &Chars) -> Chars {
    let matrix = fill_dyn_matrix(&x, &y);
    let mut lcs: Chars = vec![];
    let mut i = x.len();
    let mut j = y.len();
    while i > 0 && j > 0 {
        if x[i - 1] == y[j - 1] {
            lcs.push(x[i - 1]);
            i -= 1;
            j -= 1;
        } else if matrix[i - 1][j] == matrix[i][j] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.reverse();
    return lcs;
}
