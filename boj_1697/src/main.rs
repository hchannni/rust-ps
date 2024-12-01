// BOJ #1697. 숨바꼭질

use std::{collections::VecDeque, io};

struct Node {
    position: usize,
    time: usize,
}

impl Node {
    fn new(position: usize, time: usize) -> Self {
        Node { position, time }
    }
}

fn bfs(start: usize, target: usize) -> usize {
    let mut visited = vec![false; 100001];
    let mut queue: VecDeque<Node> = VecDeque::new();

    queue.push_back(Node::new(start, 0));
    visited[start] = true;


    // 조건 체크 잘 하자 !!! 
    // case 1. 같은 위치일 때는 이동할 필요 x
    if start == target {
        return 0;
    }

    // case 2. start가 target보다 뒤에 있을 때는 무조건 뒤로 1칸씩만 이동해야 함.
    if start > target {
        return start - target;
    }

    // 나머지 case. bfs 탐색 진행
    loop {
        // 목표 노드에 한 번이라도 도달할 때까지 루프 수행 (bfs 알고리즘 특성상 자동으로 최단시간 보장)
        if let Some(node) = queue.pop_front() {
            if node.position as i32 - 1 > 0 {
                if !visited[node.position - 1] {
                    if node.position - 1 == target {
                        return node.time + 1;
                    }

                    queue.push_back(Node::new(node.position - 1, node.time + 1));
                    visited[node.position - 1] = true;
                }
            }

            if node.position + 1 > 0 && node.position + 1 < 100001 {
                if !visited[node.position + 1] {
                    if node.position + 1 == target {
                        return node.time + 1;
                    }

                    queue.push_back(Node::new(node.position + 1, node.time + 1));
                    visited[node.position + 1] = true;
                }
            }

            if node.position * 2 > 0 && node.position * 2 < 100001 {
                if !visited[node.position * 2] {
                    if node.position * 2 == target {
                        return node.time + 1;
                    }

                    queue.push_back(Node::new(node.position * 2, node.time + 1));
                    visited[node.position * 2] = true;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let _input: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, k) = (_input[0], _input[1]);

    let res = bfs(n, k);
    println!("{}", res);
}
