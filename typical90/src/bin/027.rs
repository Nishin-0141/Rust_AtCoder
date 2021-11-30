use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut s_set: HashSet<String> = HashSet::new();
    for i in 0..n {
        let cnt = &s[i];
        if s_set.insert(cnt.to_string()) {
            println!("{}", i + 1);
        }
    }
}
