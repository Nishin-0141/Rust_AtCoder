use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut ans = 0;
    for a in 1..10000 {
        for b in 1..1000000 {
            if a > b {
                break;
            } else {
                ans += n / a / b - b + 1
            }
        }
    }
    println!("{}", &ans);
}
