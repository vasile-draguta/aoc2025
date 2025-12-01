use std::fs::File;
use std::io::{BufReader, prelude::*};

fn read_input(input: String, rotations: &mut Vec<String>) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        rotations.push(line.unwrap());
    }
}

fn solve1(rotations: &[String]) -> i32 {
    let mut result = 0;
    let mut dial = 50;
    let mut num: i32;

    for rotation in rotations {
        num = rotation[1..].parse().unwrap();
        if let Some(first_char) = rotation.chars().nth(0) {
            if first_char == 'L' {
                dial = (dial - num + 100).rem_euclid(100);
            } else {
                dial = (dial + num).rem_euclid(100);
            }
        }

        if dial == 0 {
            result += 1;
        }
    }

    result
}

fn solve2(rotations: &[String]) -> i32 {
    let mut result = 0;
    let mut dial = 50;
    let mut num: i32;

    for rotation in rotations {
        num = rotation[1..].parse().unwrap();

        if let Some(first_char) = rotation.chars().nth(0) {
            if first_char == 'L' {
                for _ in 0..num {
                    dial = dial - 1;
                    if dial == -1 {
                        dial = 99
                    }
                    if dial == 0 {
                        result += 1
                    }
                }
            } else {
                for _ in 0..num {
                    dial = dial + 1;
                    if dial == 100 {
                        dial = 0
                    }
                    if dial == 0 {
                        result += 1
                    }
                }
            }
        }
    }

    result
}

fn main() {
    let input = "src/input.txt".to_string();
    let mut rotations: Vec<String> = Vec::new();
    read_input(input, &mut rotations);

    let result1 = solve1(&rotations);
    let result2 = solve2(&rotations);
    println!("Result1: {result1}\nResult2: {result2}")
}
