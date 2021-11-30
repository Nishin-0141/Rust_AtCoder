use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut ans: usize = 0;
    for i in 1..n+1 {
        ans += i * 10000
    }
    println!("{}", ans / n);
}
