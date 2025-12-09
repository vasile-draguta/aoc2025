use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn distance_between(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find(parent, parent[i]);
    }
    parent[i]
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let xroot = find(parent, x);
    let yroot = find(parent, y);
    if xroot == yroot {
        return;
    }
    if rank[xroot] < rank[yroot] {
        parent[xroot] = yroot;
    } else if rank[xroot] > rank[yroot] {
        parent[yroot] = xroot;
    } else {
        parent[yroot] = xroot;
        rank[xroot] += 1;
    }
}

fn read_input(input: &str) -> Vec<Point> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        points.push(Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        });
    }
    points
}

fn solve1(points: &Vec<Point>, num_pairs: usize) -> usize {
    let n = points.len();

    let mut pairs: Vec<(f64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_between(&points[i], &points[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = vec![0; n];

    for i in 0..num_pairs.min(pairs.len()) {
        let (_, src, dst) = pairs[i];
        union(&mut parent, &mut rank, src, dst);
    }

    let mut component_count = std::collections::HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        *component_count.entry(root).or_insert(0usize) += 1;
    }

    let mut sizes: Vec<usize> = component_count.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product()
}

fn solve2(points: &Vec<Point>) -> f64 {
    let n = points.len();

    let mut pairs: Vec<(f64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_between(&points[i], &points[j]);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = vec![0; n];

    let mut edges_added = 0;
    let mut last_src = 0;
    let mut last_dst = 0;

    for (_, src, dst) in &pairs {
        let root_src = find(&mut parent, *src);
        let root_dst = find(&mut parent, *dst);

        if root_src != root_dst {
            union(&mut parent, &mut rank, *src, *dst);
            edges_added += 1;
            last_src = *src;
            last_dst = *dst;

            if edges_added == n - 1 {
                break;
            }
        }
    }

    points[last_src].x * points[last_dst].x
}

fn main() {
    let points = read_input("src/input.txt");
    let result1 = solve1(&points, 1000);
    let result2 = solve2(&points);
    println!("{result1}");
    println!("{result2}");
}
