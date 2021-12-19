use num::pow;
use proconio::input;

fn n_to_r(num: usize, n: usize, r: usize) -> String {
    let first: usize = n_to_ten(num.to_string(), n);
    let second: String = ten_to_n(first, r);
    return second;
}

fn ten_to_n(num: usize, n: usize) -> String {
    if num / n > 0 {
        return ten_to_n(num / n, n).to_owned() + (num % n).to_string().as_str();
    }
    return (num % n).to_string();
}

fn n_to_ten(num: String, n: usize) -> usize {
    let mut num_return = 0;
    let num_len = num.len();
    let num_vec: Vec<char> = num.chars().collect();
    for i in 0..num_len {
        num_return += pow(n, num_len - i - 1) * (num_vec[i] as usize - 48);
    }
    return num_return;
}

fn change_eight_to_five(num: String) -> String {
    let mut num_return: String = "".to_string();
    let num_vec: Vec<char> = num.chars().collect();
    for i in num_vec {
        if i == '8' {
            num_return += &'5'.to_string();
        } else {
            num_return += &i.to_string();
        }
    }
    return num_return;
}

fn main() {
    input! {
        n: usize, k: usize
    }
    let mut n_string: String = n.to_string();

    for _ in 0..k {
        n_string = n_to_r(n_string.parse().unwrap(), 8, 9);
        n_string = change_eight_to_five(n_string);
    }
    println!("{}", n_string);
}
