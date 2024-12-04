use std::fs::File;
use std::io::Read;

fn parse_numbers(line: &str) -> Vec<i32> {
    /* Parse Numbers from String. If string is empty,
    an empty list will be returned.
    */
    line.trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn get_deltas(numbers: Vec<i32>) -> Vec<i32> {
    // Calculate deltas between two adjacent levels
    let mut deltas: Vec<i32> = Vec::new();

    for i in 0..numbers.len() - 1 {
        deltas.push(numbers[i + 1] - numbers[i]);
    }

    return deltas;
}

fn safety_check(deltas: Vec<i32>) -> bool {
    // Are all deltas > 0 or all deltas < 0? -> if false    == not safe
    // Is at least one delta == 0? -> if true               == not safe
    // Is at least one delta.abs() > 3? -> if true          == not safe

    let all_positive = deltas.iter().all(|&delta| delta > 0);
    let all_negative = deltas.iter().all(|&delta| delta < 0);
    let contains_zero = deltas.iter().any(|&delta| delta == 0);
    let outlier = deltas.iter().any(|&delta| delta.abs() > 3);

    // Returns true if safe
    if (all_positive || all_negative) && !contains_zero && !outlier {
        return true;
    } else {
        return false;
    }
}

pub fn run() -> i32 {
    // let reports = r#"
    // 7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9
    // "#;

    // Read file
    let mut file = File::open("src/input/day2.txt").expect("Couldn't open file");
    let mut reports = String::new();
    file.read_to_string(&mut reports)
        .expect("Couldn't read file");

    let mut safes = 0;

    for i in reports.lines() {
        let numbers = parse_numbers(i);
        if !numbers.is_empty() {
            let deltas = get_deltas(numbers);
            if safety_check(deltas) {
                safes += 1;
            }
        }
    }
    return safes;
}
