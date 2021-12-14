use proconio::input;

fn main() {
    input! {
        h: usize, w: usize
    }
    if h == 1 || w == 1 {
        println!("{}", h * w);
    } else {
        let num1 = (h + 1) / 2;
        let num2 = (w + 1) / 2;
        println!("{}", num1 * num2);
    }
}
