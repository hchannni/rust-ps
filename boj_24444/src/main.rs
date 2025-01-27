use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

fn bfs(
    graph: &Vec<Vec<usize>>, // 인접 리스트
    visited: &mut Vec<bool>, // 방문 여부 기록
    order: &mut Vec<usize>,  // 방문 순서를 기록
    start: usize,            // 시작 노드
) {
    let mut queue = VecDeque::new(); // BFS를 위한 큐
    let mut counter = 0; // 방문 순서 카운터

    queue.push_back(start); // 시작 노드를 큐에 추가
    visited[start] = true; // 시작 노드를 방문 처리

    while let Some(node) = queue.pop_front() {
        counter += 1;
        order[node] = counter; // 방문 순서를 기록

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor); // 방문하지 않은 이웃 노드를 큐에 추가
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout);

    // 입력 받기
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();

    let n: usize = parts.next().unwrap().parse().unwrap(); // 노드 수
    let m: usize = parts.next().unwrap().parse().unwrap(); // 간선 수
    let r: usize = parts.next().unwrap().parse().unwrap(); // 시작 노드

    // 그래프 초기화
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    for _ in 0..m {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut edge_parts = input.trim().split_whitespace();
        let u: usize = edge_parts.next().unwrap().parse().unwrap();
        let v: usize = edge_parts.next().unwrap().parse().unwrap();

        graph[u].push(v);
        graph[v].push(u);
    }

    // 각 노드의 인접 리스트를 오름차순으로 정렬
    for neighbors in graph.iter_mut() {
        neighbors.sort_unstable();
    }

    // BFS 수행
    let mut visited = vec![false; n + 1];
    let mut order = vec![0; n + 1];

    bfs(&graph, &mut visited, &mut order, r);

    // 결과 출력
    for i in 1..=n {
        writeln!(writer, "{}", order[i]).unwrap();
    }
}
