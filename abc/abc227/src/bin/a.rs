use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize
    };
    // println!("{}", n);
    let mut ans = 0;
    for i in a..a+k {
        if i % n == 0 {
            ans = n;
        } else {
            ans = i % n;
        }
        // println!("{}", i);
    }
    println!("{}", ans);
}
