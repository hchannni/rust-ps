use std::io;

fn main() {
    let mut input = String::new();
    let mut results = Vec::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("input error");
        let dimensions: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let (w, h) = (dimensions[0], dimensions[1]);
        if w == 0 && h == 0 {
            break; // 입력이 0 0이면 종료
        }

        let mut map = vec![vec![0; w]; h]; // 2차원 배열 초기화

        for i in 0..h {
            input.clear();
            io::stdin().read_line(&mut input).expect("input error");
            map[i] = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
        }

        let mut visited = vec![vec![false; w]; h]; // 방문 여부 배열
        let mut island_count = 0;

        for i in 0..h {
            for j in 0..w {
                if map[i][j] == 1 && !visited[i][j] {
                    dfs(&map, &mut visited, i, j, w, h);
                    island_count += 1; // 새로운 섬 발견
                }
            }
        }

        results.push(island_count);
    }

    for result in results {
        println!("{}", result);
    }
}

fn dfs(map: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize, w: usize, h: usize) {
    // 방향 벡터 (상, 하, 좌, 우, 대각선)
    let directions = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    visited[x][y] = true; // 현재 노드 방문 처리

    for (dx, dy) in directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_x < h as isize && new_y >= 0 && new_y < w as isize {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if map[new_x][new_y] == 1 && !visited[new_x][new_y] {
                dfs(map, visited, new_x, new_y, w, h); // 재귀 호출
            }
        }
    }
}
