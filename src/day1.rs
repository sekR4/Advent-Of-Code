pub fn run() -> i32 {
    // Example Input
    let example = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#;

    // Create Lists
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut differences: Vec<i32> = Vec::new();

    // Add parsed numbers to lists
    for line in example.lines() {
        let numbers: Vec<&str> = line.trim().split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(left), Ok(right)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }

    // Sort lists
    left_list.sort();
    right_list.sort();

    // for each pair calculate differences
    for i in 0..left_list.len() {
        let _diff = (left_list[i] - right_list[i]).abs();
        differences.push(_diff);
    }

    // Return sum of differences
    let sum_differences: i32 = differences.iter().sum();
    return sum_differences;
}
