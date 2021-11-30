use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut ans: Vec<String> = Vec::new();
    for bit in 0..(1 << n) {
        let mut letter: String = "".to_string();
        let mut cnt = 0;
        let mut check = true;
        for i in 0..n {
            if (1 << i) & bit == 0 {
                letter += "(";
                cnt += 1
            } else {
                letter += ")";
                cnt -= 1
            }
            if cnt < 0 {
                check = false;
                break;
            }
        }
        if check && cnt == 0{
            ans.push(letter);
        }
    }
    ans.sort();
    for i in ans {
        println!("{}", i);
    }
    // println!("{:?}", ans);
}
