use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(input_file: &str) -> Vec<Vec<char>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn is_valid_position(grid: &[Vec<char>], row: usize, column: usize) -> bool {
    if row == 0 && column == 0
        || row == 0 && column == grid[row].len() - 1
        || row == grid.len() - 1 && column == 0
        || row == grid.len() - 1 && column == grid[row].len() - 1
    {
        return grid[row][column] != '.';
    }

    if row == 0 {
        return ((grid[row][column - 1] != '.') as i32)
            + ((grid[row][column + 1] != '.') as i32)
            + ((grid[row + 1][column] != '.') as i32)
            + ((grid[row + 1][column - 1] != '.') as i32)
            + ((grid[row + 1][column + 1] != '.') as i32)
            < 4;
    }

    if column == 0 {
        return ((grid[row - 1][column] != '.') as i32)
            + ((grid[row + 1][column] != '.') as i32)
            + ((grid[row][column + 1] != '.') as i32)
            + ((grid[row + 1][column + 1] != '.') as i32)
            + ((grid[row - 1][column + 1] != '.') as i32)
            < 4;
    }

    if row == grid.len() - 1 {
        return ((grid[row][column - 1] != '.') as i32)
            + ((grid[row][column + 1] != '.') as i32)
            + ((grid[row - 1][column] != '.') as i32)
            + ((grid[row - 1][column - 1] != '.') as i32)
            + ((grid[row - 1][column + 1] != '.') as i32)
            < 4;
    }

    if column == grid[row].len() - 1 {
        return ((grid[row - 1][column] != '.') as i32)
            + ((grid[row + 1][column] != '.') as i32)
            + ((grid[row][column - 1] != '.') as i32)
            + ((grid[row - 1][column - 1] != '.') as i32)
            + ((grid[row + 1][column - 1] != '.') as i32)
            < 4;
    }

    return ((grid[row - 1][column - 1] != '.') as i32)
        + ((grid[row - 1][column] != '.') as i32)
        + ((grid[row - 1][column + 1] != '.') as i32)
        + ((grid[row][column - 1] != '.') as i32)
        + ((grid[row][column + 1] != '.') as i32)
        + ((grid[row + 1][column - 1] != '.') as i32)
        + ((grid[row + 1][column] != '.') as i32)
        + ((grid[row + 1][column + 1] != '.') as i32)
        < 4;
}

fn solve1(grid: &mut Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut result = 0;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] != '.' && is_valid_position(grid, i, j) {
                result += 1;
                grid[i][j] = '.'; // this is used for solve2, for solve1 only this must be deleted
            }
        }
    }

    result
}

fn solve2(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut result = 0;

    loop {
        let temp = solve1(grid);
        if temp == 0 {
            break;
        }
        result += temp;
    }

    result
}

fn main() {
    let input_file = "src/input.txt";
    let mut grid: Vec<Vec<char>> = read_input(input_file);

    // let result1 = solve1(&mut grid);
    let result2 = solve2(&mut grid);

    // println!("{result1}");
    println!("{result2}");
}
