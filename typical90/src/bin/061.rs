use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [[usize; 2]; q]
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in tx {
        if i[0] == 1 {
            que.push_front(i[1]);
        } else if i[0] == 2 {
            que.push_back(i[1]);
        } else {
            println!("{}", que[i[1] - 1]);
        }
    }
}
