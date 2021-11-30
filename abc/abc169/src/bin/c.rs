use proconio::input;

fn main() {
    input! {
        a: u128, b: String
    };
    let re_b: u128 = b.replace(".", "").parse().unwrap();
    println!("{}", a * re_b / 100);
}
