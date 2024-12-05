use crate::utils::read_txt;
use std::collections::HashMap;

// Approach
// --------

// 1. Find correctly-ordered updates
// - Rules interpretation!

// 2. sum middle page number of correct updates
//   Are all correct_updates.len() % 2 != 0?

pub fn main() -> i32 {
    let file_content = read_txt("src/input/day5.txt");
    // println!("{:?}", file_content);

    // Split into rules & updates at "\n\n"
    let (rules_content, updates_content) = file_content
        .split_once("\n\n")
        .expect("The string does not contain '\\n\\n'");

    // println!("{:?}", rules_content);
    // println!("\n {:?}\n", updates_content);

    // Rules & Updates into lists

    // Parse rules into Vec<(i32, i32)>
    let rules: Vec<(i32, i32)> = rules_content
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|s| s.trim().parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    // for rule in rules {
    //     println!("{:?}", rule);
    // }

    // Parse updates into Vec<Vec<i32>>
    let updates: Vec<Vec<i32>> = updates_content
        .lines()
        .map(|line| line.split(',').map(|n| n.trim().parse().unwrap()).collect())
        .collect();

    // for update in updates {
    //     println!("{:?}", update);
    // }

    fn update_in_order(update: Vec<i32>, rules: Vec<(i32, i32)>) -> bool {
        let page_indices: HashMap<i32, usize> = update
            .iter()
            .enumerate()
            .map(|(idx, &page)| (page, idx))
            .collect();

        for (before, after) in rules.iter() {
            if page_indices.contains_key(before) && page_indices.contains_key(after) {
                if page_indices[before] >= page_indices[after] {
                    return false;
                }
            }
        }
        return true;
    }

    let mut sum_middle = 0;
    for update in updates {
        if update_in_order(update.clone(), rules.clone()) {
            // println!("Yeah")
            // Take Number in the middle & add it
            let mid_idx = update.len() / 2;
            sum_middle += update[mid_idx];
        }
    }

    return sum_middle;

    // How to check each update if it complies to the rules?
}
