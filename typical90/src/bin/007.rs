use num::pow;
use num::abs;
use std::cmp::min;
use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q]
    }
    a.push(pow(10, 10));
    a.push(-pow(10, 10));
    a.sort();
    // println!("");
    // println!("{}", a.len());
    for item in b {
        let mut left = 0;
        let mut right = n + 1;
        while left <= right {
            let mid = (left + right) / 2;
            if item < a[mid] {
                right = mid - 1;
            } else if a[mid] < item {
                left = mid + 1;
            } else {
                right = mid;
                break;
            }
        }
        println!("{}", min(abs(item - a[right]), abs(a[right + 1] - item)));
    }
}
