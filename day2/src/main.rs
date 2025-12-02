use std::fs;

fn read_input(input_file: String, ranges: &mut Vec<String>) {
    let input: String = fs::read_to_string(input_file).unwrap();
    for range in input.split(',') {
        println!("{range}");
        ranges.push(range.to_string());
    }
}

fn solve1(ranges: &[String]) -> i128 {
    let (mut a, mut b): (i128, i128);
    let mut sum: i128 = 0;
    for range in ranges {
        let mut split = range.split('-');
        a = split.next().unwrap().parse().unwrap();
        b = split.next().unwrap().parse().unwrap();
        for i in a..=b {
            let temp: String = i.to_string();
            let first = &temp[0..temp.len() / 2];
            let second = &temp[temp.len() / 2..];
            if first == second {
                sum += i;
            }
        }
    }

    sum
}

fn is_made_of_prefix(candidate: &String, prefix: String) -> bool {
    if candidate.len() % prefix.len() != 0 {
        return false;
    }

    let mut slice: String;
    for i in (0..candidate.len()).step_by(prefix.len()) {
        slice = candidate[i..i + prefix.len()].to_string();
        if prefix != slice {
            return false;
        }
    }

    true
}

fn solve2(ranges: &[String]) -> i128 {
    let (mut a, mut b): (i128, i128);
    let mut sum: i128 = 0;
    let mut prefix: String;

    for range in ranges {
        let mut split = range.split('-');
        a = split.next().unwrap().parse().unwrap();
        b = split.next().unwrap().parse().unwrap();

        for i in a..=b {
            let temp: String = i.to_string();

            for j in 1..=temp.len() / 2 {
                prefix = temp[..j].to_string();

                if is_made_of_prefix(&temp, prefix) {
                    sum += i;
                    break;
                }
            }
        }
    }

    sum
}

fn main() {
    let input_file: String = "src/input.txt".to_string();
    let mut ranges: Vec<String> = Vec::new();

    read_input(input_file, &mut ranges);

    let sum1 = solve1(&ranges);
    let sum2 = solve2(&ranges);

    println!("{sum1}"); // 16793817782
    println!("{sum2}"); // 27469417404
}
