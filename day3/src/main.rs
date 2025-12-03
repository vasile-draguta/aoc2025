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

fn solve(sequences: &[String]) -> u32 {
    let mut sum: u32 = 0;
    let mut stack: Vec<char> = Vec::new();

    for sequence in sequences {
        stack.clear();

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
    }

    sum
}

fn main() {
    let mut sequences: Vec<String> = Vec::new();

    read_input("src/input.txt", &mut sequences);

    let sum = solve(&sequences);

    println!("{sum}");
}
