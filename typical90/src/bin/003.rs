use proconio::input;
use std::vec;
use std::collections::VecDeque;


fn tree_diameter(n: usize, graph: Vec<Vec<usize>>) -> usize {
    let num_max = 100000000;
    
    // First BFS
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![num_max; n];
    q.push_back(0);
    dist[0] = 0;
    while q.len() > 0 {
        let v = q.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let to = graph[v][i];
            if dist[to] != num_max {
                continue;
            }
            dist[to] = dist[v] + 1;
            q.push_back(to);
        }
    }
    
    // Search longest path
    let mut num: [usize; 2] = [0; 2];
    for i in 0..n {
        if num[0] < dist[i] {
            num[0] = dist[i];
            num[1] = i;
        }
    }
    
    // Second BFS
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![num_max; n];
    q.push_back(num[1]);
    dist[num[1]] = 0;
    while q.len() > 0 {
        let v = q.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let to = graph[v][i];
            if dist[to] != num_max {
                continue;
            }
            dist[to] = dist[v] + 1;
            q.push_back(to);
        }
    }
    return *dist.iter().max().unwrap();
}


fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n-1]
    }

    // graph の初期化と双方向の座標を挿入
    let mut graph: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        graph.push([].to_vec());
    }
    for i in ab {
        graph[i[0] - 1].push(i[1] - 1);
        graph[i[1] - 1].push(i[0] - 1);
    }
    let ans = tree_diameter(n, graph) + 1;
    println!("{}", ans);
}