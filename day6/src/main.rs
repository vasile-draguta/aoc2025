use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    let input_file: &str = "src/input.txt";
    let (nums, operations): (Vec<Vec<u64>>, Vec<char>) = read_input(input_file);

    let result1 = solve1(&nums, &operations);
    println!("{}", result1);
}
