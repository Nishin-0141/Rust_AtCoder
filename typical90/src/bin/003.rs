use proconio::input;
use std::vec;
// 木の直径を求めるプログラム
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
    let dist_first = bfs(graph, 0, n);
    let mut num = 0;
    let mut num_index = 0;
    for i in 0..n {
        if num < dist_first[i] {
            num = dist_first[i];
            num_index = i;
        }
    }
    let dist_second = bfs(graph, num_index, n);
    println!("{}", dist_second.iter().max().unwrap());
    /*
    // 1回目のBFS
    let mut q: Vec<usize> = Vec::new();
    let mut dist: Vec<usize>= vec![100000000; n];
    q.push(0);
    dist[0] = 0;
    while q.len() > 0 {
        let v = q.pop().unwrap();
        for i in 0..graph[v].len() {
            let to = graph[v][i];
            if dist[to] != 100000000 {
                continue;
            }
            dist[to] = dist[v] + 1;
            q.push(to);
        }
    }
    // println!("{:?}", dist);

    // 2回目のBFS
    let mut num: [usize; 2] = [0; 2];
    for i in 0..n {
        if num[0] < dist[i] {
            num[0] = dist[i];
            num[1] = i;
        }
    }
    let mut q: Vec<usize> = Vec::new();
    let mut dist= vec![100000000; n];
    q.push(num[1]);
    dist[num[1]] = 0;
    while q.len() > 0 {
        let v = q.pop().unwrap();
        for i in 0..graph[v].len() {
            let to = graph[v][i];
            if dist[to] != 100000000 {
                continue;
            }
            dist[to] = dist[v] + 1;
            q.push(to);
        }
    }
    println!("{}", dist.iter().max().unwrap() + 1)
    */
}
fn bfs(graph: Vec<Vec<usize>>, start: usize, n: usize) -> Vec<i32>{
    let mut q: Vec<usize> = Vec::new();
    let mut dist: Vec<i32> = vec![100000000; n];
    q.push(start);
    dist[start] = 0;
    while q.len() > 0 {
        let v = q.pop().unwrap();
        for i in 0..graph[v].len() {
            let to = graph[v][i];
            if dist[to] != 100000000 {
                continue;
            }
            dist[to] = dist[v] + 1;
            q.push(to);
        }
    }
    return dist;
}