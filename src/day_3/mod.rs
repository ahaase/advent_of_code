use crate::file_loader;
use crate::runner;

pub fn run() {
    runner::run("day_3", run_pt1, 4361);
    runner::run("day_3", run_pt2, 467835);
}

fn run_pt2(file_name: &str) -> i32 {
    let mut total_result = 0;

    let lines = file_loader::get_lines(file_name);

    for i in 0..lines.len() {
        let current = lines
            .get(i)
            .unwrap()
            .char_indices();

        for (x_pos, char) in current {
            if char != '*' {
                continue;
            }

            let numbers = get_numbers_surrounding_point(&lines, x_pos, i);

            let res = multiply_array(numbers);

            total_result += res;
        }
    }

    total_result
}

fn run_pt1(file_name: &str) -> i32 {
    let mut total_result = 0;

    let lines = file_loader::get_lines(file_name);

    for i in 0..lines.len() {
        let current = lines[i].to_owned();

        let mut found_number = false;
        let mut number_string = String::from("");
        let mut x_start: usize = 0;

        for (x_pos, char) in current.char_indices() {
            if char.is_numeric() {
                number_string.push(char);

                if !found_number {
                    found_number = true;
                    x_start = get_x_start(x_pos);
                }

                // Do not continue if we're at end of line
                if x_pos != current.len() - 1 {
                    continue;
                }
            }

            if !found_number {
                continue;
            }

            let number: i32 = number_string
                .parse()
                .unwrap();

            let x_end = get_x_end(&current, x_pos);

            let surroundings = get_surroundings_flat(&lines, i, x_start, x_end);

            let without_dots = surroundings.replace(".", "");

            if without_dots != number_string {
                total_result += number;
            }

            // Reset
            x_start = 0;
            number_string = String::from("");
            found_number = false;
        }
    }

    total_result
}

fn get_surroundings_flat(lines: &Vec<String>, index: usize, x_start: usize, x_end: usize) -> String {
    let mut surroundings = String::from("");

    for line in get_lines(lines, index) {
        match line {
            Some(ln) => surroundings.push_str(&ln[x_start..x_end]),
            None => ()
        }
    }

    surroundings
}

fn get_x_start(pos: usize) -> usize {
    if pos > 0 {
        pos - 1
    } else {
        pos
    }
}

fn get_x_end(line: &String, pos: usize) -> usize {
    if pos < line.len() {
        pos + 1
    } else {
        pos
    }
}

fn get_lines(lines: &Vec<String>, index: usize) -> [Option<String>; 3] {
    let top = if index > 0 {
        lines.get(index - 1)
    } else {
        None
    };

    let current = lines.get(index);

    let below = if index < lines.len() {
        lines.get(index + 1)
    } else {
        None
    };

    [top.cloned(), current.cloned(), below.cloned()]
}

fn get_numbers_in_range(line: Option<String>, x_start: usize, x_end: usize) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    match line {
        Some(ln) => {
            let chars: Vec<char> = ln.chars().collect();
            let mut found_number = false;

            for index in x_start..x_end + 1 {
                if !chars[index].is_numeric() {
                    found_number = false;

                    continue;
                }

                if !found_number {
                    numbers.push(get_number_at_index(&chars, index))
                }

                found_number = true;
            }

            numbers
        },
        None => numbers,
    }
}

fn get_number_at_index(line: &Vec<char>, index: usize) -> i32 {
    let mut start = index;
    let mut end = index;

    while start > 0 && line.get(start - 1).expect("Could not read char").is_numeric() {
        start -= 1;
    }

    while end < line.len() - 1 && line.get(end + 1).expect("Could not read char").is_numeric() {
        end += 1;
    }

    line[start..end + 1]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn multiply_array(numbers: Vec<i32>) -> i32 {
    let mut res = 0;

    if numbers.len() <= 1 {
        return res;
    }

    for number in numbers {
        if res == 0 {
            res = number;
            continue;
        }

        res *= number;
    }

    res
}

fn get_numbers_surrounding_point(lines: &Vec<String>, x: usize, y: usize) -> Vec<i32> {
    let current = lines
        .get(y)
        .unwrap();

    let x_start = get_x_start(x);
    let x_end = get_x_end(&current, x);

    let mut numbers: Vec<i32> = Vec::new();

    for line in get_lines(&lines, y) {
        numbers.append(&mut get_numbers_in_range(line, x_start, x_end));
    }

    numbers
}
