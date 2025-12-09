use std::fs::File;
use std::io::{BufReader, prelude::*};

fn read_input(input: &str) -> Vec<(i64, i64)> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut points = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        points.push((
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        ));
    }
    points
}

fn solve1(points: &Vec<(i64, i64)>) -> i64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let length = (points[i].0 - points[j].0).abs() as i64 + 1;
            let width = (points[i].1 - points[j].1).abs() as i64 + 1;
            if length * width > max_area {
                max_area = length * width;
            }
        }
    }
    max_area
}

fn main() {
    let points = read_input("src/input.txt");
    let result1 = solve1(&points);
    println!("{result1}");
}
