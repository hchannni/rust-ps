// BOJ #10026. 적록색약
// 같은 색상이 상하좌우로 인접해 있는 경우에 두 글자는 같은 구역에 속한다.
// 그림이 입력으로 주어졌을 때 (N x N 배열)
// 적록색약인 사람이 봤을 때와 아닌 사람이 봤을 때의 구역의 수를 구하는 프로그램을 작성하시오.
// R / G / B의 텍스트를 구분, 적록색약인 사람은 R과 G를 동일하게 인식하도록 로직 작성

use std::io::{self};

fn dfs(graph: &Vec<Vec<String>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) {
    if visited[x][y] {
        return;
    }
    visited[x][y] = true;

    // 상하좌우 인접한 노드에 대해서 탐색 수행
    for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let next_x = x as isize + dr;
        let next_y = y as isize + dc;
        if next_x >= 0
            && next_x < graph.len() as isize
            && next_y >= 0
            && next_y < graph.len() as isize
        {
            if graph[x][y] == graph[next_x as usize][next_y as usize]
                && !visited[next_x as usize][next_y as usize]
            {
                // 인접 좌표가 현재 좌표와 색이 같고, 방문하지 않았을 경우에만 dfs 수행
                dfs(graph, next_x as usize, next_y as usize, visited);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut graph: Vec<Vec<String>> = Vec::new();
    for _ in 0..n {
        io::stdin().read_line(&mut input).expect("input error");
        let row: Vec<String> = input.trim().chars().map(|c| c.to_string()).collect();
        graph.push(row);
        input.clear();
    }

    // 먼저 적록색약이 아닌 케이스의 구역 수 카운트
    let mut visited = vec![vec![false; n + 1]; n + 1];
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                dfs(&graph, i, j, &mut visited);
                count += 1; // dfs 한 사이클이 끝났다는 것은 -> 같은 색상의 한 구역이 완료되었다는 뜻이다.
            }
        }
    }

    // 적록색약을 카운트하기 위해 R을 모두 G로 바꿔준다.
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == 'R'.to_string() {
                graph[i][j] = 'G'.to_string();
            }
        }
    }

    // 적록색약의 케이스에서 구역 수 카운트
    visited = vec![vec![false; n + 1]; n + 1];
    let mut count_saekyak = 0;
    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                dfs(&graph, i, j, &mut visited);
                count_saekyak += 1; // dfs 한 사이클이 끝났다는 것은 -> 같은 색상의 한 구역이 완료되었다는 뜻이다.
            }
        }
    }

    println!("{} {}", count, count_saekyak);
}
