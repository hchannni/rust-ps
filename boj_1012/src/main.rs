use std::io::{self};

fn dfs(graph: &Vec<Vec<usize>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) {
    if visited[x][y] {
        return;
    }
    visited[x][y] = true;

    for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;

        if next_x >= 0
            && next_x < graph.len() as isize
            && next_y >= 0
            && next_y < graph[0].len() as isize
        {
            if graph[next_x as usize][next_y as usize] == 1
                && !visited[next_x as usize][next_y as usize]
            {
                dfs(graph, next_x as usize, next_y as usize, visited);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let t: usize = input.trim().parse().unwrap(); // 테스트 케이스의 개수
    input.clear();

    for _ in 0..t {
        // 1. 배추밭 정보 받기
        io::stdin().read_line(&mut input).expect("input error");
        let mut _graph_info: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        input.clear();

        let m: usize = _graph_info[0]; // 배추밭의 가로길이
        let n: usize = _graph_info[1]; // 배추밭의 세로길이
        let k: usize = _graph_info[2]; // 배추가 심어져 있는 위치의 개수

        let mut graph: Vec<Vec<usize>> = vec![vec![0; m]; n]; // n x m 크기로 초기화
        for _ in 0..k {
            // 2. 배추 심은 곳 위치 등록
            io::stdin().read_line(&mut input).expect("input error");
            let mut _cabbage_info: Vec<usize> = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            input.clear();

            graph[_cabbage_info[1]][_cabbage_info[0]] = 1; // 배추 심은 곳 표시
        }

        // 3. 배추 심어져 있는 공간에 대해 탐색 수행
        let mut visited = vec![vec![false; m]; n];
        let mut count = 0;
        for i in 0..n {
            for j in 0..m {
                if graph[i][j] == 1 && !visited[i][j] {
                    dfs(&graph, i, j, &mut visited);
                    count += 1;
                }
            }
        }

        println!("{}", count);
    }
}
