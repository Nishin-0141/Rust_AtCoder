use num::pow;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }
    let mod_num = pow(10, 9) + 7;
    // dp[i][j] :=
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 7]; n + 1];
    for i in 0..n + 1 {
        for j in 0..7 {
            dp[i + 1][j] = 1;
        }
    }
    println!("{}", dp[n][6] % mod_num);
}
