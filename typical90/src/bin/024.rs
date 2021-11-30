use proconio::input;

fn main() {
    input! {
        n: usize, k: isize,
        a: [isize; n],
        b: [isize; n]
    }
    let mut num = 0;
    for i in 0..n {
        if a[i] > b[i] {
            num += a[i] - b[i];
        } else {
            num += b[i] - a[i];
        }
    }
    if (k - num) / 2 >= 0 && (k - num) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
   // println!("{}", num);
}
