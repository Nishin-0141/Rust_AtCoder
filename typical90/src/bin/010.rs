use proconio::input;
use std::vec;

fn main() {
    input! {
        n: usize,
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; n]
    }
    let mut one_sum = vec![0];
    let mut two_sum = vec![0];
    for i in 0..n {
        if cp[i][0] == 1 {
            one_sum.push(cp[i][1]);
        } else {
            two_sum.push(cp[i][1]);
        }
    }
}
