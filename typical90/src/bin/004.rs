use std::vec;
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h]
    }
    let mut sum_row = vec![0; h];
    let mut sum_column = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            sum_row[i] += a[i][j];
            sum_column[j] += a[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            let ans = sum_row[i] + sum_column[j] - a[i][j];
            print!("{} ", ans);
        }
        println!("");
    }
}
