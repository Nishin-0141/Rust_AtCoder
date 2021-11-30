use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    };
    let mut check = false;
    let atcoder = "atcoder";
    for i in 0..s.len() {
        if &s[i..i+1] != &t[i..i+1] {
            println!("{}", check);
        }
    }
}
