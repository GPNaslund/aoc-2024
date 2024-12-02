use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = env::current_dir().unwrap().join("../input.txt");
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {}
            Err(err) => println!("Failed to read line!"),
        }
    }
}

fn is_safe(report: String) {
    let splitted: Vec<&str> = report.split(' ').collect();
    let is_increasing: bool = splitted.get(0).unwrap().parse::<i32>() >
    for i in 0..splitted.len() {
        
    }
}
