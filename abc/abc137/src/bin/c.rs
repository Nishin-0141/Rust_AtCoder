use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut map = HashMap::new();
    for i in 0..n {
        for j in 0..10 {
            map.insert(&s[i][j..j+1], i);
        }
    }
    for (i, j) in map {
        println!("i = {:?},j = {:?}", i, j);
    }
}
