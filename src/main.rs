pub mod file_loader;

use regex::Regex;

struct Card {
    count: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>,
}

fn main() {
    run(run_pt1, 13);
    run(run_pt2, 30);
}

fn run(function: fn(&str) -> u32, expected_result: u32) {
    let test_result = function("input-test");

    println!("Test result: {test_result}");

    assert!(test_result == expected_result);

    let result = function("input");

    println!("Result: {result}");
}

fn run_pt1(filename: &str) -> u32 {
    let lines = file_loader::get_lines(filename);
    let mut result = 0;

    let cards = build_cards(lines);

    for card in cards {
        result += calculate_card(card);
    }

    return result;
}

fn run_pt2(filename: &str) -> u32 {
    let lines = file_loader::get_lines(filename);

    let mut cards = build_cards(lines);

    for i in 0..cards.len() - 1 {
        let card = cards.get(i).unwrap();

        let card_count = card.count;
        let winning_numbers = count_winning_numbers(card);

        let end_index = if i + winning_numbers as usize > cards.len() {
            cards.len()
        } else {
            i + winning_numbers as usize
        };

        for j in i + 1..end_index + 1 {
            *&mut cards[j].count += card_count;
        }

    }

    let mut result = 0;

    for card in cards {
        result += card.count;
    }

    result
}

fn build_cards(lines: Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let mut parts = line
            .split(":")
            .last()
            .unwrap()
            .split("|");

        let first_part = parts.next().unwrap();
        let second_part = parts.next().unwrap();

        cards.push(Card {
            count: 1,
            winning_numbers: get_numbers(first_part),
            actual_numbers: get_numbers(second_part),
        });
    }

    cards
}

fn calculate_card(card: Card) -> u32 {
    let mut result = 0;

    for number in card.actual_numbers {
        if !card.winning_numbers.contains(&number) {
            continue;
        }

        if result == 0 {
            result = 1;
            continue;
        }

        result *= 2;
    }

    result
}

fn count_winning_numbers(card: &Card) -> u32 {
    let mut result = 0;

    for number in card.actual_numbers.as_slice() {
        if card.winning_numbers.contains(&number) {
            result += 1;
        }
    }

    result
}

fn get_numbers(line: &str) -> Vec<u32> {
    let re = Regex::new(r"(?<num>[\d]+)").unwrap();

    match re.captures_iter(line)
        .map(|c| c.name("num").unwrap().as_str().parse())
        .collect() {
            Err(e) => {
                println!("{}", e);
                Vec::new()
            },
            Ok(val) => val,
        }
}
