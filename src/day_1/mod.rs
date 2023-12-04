use std::collections::HashMap;

use crate::file_loader;
use crate::runner;

pub fn get_numbers_map() -> std::collections::HashMap<&'static str, &'static str> {
    HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ])
}

pub fn run() {
    runner::run("day_1", solution, 281);
}

fn solution(file_name: &str) -> i32 {
    let numbers = get_numbers_map();
    let number_strings = numbers.keys();

    let rows = file_loader::get_lines(file_name);

    let mut answer: i32 = 0;

    for row in rows {
        let mut first: Option<String> = None;
        let mut last: Option<String> = None;
        let mut found_number_string = String::from("");

        let chars = row.chars();

        if chars.clone().count() == 0 {
            continue;
        }

        for char in chars {
            found_number_string.push(char);

            if char.is_numeric() {
                if first == None {
                    first = Some(char.to_string());
                }

                last = Some(char.to_string());

                found_number_string.clear();
                continue;
            }

            for key in number_strings.clone() {
                if !found_number_string.contains(key) {
                    continue;
                }

                let number_string = numbers.get(key)
                    .expect(&format!("Key missing {}", found_number_string));

                if first == None {
                    first = Some(number_string.to_string());
                }

                last = Some(number_string.to_string());

                while found_number_string.contains(key) {
                    found_number_string.remove(0);
                }
            }
        }

        let mut row_result = String::from("");

        match first {
            Some(val) => row_result.push_str(&val),
            None => ()
        }

        match last {
            Some(val) => row_result.push_str(&val),
            None => ()
        }

        let parsed = row_result.parse::<i32>();

        match parsed {
            Ok(val) => {
                let new_answer = answer + val;
                answer = new_answer
            },
            Err(e) => println!("{}", e)
        }
    }

    answer
}
