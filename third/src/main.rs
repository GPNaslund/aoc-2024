use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file_path = env::current_dir().unwrap().join("./input.txt");
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    let mut all_results = vec![];

    for line in reader.lines() {
        match line {
            Ok(val) => {
                let mut results = vec![];
                for (_, [vals]) in re.captures_iter(val.as_str()).map(|c| c.extract()) {
                    results.push(vals.to_string());
                }
                all_results.push(results.clone());
            }
            Err(err) => println!("Failed to get line: {}", err),
        }
    }

    let mut sum = 0;

    for pair_array in all_results.iter() {
        for pair in pair_array.iter() {
            let splitted: Vec<&str> = pair.split(',').collect();
            let first: i32 = splitted
                .get(0)
                .expect("Expected a string with atleast 1 number")
                .parse::<i32>()
                .expect("Expected first part to be  a number");
            let second: i32 = splitted
                .get(1)
                .expect("Expected a string with two numbers")
                .parse::<i32>()
                .expect("Expected second part to be a number");
            sum += first * second;
        }
    }

    println!("The total sum is: {}", sum);
}
