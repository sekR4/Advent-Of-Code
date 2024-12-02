pub fn run() {
    // Example Input
    let example = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in example.lines() {
        let numbers: Vec<&str> = line.trim().split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(left), Ok(right)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }

    println!("Left list: {:?}", left_list);
    println!("Right list: {:?}", right_list);
}
