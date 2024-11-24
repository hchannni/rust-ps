// BOJ #2667. 단지번호붙이기

use std::{io, vec};

struct GraphMatrix {
    num_vertices: usize,
    adjacency_matrix: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    danji_info: Vec<usize>,
}

impl GraphMatrix {
    fn new(num_vertices: usize, adjacency_matrix: Vec<Vec<char>>) -> Self {
        let danji_info: Vec<usize> = Vec::new();
        let visited: Vec<Vec<bool>> = vec![vec![false; num_vertices]; num_vertices];

        GraphMatrix {
            num_vertices,
            adjacency_matrix,
            visited,
            danji_info,
        }
    }

    fn dfs(&mut self, row: usize, col: usize) {
        let mut house_count: usize = 0;
        if self.adjacency_matrix[row][col] == '1' && !self.visited[row][col] {
            // 이 부분에서 집이 존재하는 곳에서만 dfs를 수행하도록 조건을 걸어줌.
            self._dfs_recursive(row, col, &mut house_count);
            self.danji_info.push(house_count);
        }
    }

    fn _dfs_recursive(&mut self, row: usize, col: usize, house_count: &mut usize) {
        if self.visited[row][col] {
            return;
        }

        self.visited[row][col] = true;
        *house_count += 1; // 또 다른 재귀에 들어왔다는 것은 집 수가 하나 더 늘어났다는 의미!

        // 상하좌우 방향
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for &(dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if self._is_valid_position_isize(new_row, new_col) {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if self.adjacency_matrix[new_row][new_col] == '1' && !self.visited[new_row][new_col]
                {
                    self._dfs_recursive(new_row, new_col, house_count);
                }
            }
        }
    }

    // 유효한 좌표인지 확인
    fn _is_valid_position(&self, row: usize, col: usize) -> bool {
        row < self.num_vertices && col < self.num_vertices
    }

    // isize 타입을 사용하는 유효성 검사
    fn _is_valid_position_isize(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.num_vertices as isize && col >= 0 && col < self.num_vertices as isize
    }
}

fn main() {
    //////  input 받기 //////
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input 받기 실패");
    let n: usize = input.trim().parse().unwrap();
    ////////////////////////

    //////  input 받기 //////
    let mut graph_matrix: Vec<Vec<char>> = Vec::new();
    let mut input = String::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("input 받기 실패");

        // 입력받은 문자열의 줄바꿈 제거 후 char로 변환
        let trimmed = input.trim();
        graph_matrix.push(trimmed.chars().collect());
    }
    ////////////////////////

    let mut graph = GraphMatrix::new(n, graph_matrix);
    for i in 0..n {
        for j in 0..n {
            graph.dfs(i, j);
        }
    }

    graph.danji_info.sort();
    let danji_num = graph.danji_info.len();
    println!("{}", danji_num);
    for i in 0..danji_num {
        println!("{}", graph.danji_info[i]);
    }
}
