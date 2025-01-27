use std::io::{self, BufWriter, Write};

fn dfs(
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    order: &mut Vec<usize>,
    node: usize,
    counter: &mut usize,
) {
    visited[node] = true;
    *counter += 1;
    order[node] = *counter;

    for &neighbor in &graph[node] {
        if !visited[neighbor] {
            dfs(graph, visited, order, neighbor, counter);
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

    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let r: usize = parts.next().unwrap().parse().unwrap();

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

    // 각 노드의 인접 리스트를 내림차순으로 정렬
    for neighbors in graph.iter_mut() {
        neighbors.sort_unstable_by(|a, b| b.cmp(a));
    }

    // DFS 수행
    let mut visited = vec![false; n + 1];
    let mut order = vec![0; n + 1];
    let mut counter = 0;

    dfs(&graph, &mut visited, &mut order, r, &mut counter);

    // 결과 출력
    for i in 1..=n {
        writeln!(writer, "{}", order[i]).unwrap();
    }
}
