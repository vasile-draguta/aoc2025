use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn read_input(input_file: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    let mut line_num = 0;

    for line in reader.lines() {
        for current_token in line.unwrap().split(' ') {
            let current = current_token.trim();
            if current.is_empty() {
                continue;
            }
            if current.contains("+") || current.contains("*") {
                operations.push(current.trim().chars().nth(0).unwrap());
            } else {
                let to_int: u64 = current.parse().unwrap();
                if nums.get(line_num).is_none() {
                    nums.push(Vec::new());
                }
                nums[line_num].push(to_int);
            }
        }
        line_num += 1;
    }

    (nums, operations)
}

fn read_from_right_to_left(
    input_file: &str,
    num_of_lines: usize,
    num_of_columns: usize,
) -> Vec<Vec<u64>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut from_right_to_left: Vec<Vec<u64>> = Vec::new();

    for _ in 0..num_of_columns {
        from_right_to_left.push(vec![0; num_of_lines]);
    }

    let mut char_matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().rev().collect())
        .collect();
    char_matrix.pop();

    let mut current_list = 0;
    let mut current_index = 0;

    for i in 0..char_matrix[0].len() {
        let mut num_break: bool = true;

        for j in 0..num_of_lines {
            if char_matrix[j][i] != ' ' {
                let digit: u64 = char_matrix[j][i].to_digit(10).unwrap().into();
                from_right_to_left[current_list][current_index] =
                    from_right_to_left[current_list][current_index] * 10 + digit;
                num_break = false;
            }
        }
        if num_break == true {
            current_list += 1;
            current_index = 0;
        } else {
            current_index += 1;
        }
    }

    from_right_to_left
}

fn solve1(nums: &[Vec<u64>], operations: &[char]) -> u64 {
    let mut result: u64 = 0;

    for i in 0..operations.len() {
        let operation = operations[i];
        let mut temp: u64 = if operation == '+' { 0 } else { 1 };
        for j in 0..nums.len() {
            if operation == '+' {
                temp += nums[j][i];
            } else {
                temp *= nums[j][i];
            }
        }
        result += temp;
    }

    result
}

fn solve2(nums: &[Vec<u64>], operations: &[char]) -> u64 {
    let mut result: u64 = 0;

    let mut current_index = 0;

    for i in (0..operations.len()).rev() {
        let operation = operations[i];
        let mut temp: u64 = if operation == '+' { 0 } else { 1 };

        for j in 0..nums[0].len() {
            if nums[current_index][j] == 0 {
                continue;
            }

            if operation == '+' {
                temp += nums[current_index][j];
            } else {
                temp *= nums[current_index][j];
            }
        }
        current_index += 1;
        result += temp;
    }

    result
}

fn main() {
    let input_file: &str = "src/input.txt";
    let (nums, operations): (Vec<Vec<u64>>, Vec<char>) = read_input(input_file);
    let from_right_to_left: Vec<Vec<u64>> =
        read_from_right_to_left(input_file, nums.len(), nums[0].len());

    let result1 = solve1(&nums, &operations);
    let result2 = solve2(&from_right_to_left, &operations);
    println!("{}", result1);
    println!("{}", result2);
}
