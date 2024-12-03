use std::fs::File;
use std::io::Read;

pub fn run() -> i32 {
    // let example = r#"
    //     3   4
    //     4   3
    //     2   5
    //     1   3
    //     3   9
    //     3   3
    // "#;

    // Read file
    let mut file = File::open("src/input/day1.txt").expect("Couldn't open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Couldn't read file");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    // Add parsed numbers to lists
    for line in file_content.lines() {
        let numbers: Vec<&str> = line.trim().split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(left), Ok(right)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }

    let mut sum_of_lefties: i32 = 0;

    for i in 0..left_list.len() {
        let _cnt = right_list.iter().filter(|&&x| x == left_list[i]).count();
        let _member_times_cnt = left_list[i] * _cnt as i32;
        sum_of_lefties += _member_times_cnt;
    }

    return sum_of_lefties;
}
