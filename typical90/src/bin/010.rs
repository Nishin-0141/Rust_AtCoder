use proconio::input;
use std::vec;

fn main() {
    input! {
        n: usize,
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; q]
    }
    let mut one_sum = vec![0; n+1];
    let mut two_sum = vec![0; n+1];
    for i in 0..n {
        if cp[i][0] == 1 {
            one_sum[i + 1] = one_sum[i] + cp[i][1];
            two_sum[i + 1] = two_sum[i];
        } else {
            one_sum[i + 1] = one_sum[i];
            two_sum[i + 1] = two_sum[i] + cp[i][1];
        }
    }
    // println!("{:?}", one_sum);
    // println!("{:?}", two_sum);
    for i in 0..q {
        let ans1 = one_sum[lr[i][1]] - one_sum[lr[i][0] - 1];
        let ans2 = two_sum[lr[i][1]] - two_sum[lr[i][0] - 1];
        println!("{} {}", ans1, ans2);
    }
}
