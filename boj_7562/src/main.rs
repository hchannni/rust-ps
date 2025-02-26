use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let t: usize = input.trim().parse().unwrap(); // 테스트 케이스의 개수

    let directions = vec![
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("input error");
        let n: usize = input.trim().parse().unwrap(); // 체스판의 크기

        input.clear();
        io::stdin().read_line(&mut input).expect("input error");
        let start: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(); // 시작 위치

        input.clear();
        io::stdin().read_line(&mut input).expect("input error");
        let target: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(); // 목표 위치

        let mut visited = vec![vec![false; n]; n]; // 방문 여부 배열
        let mut queue = VecDeque::new();
        queue.push_back((start[0], start[1], 0)); // (x, y, 이동 횟수)
        visited[start[0]][start[1]] = true;

        while let Some((x, y, moves)) = queue.pop_front() {
            if x == target[0] && y == target[1] {
                println!("{}", moves); // 목표 위치에 도달하면 이동 횟수 출력
                break;
            }

            for (dx, dy) in &directions {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if new_x < n && new_y < n && !visited[new_x][new_y] {
                    visited[new_x][new_y] = true; // 방문 처리
                    queue.push_back((new_x, new_y, moves + 1)); // 큐에 추가
                }
            }
        }
    }
}
