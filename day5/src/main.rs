use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(input_file: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut read_ids = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            read_ids = true;
            continue;
        }

        if read_ids {
            let x: u64 = line.parse().unwrap();
            ids.push(x);
            println!("{x}");
        } else {
            let mut split = line.split('-');
            let a: u64 = split.next().unwrap().parse().unwrap();
            let b: u64 = split.next().unwrap().parse().unwrap();
            println!("{a}, {b}");
            ranges.push((a, b));
        }
    }

    (ranges, ids)
}

fn solve1(ranges: Vec<(u64, u64)>, ids: Vec<u64>) -> u64 {
    let mut result: u64 = 0;

    for id in ids {
        let mut is_in_range: bool = false;
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                is_in_range = true;
                break;
            }
        }
        if is_in_range {
            result += 1;
        }
    }

    result
}

fn main() {
    let input_file = "src/input.txt";
    let (ranges, ids): (Vec<(u64, u64)>, Vec<u64>) = read_input(input_file);
    let result1 = solve1(ranges, ids);
    println!("{result1}");
}
