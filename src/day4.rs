use std::array;
use std::collections::btree_set::Range;
use std::fs::File;
use std::io::Read;

const EXAMPLE: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

fn txt_2_matrix(txt: &str) -> Vec<Vec<char>> {
    /*
    Python
    ------
    lines = text.strip().split('\n')
    matrix = [list(line) for line in lines]
    */
    let lines: Vec<&str> = txt.trim().lines().collect();
    return lines.iter().map(|&line| line.chars().collect()).collect();
}

fn count_xmas(txt: &str) -> i32 {
    /*
    - horizontal,                       <-,->
    - vertical,                         v, A
    - diagonal,                         \, /
    - written backwards,                samx
    - even overlapping other words      xmasamxmas
    */

    let matrix = txt_2_matrix(txt);
    let word_to_find = "XMAS";
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let rows = matrix.len();
    let cols = matrix[0].len();

    fn word_found(
        matrix: &Vec<Vec<char>>,
        x: i32,
        y: i32,
        word: &str,
        dx: i32,
        dy: i32,
        rows: i32,
        cols: i32,
    ) -> bool {
        for i in 0..word.len() {
            let nx = x + dx * i as i32;
            let ny = y + dy * i as i32;
            if nx < 0 || ny < 0 || nx >= rows || ny >= cols {
                return false;
            }
            if matrix[nx as usize][ny as usize] != word.chars().nth(i).unwrap() {
                return false;
            }
        }
        return true;
    }

    let mut cnt = 0;

    for x in 0..rows {
        for y in 0..cols {
            for i in 0..directions.len() {
                let dx = directions[i].0;
                let dy = directions[i].1;
                if word_found(
                    &matrix,
                    x as i32,
                    y as i32,
                    word_to_find,
                    dx as i32,
                    dy as i32,
                    rows as i32,
                    cols as i32,
                ) {
                    cnt += 1
                }
            }
        }
    }

    return cnt;
}

fn file_2_content(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Couldn't open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Couldn't read file");
    file_content
}

fn count_xmas_v2(txt: &str) -> i32 {
    let matrix = txt_2_matrix(txt);
    let words_to_find: [&str; 2] = ["MAS", "SAM"];
    let rows = matrix.len();
    let cols = matrix[0].len();

    fn x_found(
        matrix: &Vec<Vec<char>>,
        x: i32,
        y: i32,
        words: [&str; 2],
        dx: i32,
        dy: i32,
        rows: i32,
        cols: i32,
    ) -> bool {
        let mut letters = Vec::new();

        // i = -1, 0, 1
        for i in -1..2 {
            let nx = x + dx * i as i32;
            let ny = y + dy * i as i32;
            if nx < 0 || ny < 0 || nx >= rows || ny >= cols {
                return false;
            }
            letters.push(matrix[nx as usize][ny as usize]);
        }
        let letters_str: String = letters.into_iter().collect();
        words.contains(&letters_str.as_str())
    }

    let mut cnt = 0;

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            // unklar ob das so in rust geht
            if matrix[x as usize][y as usize] != 'A' {
                continue;
            }
            let dia1 = x_found(
                &matrix,
                x as i32,
                y as i32,
                words_to_find,
                1,
                1,
                rows as i32,
                cols as i32,
            );
            let dia2 = x_found(
                &matrix,
                x as i32,
                y as i32,
                words_to_find,
                1,
                -1,
                rows as i32,
                cols as i32,
            );
            if dia1 && dia2 {
                cnt += 1
            }
        }
    }

    return cnt;
}

pub fn main() -> i32 {
    let new_txt = file_2_content("src/input/day4.txt");
    // return count_xmas(&new_txt);
    return count_xmas_v2(&new_txt);
}
