pub mod input;

fn main() {
    let mut list1: Vec<&str> = Vec::new();
    let mut list2: Vec<&str> = Vec::new();

    for line in input::INPUT.lines() {

        let splitted: Vec<&str> = line.split("  ").collect();

        let list1val: &str = splitted.get(0).expect("Expected list 1 value but got nothing");
        let list2val: &str = splitted.get(1).expect("Expected list 2 value but got nothing");

        list1.push(list1val.trim());
        list2.push(list2val.trim());
    }

    list1.sort();
    list2.sort();


    part_one(&list1, &list2);
    part_two(&list1, &list2);
}

fn part_one(list1: &Vec<&str>, list2: &Vec<&str>) {
    let mut diffs: Vec<u32> = Vec::new();

    for i in 0..list1.len() {
        let list1num = list1.get(i).unwrap().parse::<i32>().expect("Expected to be able to parse list 1 input");
        let list2num = list2.get(i).unwrap().parse::<i32>().expect("Expected to be able to parse list 2 input");

        let diff = list1num.abs_diff(list2num);
        diffs.push(diff);
    }

    let sum: u32 = diffs.iter().sum();
    println!("Sum of diff: {}", sum);
}

fn part_two(list1: &Vec<&str>, list2: &Vec<&str>) {
    let mut similarity_score: i32 = 0;

    for input1 in list1.iter() {
        let mut timesFound = 0;
        for input2 in list2.iter() {
            if input1.eq(input2) {
                timesFound += 1;
            }
        }
        let input1num = input1.parse::<i32>().expect("Failed to parse input number");
        similarity_score += input1num * timesFound;
    }

    println!("Similarity score: {}", similarity_score);
}