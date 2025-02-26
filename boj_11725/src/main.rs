use std::collections::VecDeque;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // 트리 초기화
    let mut tree = vec![Vec::new(); n + 1]; // 1부터 n까지 사용하므로 n+1 크기로 초기화
    let mut parent = vec![0; n + 1]; // 부모 노드 정보를 저장할 배열
    let mut visited = vec![false; n + 1];

    // 간선 정보 입력
    for _ in 0..n - 1 {
        input.clear();
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        tree[edge[0]].push(edge[1]);
        tree[edge[1]].push(edge[0]);
    }

    // BFS를 사용하여 부모 노드 정보 저장
    let mut queue = VecDeque::new();
    visited[0] = true;
    visited[1] = true;
    queue.push_back(1); // 루트 노드 (1)부터 시작
    parent[1] = 0; // 루트 노드의 부모는 없음

    while let Some(node) = queue.pop_front() {
        for &neighbor in &tree[node] {
            if visited[neighbor] == false {
                parent[neighbor] = node; // 부모 노드 설정
                queue.push_back(neighbor); // 큐에 추가
                visited[neighbor] = true; // 방문 배열에 기록
            }
        }
    }

    // 결과 출력
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout);
    for i in 2..=n {
        writeln!(writer, "{}", parent[i]).unwrap();
    }
}
