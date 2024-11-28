// BOJ #1325. 효율적인 해킹

use std::{collections::VecDeque, io};

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    trust_count: Vec<usize>,
}

impl Graph {
    fn new(count_node: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); count_node + 1],
            trust_count: vec![0; count_node + 1],
        }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.adjacency_list[a].push(b);
    }

    fn bfs(&mut self, start: usize) {
        let mut visited = vec![false; self.adjacency_list.len() + 1];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;

        loop {
            if let Some(cur_node) = queue.pop_front() {
                for i in 0..self.adjacency_list[cur_node].len() {
                    let n = self.adjacency_list[cur_node][i];

                    if !visited[n] {
                        self.trust_count[n] += 1;
                        visited[n] = true;
                        queue.push_back(n);
                    }
                }
            } else {
                break;
            }
        }
    }
}

fn main() {
    // A가 B를 신뢰할 경우, B를 해킹하면 A도 해킹할 수 있다.

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input 받기 실패");
    let _input: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (_input[0], _input[1]);

    let mut bfs_graph = Graph::new(n);
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("input fail");
        let _input: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (a, b) = (_input[0], _input[1]);

        bfs_graph.add_edge(a, b);
    }

    for i in 1..n + 1 {
        bfs_graph.bfs(i);
    }

    let mut max: usize = 0;
    for i in 1..n + 1 {
        if bfs_graph.trust_count[i] > max {
            max = bfs_graph.trust_count[i];
        }
    }

    for i in 1..n + 1 {
        if bfs_graph.trust_count[i] == max {
            print!("{} ", i);
        }
    }
    println!();
}
