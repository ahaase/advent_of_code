use crate::file_loader;
use crate::runner;

pub fn run() {
    runner::run("day_3", run_pt1, 4361);
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

            let surroundings = get_surroundings(&lines, i, x_start, x_end);

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

fn get_surroundings(lines: &Vec<String>, index: usize, x_start: usize, x_end: usize) -> String {
    let mut surroundings = String::from("");

    if index > 0 {
        let line = lines
            .get(index - 1)
            .expect("Could not read above line");

        surroundings.push_str(&line[x_start..x_end]);
    }

    let line = lines
        .get(index)
        .expect("Could not read current line");

    let slice = &line[x_start..x_end];//.replace(&number_string, "");

    surroundings.push_str(slice);

    if index < lines.len() - 1 {
        let line = lines
            .get(index + 1)
            .expect("Could not read below line");

        surroundings.push_str(&line[x_start..x_end]);
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
