use std::cmp::{max, Ord};

pub type Matrix = Vec<Vec<i32>>;

fn create_matrix(rows: usize, cols: usize) -> Matrix {
    vec![vec![0; cols]; rows]
}

fn fill_dyn_matrix<T: Ord>(x: &Vec<T>, y: &Vec<T>) -> Matrix {
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

pub fn diff<T: Ord + Copy>(x: &Vec<T>, y: &Vec<T>) -> Vec<T> {
    let matrix = fill_dyn_matrix(&x, &y);
    let mut lcs: Vec<T> = vec![];
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
