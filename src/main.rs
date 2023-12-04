pub mod file_loader;

use regex::Regex;

struct Card {
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>,
}

fn main() {
    assert!(run("input-test") == 13);

    let result = run("input");

    println!("Result: {result}");
}

fn run(filename: &str) -> u32 {
    let lines = file_loader::get_lines(filename);
    let mut result = 0;

    let cards = build_cards(lines);

    for card in cards {
        result += calculate_card(card);
    }

    return result;
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
