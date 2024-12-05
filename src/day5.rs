use crate::utils::read_txt;
// Approach
// --------

// 1. Find correctly-ordered updates
// - Rules interpretation!

// 2. sum middle page number of correct updates
//   Are all correct_updates.len() % 2 != 0?

pub fn main() {
    let file_content = read_txt("src/input/day5.example.txt");
    // println!("{:?}", file_content);

    // Split into rules & updates at "\n\n"
    let (rules_content, updates_content) = file_content
        .split_once("\n\n")
        .expect("The string does not contain '\\n\\n'");

    println!("{:?}", rules_content);
    println!("\n {:?}\n", updates_content);

    // Rules & Updates into lists
    let rules: Vec<&str> = rules_content.split("\n").collect();
    let updates: Vec<&str> = updates_content.split("\n").collect();

    // How to check each update if it complies to the rules?
}
