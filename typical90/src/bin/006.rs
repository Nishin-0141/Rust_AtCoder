use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        s: String
    }
    let mut c: Vec<Vec<usize>> = Vec::new();
    let mut word = &s[0..1];
    let mut word_index = 0;
    for i in 0..n-k+1 {
        if word > &s[i..i+1] {
            word = &s[i..i+1];
            word_index = i;
        }
    }
    let mut ans = word;
    println!("{} {}", ans, word_index);
    for i in word_index+1..n {

    }
}
