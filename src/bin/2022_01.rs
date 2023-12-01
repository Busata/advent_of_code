use std::{env};
use std::fs::{read_to_string};
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = read_to_string(file_path).unwrap();

    let mut calories: Vec<i32> = input
        .split("\r\n\r\n")
        .map(|elf_calories| elf_calories.split_whitespace().map(|x| x.parse::<i32>().unwrap()).sum())
        .collect();

    calories.sort();

    let topThree: i32 = calories.iter().rev().take(3).sum();

    println!("Top three: {}", topThree);


}

