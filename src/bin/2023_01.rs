use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

// https://adventofcode.com/2023/day/1

/*
The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

Some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

 */
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let result: i32 = lines.filter_map(|x| x.ok().and_then(parse_number)).sum();
        println!("{}", result);
    }
}

fn parse_number(input: String) -> Option<i32> {
    let only_digits: Vec<char> = input.clone()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();

    let first_number = only_digits.first()?;
    let last_number = only_digits.last()?;

    let combined = format!("{}{}", first_number, last_number);

    combined.parse::<i32>().ok()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
