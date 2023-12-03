use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_day_1_part_1() {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let mut number_string = String::new();
        let mut last_char = None;
        if let Ok(line) = line {
            for char in line.chars().filter(|c| c.is_digit(10)) {
                last_char = Some(char);
                if number_string.len() == 0 {
                    number_string.push(char);
                }
            }
            if let Some(lc) = last_char {
                number_string.push(lc);
            }
            let number_in_line = number_string.parse::<i32>().unwrap();
            result += number_in_line;
        }
    }

    println!("day 1 part 1: {}", result);
}