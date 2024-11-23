// BOJ #13023. ABCDE

use std::io;

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(num_vertices: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); num_vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list[from].push(to);

        // 방향이 없는 그래프일 경우, to -> from 도 추가
        self.adjacency_list[to].push(from);
    }

    fn dfs(&mut self, start: usize) -> bool {
        let mut visited = vec![false; self.adjacency_list.len()];
        let res = self._dfs_recursive(start, &mut visited, 1);
        res
    }

    fn _dfs_recursive(&mut self, node: usize, visited: &mut Vec<bool>, depth: usize) -> bool {
        if visited[node] {
            return false;
        }

        visited[node] = true;
        if depth == 5 {
            return true;
        }

        for &neighbor in &self.adjacency_list[node].clone() {
            if self._dfs_recursive(neighbor, visited, depth + 1) {
                return true;
            }
        }

        visited[node] = false;
        false
    }
}

fn main() {
    //////  input 받기 //////
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input 받기 실패");
    let _input: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (_input[0], _input[1]);
    ////////////////////////

    let mut graph = Graph::new(n);
    for _ in 0..m {
        //////  input 받기 //////
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input 받기 실패");
        let _input: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (a, b) = (_input[0], _input[1]);
        ////////////////////////

        graph.add_edge(a, b);
    }

    for i in 0..n {
        let res = graph.dfs(i);

        if res {
            println!("1");
            return;
        } else {
            continue;
        }
    }

    println!("0");
}
