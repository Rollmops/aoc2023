use std::collections::HashMap;
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

pub fn solve_day_1_part_2() {
    let digit_names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut digit_map: HashMap<String, i32> = HashMap::new();
    for (index, name) in digit_names.iter().enumerate() {
        let number = (index as i32) + 1;
        digit_map.insert(name.to_string(), number);
        digit_map.insert(number.to_string(), number);
    }

    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);



    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut first_number = 0;
            let mut last_number = 0;
            let mut first_index = line.len() ;
            let mut last_index = 0;
            for (digit_name, number) in &digit_map {
                if let Some(first) = line.find(digit_name) {
                    if first < first_index {
                        first_index = first;
                        first_number = *number;
                    }
                };
                if let Some(last) = line.rfind(digit_name) {
                    if last >= last_index {
                        last_index = last;
                        last_number = *number;
                    }
                }
            }

            if first_number == 0 { first_number = last_number; }
            if last_number == 0 { last_number = first_number; }

            let number_in_line = format!("{}{}", first_number, last_number).parse::<i32>().unwrap();

            result += number_in_line;
        }
    }

    println!("day 1 part 2: {}", result);
}