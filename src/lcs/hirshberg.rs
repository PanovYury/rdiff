use std::cmp::max;
use super::Chars;


fn lcs_length(seq_1: &Chars, seq_2: &Chars) -> Vec<i32> {
    let mut curr = vec![0; seq_2.len()];
    for x_item in seq_1 {
        let prev = curr.clone();
        for (y_i, y_item) in seq_2.iter().enumerate() {
            if x_item == y_item {
                curr[y_i + 1] = prev[y_i] + 1;
            } else {
                curr[y_i + 1] = max(curr[y_i], prev[y_i + 1])
            }
        }
    }
    return curr;
}

fn find_max(array: &Vec<i32>) -> (usize, i32) {
    let mut max = (0, array[0]);
    for (idx, item) in array.iter().enumerate() {
        if max.1 < *item {
            max = (idx, *item);
        }
    }
    return max;
}

pub fn lcs_hirshberg(seq_1: &Chars, seq_2: &Chars) -> Chars {
    let x_len = seq_1.len();
    if x_len == 0 {
        return vec![];
    } else if x_len == 1 {
        return if seq_2.contains(&seq_1[0]) {
            vec![seq_1[0]]
        } else {
            vec![]
        };
    }
    let i = x_len / 2;
    let xb = Vec::from(&seq_1[..i]);
    let xe = Vec::from(&seq_1[i..]);
    let l1 = lcs_length(&xb, seq_2);
    let l2: Vec<i32> = lcs_length(
        &xe.iter().rev().cloned().collect(),
        &seq_2.iter().rev().cloned().collect(),
    )
    .iter()
    .rev()
    .cloned()
    .collect();
    let sum: Vec<i32> = l1.iter().zip(l2).map(|(x_i, y_i)| x_i + y_i).collect();
    let (j, _) = find_max(&sum);
    let yb: Chars = Vec::from(&seq_2[..j]);
    let ye: Chars = Vec::from(&seq_2[j..]);

    let mut res_x = lcs_hirshberg(&xb, &xe);
    let mut res_y = lcs_hirshberg(&yb, &ye);
    res_x.append(&mut res_y);
    return res_x;
}