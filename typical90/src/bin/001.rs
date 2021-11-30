use proconio::input;
use std::vec;

// 最小値の最大化
fn main() {
    input! {
        n: usize, l: usize,
        k: usize,
        a: [usize; n]
    }
    let mut length = vec![0; n+1];
    length[0] = a[0];
    for i in 1..n {
        length[i] = a[i] - a[i - 1];
    }
    length[n] = l - a[n - 1];
    // println!("{:?}", length);

    // 二分探索（ある数 x が）
    let mut left = 0;
    let mut right = l;
    let mut ans = 0;
    while left <= right {
        let mut cnt = 0;
        let mid = (left + right) / 2;
        let mut num = 0;
        for i in 0..n+1 {
            if num < mid {
                num += length[i];
            } else {
                cnt += 1;
                num = length[i];
            }
        }
        if num >= mid {
            cnt += 1
        }
        if cnt > k {
            left = mid + 1;
            ans = mid;
        } else {
            right = mid - 1;
        }
    }
    println!("{}", ans);
}
