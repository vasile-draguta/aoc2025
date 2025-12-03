use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::iter::zip;

fn read_input(input: &str, sequences: &mut Vec<String>) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line);
        sequences.push(line.unwrap());
    }
}

fn solve1(sequences: &[String]) -> u32 {
    let mut sum: u32 = 0;
    let mut stack: Vec<char> = Vec::new();

    for sequence in sequences {
        for (c, i) in zip(sequence.chars(), 0..sequence.len()) {
            if stack.len() == 2 {
                if stack[1] > stack[0] {
                    stack[0] = stack[1];
                    stack[1] = c;
                }

                if c > stack[0] && i > sequence.len() - 1 {
                    stack.clear();
                    stack.push(c);
                } else if c > stack[1] {
                    stack.pop();
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }
        println!("{:?}", stack);
        sum += stack[0].to_digit(10).unwrap() * 10 + stack[1].to_digit(10).unwrap();
        stack.clear();
    }

    sum
}

fn solve2(sequences: &[String]) -> u64 {
    let mut sum: u64 = 0;
    let mut stack: Vec<char> = Vec::new();

    for sequence in sequences {
        for (c, i) in zip(sequence.chars(), 0..sequence.len()) {
            while !stack.is_empty()
                && c > stack[stack.len() - 1]
                && sequence.len() - i + stack.len() > 12
            {
                stack.pop();
            }

            if stack.is_empty() || stack.len() < 12 {
                stack.push(c);
            } else {
                if stack[stack.len() - 1] < c {
                    stack.pop();
                    stack.push(c);
                }
            }
        }
        println!("{:?}", stack);
        sum += stack
            .clone()
            .into_iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        stack.clear();
    }

    sum
}

fn main() {
    let mut sequences: Vec<String> = Vec::new();

    read_input("src/input.txt", &mut sequences);

    let sum1 = solve1(&sequences);
    let sum2 = solve2(&sequences);

    println!("{sum1}");
    println!("{sum2}");
}
