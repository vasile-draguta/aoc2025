use std::collections::HashSet;
use std::collections::VecDeque;
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

fn solve1(ranges: &Vec<(u64, u64)>, ids: &Vec<u64>) -> u64 {
    let mut result: u64 = 0;

    for id in ids {
        let mut is_in_range: bool = false;
        for range in ranges {
            if *id >= range.0 && *id <= range.1 {
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

fn merge_intevals(first: (u64, u64), second: (u64, u64)) -> (u64, u64) {
    if first.0 <= second.0 && first.1 >= second.1 {
        return first;
    }

    if first.0 >= second.0 && first.1 <= second.1 {
        return second;
    }

    if first.0 < second.0
        && first.1 < second.1
        && (first.1 >= second.0 && first.1 <= second.1
            || first.0 >= second.0 && first.0 <= second.1)
    {
        return (first.0, second.1);
    }

    if second.0 < first.0
        && second.1 < first.1
        && (second.1 >= first.0 && second.1 <= first.1
            || second.0 >= first.0 && second.0 <= first.1)
    {
        return (second.0, first.1);
    }

    (0, 0)
}

fn solve2(ranges: &mut Vec<(u64, u64)>) -> u64 {
    let mut result = 0;
    let (mut first, mut second): ((u64, u64), (u64, u64));

    ranges.sort();

    let mut unique_ranges: HashSet<(u64, u64)> = HashSet::new();
    let mut deque: VecDeque<(u64, u64)> = ranges.iter().cloned().collect();

    while deque.len() >= 2 {
        first = deque.pop_front().unwrap();
        second = deque.pop_front().unwrap();

        let res = merge_intevals(first, second);

        if res == (0, 0) {
            unique_ranges.insert(first);
            deque.insert(0, second);
        } else {
            deque.insert(0, res);
        }
    }

    while !deque.is_empty() {
        unique_ranges.insert(deque.pop_front().unwrap());
    }

    for interval in unique_ranges {
        let a = interval.0;
        let b = interval.1;
        println!("{a}, {b}");
        result += b - a + 1;
    }

    result
}

fn main() {
    let input_file = "src/input.txt";
    let (mut ranges, ids): (Vec<(u64, u64)>, Vec<u64>) = read_input(input_file);
    let result1 = solve1(&ranges, &ids);
    let result2 = solve2(&mut ranges);
    println!("{result1}");
    println!("{result2}");
}
