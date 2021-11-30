use proconio::input;
use num::integer::gcd;

fn main() {
    input! {
        a: usize, b: usize, c: usize
    }
    let gcd_num = gcd(gcd(a, b), c);
    println!("{}", (a - gcd_num) / gcd_num + (b - gcd_num) / gcd_num + (c - gcd_num) / gcd_num);
}
