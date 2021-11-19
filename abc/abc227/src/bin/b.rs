use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n]
    };
    let mut check;
    let mut ans = 0;
    for i in s {
        check = false;
        // println!("{}", i);
        for a in 1..334 {
            for b in 1..334 {
                if 4 * a * b + 3 * a + 3 * b == i {
                    check = true;
                    break;
                }
            }
        }
        if !check {
            ans += 1;
        }
    }
    println!("{}", ans)
}
