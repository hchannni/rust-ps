// BOJ #1707. 이분 그래프

use std::io;

struct Graph {
    adj_list: Vec<Vec<usize>>,
    visited: Vec<bool>,
    color: Vec<SetColor>,
}

#[derive(Clone, PartialEq)]
enum SetColor {
    RED,
    BLUE,
    Nil,
}

impl Graph {
    fn new(num_vertices: usize) -> Self {
        let adj_list: Vec<Vec<usize>> = vec![Vec::new(); num_vertices + 1];
        let visited: Vec<bool> = vec![false; num_vertices + 1];
        let color: Vec<SetColor> = vec![SetColor::Nil; num_vertices + 1];

        Graph {
            adj_list,
            visited,
            color,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        // 무방향 그래프로 문제를 해결하겠다.
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
    }

    fn dfs(&mut self, start_node: usize) -> bool {
        let mut res: bool = true;

        self.color[start_node] = SetColor::RED;
        self.dfs_recursive(start_node, &mut res);

        res
    }

    fn dfs_recursive(&mut self, node: usize, res: &mut bool) {
        if *res == false {
            return;
        }

        if self.visited[node] {
            return;
        }

        self.visited[node] = true;

        let neighbors = &self.adj_list[node].clone();
        for &adj_node in neighbors {
            if self.color[adj_node] != SetColor::Nil {
                if self.color[adj_node] == self.color[node] {
                    *res = false;
                    return;
                } else {
                    continue;
                }
            } else {
                if self.color[node] == SetColor::RED {
                    self.color[adj_node] = SetColor::BLUE;
                } else if self.color[node] == SetColor::BLUE {
                    self.color[adj_node] = SetColor::RED;
                }

                self.dfs_recursive(adj_node, res);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let k: usize = input.trim().parse().unwrap(); // test case의 개수

    for _ in 0..k {
        input.clear();
        io::stdin().read_line(&mut input).expect("input error");
        let _input: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (v, e) = (_input[0], _input[1]); // 정점의 개수, 간선의 개수

        // 1. 그래프 인스턴스 생성
        let mut graph = Graph::new(v);

        // 2. Edge 정보 입력 받아와서 저장
        for _ in 0..e {
            input.clear();
            io::stdin().read_line(&mut input).expect("input error");
            let _input: Vec<usize> = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let (u, v) = (_input[0], _input[1]);
            graph.add_edge(u, v);
        }

        // 3. 모든 node에 대해 dfs 수행
        let mut res = true;
        for i in 1..=v {
            if !graph.visited[i] {
                if !graph.dfs(i) {
                    res = false;
                    break;
                }
            }
        }

        // 4. 결과 출력
        if res {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
