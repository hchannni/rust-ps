use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).expect("input error");
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort();
    let mut count = 0;

    for i in 0..n {
        let target = numbers[i];
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            // 찾고 있는 숫자 그 자체는 제외
            if left == i {
                left += 1;
                continue;
            }
            if right == i {
                right -= 1;
                continue;
            }

            let sum = numbers[left] + numbers[right];

            if sum == target {
                count += 1; // 좋다 수 발견
                break;
            } else if sum < target {
                left += 1; // 합이 작으면 왼쪽 포인터 이동
            } else {
                right -= 1; // 합이 크면 오른쪽 포인터 이동
            }
        }
    }

    println!("{}", count);
}
