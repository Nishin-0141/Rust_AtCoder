use proconio::input;

fn main() {
    input! {
        a: isize, b: isize
    };
    let mut ans = -100000;
    if ans < a + b {
        ans = a + b;
    }
    if ans < a - b {
        ans = a - b;
    }
    if ans < a * b { 
        ans = a * b;
    }
    println!("{}", ans);
}
