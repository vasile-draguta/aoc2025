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

fn is_inside_or_on_boundary(poly: &[(i64, i64)], px: i64, py: i64) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }

    if poly.iter().any(|&v| v == (px, py)) {
        return true;
    }

    let mut inside = false;
    let (mut x1, mut y1) = poly[n - 1];

    for i in 0..n {
        let (x2, y2) = poly[i];

        let cross = (x2 - x1) * (py - y1) - (px - x1) * (y2 - y1);
        if cross == 0 {
            if x1.min(x2) <= px && px <= x1.max(x2) && y1.min(y2) <= py && py <= y1.max(y2) {
                return true;
            }
        }

        if (y1 > py) != (y2 > py) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let lhs = dx * (py - y1);
            let rhs = (px - x1) * dy;

            if (dy > 0 && lhs > rhs) || (dy < 0 && lhs < rhs) {
                inside = !inside;
            }
        }

        (x1, y1) = (x2, y2);
    }

    inside
}

fn solve2(points: &Vec<(i64, i64)>) -> i64 {
    let n = points.len();
    let mut max_area = 0;

    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for k in 0..n {
        edges.push((points[k], points[(k + 1) % n]));
    }

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            let area = (max_x - min_x + 1) * (max_y - min_y + 1);

            if area <= max_area {
                continue;
            }

            let corners = [
                (min_x, min_y),
                (max_x, min_y),
                (min_x, max_y),
                (max_x, max_y),
            ];
            let mut valid = true;
            for &(cx, cy) in &corners {
                if !is_inside_or_on_boundary(points, cx, cy) {
                    valid = false;
                    break;
                }
            }

            if valid {
                for &(vx, vy) in points {
                    if vx > min_x && vx < max_x && vy > min_y && vy < max_y {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                for &((ex1, ey1), (ex2, ey2)) in &edges {
                    if ex1 == ex2 {
                        let edge_x = ex1;
                        let edge_min_y = ey1.min(ey2);
                        let edge_max_y = ey1.max(ey2);
                        if edge_x > min_x && edge_x < max_x {
                            if edge_min_y < max_y && edge_max_y > min_y {
                                valid = false;
                                break;
                            }
                        }
                    }
                    if ey1 == ey2 {
                        let edge_y = ey1;
                        let edge_min_x = ex1.min(ex2);
                        let edge_max_x = ex1.max(ex2);
                        if edge_y > min_y && edge_y < max_y {
                            if edge_min_x < max_x && edge_max_x > min_x {
                                valid = false;
                                break;
                            }
                        }
                    }
                }
            }

            if valid {
                max_area = area;
            }
        }
    }
    max_area
}

fn main() {
    let points = read_input("src/input.txt");
    let result1 = solve1(&points);
    let result2 = solve2(&points);
    println!("{result1}");
    println!("{result2}");
}
