use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(input_file: &str) -> (Vec<Vec<char>>, usize) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let diagram: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let start_index = diagram[0].len() / 2;

    (diagram, start_index)
}

fn solve1(
    diagram: &mut Vec<Vec<char>>,
    mut current_row: usize,
    current_column: usize,
    result: &mut i32,
) {
    while current_row <= (diagram.len() - 1) && diagram[current_row][current_column] == '.' {
        diagram[current_row][current_column] = '|';
        current_row += 1;
    }

    if current_row < diagram.len() - 1 {
        if diagram[current_row][current_column] == '^' {
            *result += 1;
            solve1(diagram, current_row, current_column - 1, result);
            solve1(diagram, current_row, current_column + 1, result);
        }
    }
}

fn solve2(diagram: &mut Vec<Vec<char>>, dp: &mut Vec<Vec<u64>>) {
    for i in (0..diagram.len()).rev() {
        for j in 0..diagram[i].len() {
            if i == diagram.len() - 1 {
                if diagram[i][j] == '|' {
                    dp[i][j] = 1;
                }
            } else {
                if diagram[i + 1][j] == '|' {
                    dp[i][j] = dp[i + 1][j];
                } else if diagram[i + 1][j] == '^' {
                    dp[i][j] = dp[i + 1][j - 1] + dp[i + 1][j + 1];
                }
            }
        }
    }
}

fn main() {
    let input_file = "src/input.txt";
    let (mut diagram, start_index) = read_input(input_file);
    let mut result = 0;
    let mut dp: Vec<Vec<u64>> = Vec::new();
    for _ in 0..diagram.len() {
        dp.push(vec![0; diagram[0].len()]);
    }
    solve1(&mut diagram, 1, start_index, &mut result);
    solve2(&mut diagram, &mut dp);

    println!("{:?}", result);
    println!("{:?}", dp[0][dp[0].len() / 2]);
}
