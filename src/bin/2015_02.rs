use std::cmp::min;
use std::fs::read_to_string;

fn main() {
    let file_path = "input/2015_02.txt";

    let input = read_to_string(file_path).unwrap();

    let part1 = input.split_ascii_whitespace().fold(0, |acc, x| {
        let measurements = x.split("x").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let (l, w, h) = match &measurements[..] {
            &[first, second, third, ..] => (first, second, third),
            _ => unreachable!()
        };

        acc + ((2 * l * w) + (2 * w * h) + (2 * h * l)) + min(w * h, min((l * w), l * h))
    });





    println!("Part 1: {}", part1);
}
