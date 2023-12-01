use std::{env};
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let score:i32 = read_to_string(file_path).unwrap().split("\n").map(|x| {
        if let Some((elf_input, player_input)) = x.trim().split_once(" ") {
            calculate_score_part_2(elf_input, player_input)
        } else {
            0
        }
    }).sum();

    println!("Total score: {}", score);

}

fn calculate_score_part_1(elf_input: &str, player_input: &str) -> i32 {
    let elf_pick = match elf_input {
        "A" => "rock",
        "B" => "paper",
        "C" => "scissor",
        _ => ""
    };

    let player_pick = match player_input {
        "A" => "rock",
        "B" => "paper",
        "C" => "scissor",
        _ => ""
    };

    calculate_match_score(elf_pick, player_pick)
}


fn calculate_score_part_2(elf_input: &str, player_input: &str) -> i32 {
    let elf_pick = match elf_input {
        "A" => "rock",
        "B" => "paper",
        "C" => "scissor",
        _ => ""
    };

    let player_pick = match player_input {
        "X" => match elf_pick { // LOSE
            "rock" => "scissor",
            "paper" => "rock",
            "scissor" => "paper",
            _ => ""
        },
        "Y" => elf_pick,
        "Z" => match elf_pick { // WIN
            "rock" => "paper",
            "paper" => "scissor",
            "scissor" => "rock",
            _ => ""
        }
        _ => ""
    };

    calculate_match_score(elf_pick, player_pick)
}

fn calculate_match_score(elf_pick: &str, player_pick: &str) -> i32 {
    let base_score = match player_pick {
        "rock" => 1, //Rock
        "paper" => 2, //Paper
        "scissor" => 3, //Scissor
        &_ => 0
    };

    let match_score = if player_pick == "rock" && elf_pick == "scissor" {
        6
    } else if player_pick == "paper" && elf_pick == "rock" {
        6
    } else if player_pick == "scissor" && elf_pick == "paper" {
        6
    } else if player_pick == elf_pick {
        3
    } else {
        0
    };

    base_score + match_score
}