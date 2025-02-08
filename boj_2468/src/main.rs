use std::io::{self};

fn dfs(graph: &Vec<Vec<usize>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, safety: usize) {
    if visited[x][y] == true {
        return;
    }
    visited[x][y] = true;

    // 그래프 형식에서 x가 row, y가 column
    for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;

        if next_x >= 0
            && next_x < graph.len() as isize
            && next_y >= 0
            && next_y < graph[0].len() as isize
        {
            if graph[next_x as usize][next_y as usize] >= safety
                && !visited[next_x as usize][next_y as usize]
            {
                dfs(graph, next_x as usize, next_y as usize, visited, safety);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut graph = vec![Vec::<usize>::new(); n];
    for i in 0..n {
        io::stdin().read_line(&mut input).expect("input error");
        graph[i] = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        input.clear();
    }

    let mut visited = vec![vec![false; n]; n];
    let mut max_safety_areas: usize = 0;
    let mut cur_safety_areas: usize = 0;
    for safety in 1..=100 {
        for i in 0..n {
            for j in 0..n {
                if graph[i][j] >= safety && !visited[i][j] {
                    dfs(&graph, i, j, &mut visited, safety);
                    cur_safety_areas += 1;
                }
            }
        }
        if cur_safety_areas > max_safety_areas {
            max_safety_areas = cur_safety_areas;
        }
        visited = vec![vec![false; n]; n];
        cur_safety_areas = 0;
    }

    println!("{}", max_safety_areas);
}
