use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = env::current_dir().unwrap().join("./input.txt");
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut nested_ints: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let splitted: Vec<&str> = line.split(' ').collect();
                let ints: Vec<i32> = splitted.iter().map(|s| s.parse::<i32>().unwrap()).collect();
                nested_ints.push(ints);
            }
            Err(err) => println!("Failed to get line: {}", err),
        }
    }

    part_one(&nested_ints);
    part_two(&nested_ints);
}

fn part_one(nested_ints: &Vec<Vec<i32>>) {
    let mut safe_count = 0;
    for ints in nested_ints {
        let steps = calc_steps(ints);
        let safe = is_safe(&steps);
        if safe {
            safe_count += 1
        }
    }
    println!("Safe count: {}", safe_count);
}

fn part_two(nested_ints: &Vec<Vec<i32>>) {
    let mut safe_count = 0;
    for ints in nested_ints {
        let safe = is_safe_problem_dampener(&ints);
        if safe {
            safe_count += 1;
        }
    }
    println!("Safe count with problem dampener: {}", safe_count);
}

fn calc_steps(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..numbers.len() - 1 {
        let current = numbers.get(i).unwrap();
        let next = numbers.get(i + 1).unwrap();
        result.push(next - current);
    }
    return result;
}

fn is_safe(steps: &Vec<i32>) -> bool {
    let is_increasing = steps.get(0).unwrap() > &0;
    return steps
        .iter()
        .all(|&x| (x.abs() > 0 && x.abs() < 4) && ((x > 0) == is_increasing));
}

fn is_safe_problem_dampener(ints: &Vec<i32>) -> bool {
    let steps = &calc_steps(ints);
    let safe = is_safe(steps);
    if safe {
        return true;
    }

    for i in 0..ints.len() {
        let mut modified = ints.clone();
        modified.remove(i);
        let modified_steps = calc_steps(&modified);
        let safe = is_safe(&modified_steps);
        if safe {
            return true;
        }
    }
    return false;
}
