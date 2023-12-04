use regex::Regex;
use crate::file_loader;
use crate::runner;

mod game;

// To improve this we should avoid duplicate code for the three colors

pub fn run() {
    runner::run("day_2", run_pt1, 8);
    runner::run("day_2", run_pt2, 2286);
}

fn run_pt1(file_name: &str) -> i32 {
    let lines = file_loader::get_lines(file_name);

    let mut total = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let game = build_game(&line);

        let id = game.id;

        if game.is_valid() {
            total += id;
        }
    }

    total
}

fn run_pt2(file_name: &str) -> i32 {
    let lines = file_loader::get_lines(file_name);

    let mut total = 0;
    
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let game = build_game(&line);

        let power = game.max_red * game.max_blue * game.max_green;

        total += power;
    }

    total
}

fn build_game(line: &str) -> game::Game {
    let id_regex = Regex::new(r"Game (?<id>\d+):").unwrap();
    
    let Some(captures) = id_regex.captures(line) else {
        panic!("No match for line {}", line);
    };

    let id: i32 = captures["id"].parse().unwrap();

    let trimmed = line.split(": ")
        .last()
        .unwrap();

    let rounds: Vec<String> = trimmed.split("; ")
        .map(|s| s.to_string())
        .collect();

    let mut game = game::Game {
        id,
        rounds: Vec::new(),
        max_red: 0,
        max_green: 0,
        max_blue: 0,
    };

    for round_string in rounds {
        let round = build_round(&round_string);

        if round.red > game.max_red {
            game.max_red = round.red;
        }

        if round.green > game.max_green {
            game.max_green = round.green;
        }

        if round.blue > game.max_blue {
            game.max_blue = round.blue;
        }

        game.rounds.push(round);
    }

    game
}

fn build_round(line: &str) -> game::Round {
    let parts: Vec<String> = line.split(", ")
        .map(|s| s.to_string())
        .collect();

    let mut round = game::Round {
        red: 0,
        green: 0,
        blue: 0,
    };

    for part in parts {
        let count = build_count(&part);

        if count.0 == "red" {
            round.red += count.1;
        } else if count.0 == "green" {
            round.green += count.1;
        } else if count.0 == "blue" {
            round.blue += count.1;
        }
    }

    round
}

fn build_count(line: &str) -> (String, i32) {
    let count_regex = Regex::new(r"(?<count>\d+) (?<color>([a-z]+))").unwrap();

    let Some(captures) = count_regex.captures(line) else {
        panic!("No count found for line {}", line);
    };

    let color: String = captures["color"].parse().unwrap();
    let count: i32 = captures["count"].parse().unwrap();

    (color, count)
}
