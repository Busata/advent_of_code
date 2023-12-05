use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = read_to_string(file_path).unwrap();

    let scoring: HashMap<i32, i32> = input.split("\r\n").map(|card_line| {
        let (card_info, number_set) = card_line.split_once(":").unwrap();

        let card_number = card_info.trim().split_once(" ").unwrap().1.trim().parse::<i32>().unwrap();

        let (winning_numbers_part, own_numbers_part) = number_set.split_once('|').unwrap();

        let winning_numbers: Vec<i32> = parse_numbers_line(winning_numbers_part);
        let own_numbers: Vec<i32> = parse_numbers_line(own_numbers_part);

        let matching = own_numbers.iter().fold(0, |matching_numbers, next_number| {
            if !winning_numbers.contains(next_number) {
                return matching_numbers;
            }

            matching_numbers + 1
        });

        (card_number, matching)
    }).collect();

    let part_1_score: i32 = scoring.values().filter(|x| **x > 0).map(|matching| 2_i32.pow((matching - 1) as u32)).sum();
    println!("Result pt1: {}", part_1_score);

    let part_2_score: i32 = scoring.keys().map(|card_number| {
        find_total_scratch_cards(*card_number, &scoring)
    }).sum::<i32>();
    println!("Result pt2: {}", part_2_score);
}

fn find_total_scratch_cards(card_number: i32, scoring: &HashMap<i32, i32>) -> i32 {
    let matches = *scoring.get(&card_number).unwrap();

    ((card_number + 1)..(card_number + matches + 1))
        .map(|card_number| find_total_scratch_cards(card_number, scoring))
        .sum::<i32>() + 1 // The card itself
}

fn parse_numbers_line(winning_numbers: &str) -> Vec<i32> {
    winning_numbers
        .trim()
        .split_whitespace()
        .filter(|str| !str.is_empty())
        .map(|str| str.trim().parse::<i32>().unwrap()).collect()
}