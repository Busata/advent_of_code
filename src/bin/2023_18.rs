use std::fs::read_to_string;
use std::future::Future;
use std::hash::{Hash, Hasher};
use crate::helpers::Direction;

mod helpers;

fn main() {
    let file_path = "input/2023_18.txt";
    let input = read_to_string(file_path).unwrap();

    let (points, total_meters) = calculate_part1(input.clone());

    let area_including_borders = calculate_area(points, total_meters);

    println!("Part 1: {:?}", area_including_borders);

    let (points, total_meters) = calculate_part2(input.clone());

    let area_including_borders = calculate_area(points, total_meters);

    println!("Part 2: {:?}", area_including_borders);

}

fn calculate_area(points: Vec<(i64, i64)>, total_meters: i64) -> i64 {
    let area = points.iter().zip(points.iter().skip(1)).fold(0, |acc, pair| {
        let first = pair.0;
        let second = pair.1;

        acc + ((first.0 * second.1) - (second.0 * first.1))
    }).abs() / 2;


    let area_including_borders = area + (total_meters / 2) + 1;
    area_including_borders
}


fn calculate_part1(input: String) -> (Vec<(i64, i64)>, i64) {
    let mut points: Vec<(i64, i64)> = Vec::new();

    let mut current_position = (0, 0);

    points.push(current_position);

    let mut total_meters = 0;


    input.split("\r\n").for_each(|instruction_line| {
        let parts = instruction_line.split(' ').collect::<Vec<&str>>();

        let direction = Direction::from_instruction(parts[0]);
        let meters = parts[1].parse::<i64>().unwrap();

        current_position = add_positions(current_position, direction.mul(meters));

        points.push(current_position);
        total_meters += meters;
    });

    (points, total_meters)
}

fn calculate_part2(input: String) -> (Vec<(i64, i64)>, i64) {
    let mut points: Vec<(i64, i64)> = Vec::new();

    let mut current_position = (0, 0);

    points.push(current_position);

    let mut total_meters = 0;


    input.split("\r\n").for_each(|instruction_line| {
        let parts = instruction_line.split(' ').collect::<Vec<&str>>();

        let instruction_part = parts[2];

        let meters_in_hex = &instruction_part[2..7];
        let direction_as_number = &instruction_part[7..8];

        let direction = Direction::from_instruction(direction_as_number);
        let meters = i64::from_str_radix(meters_in_hex, 16).unwrap();

        current_position = add_positions(current_position, direction.mul(meters));

        points.push(current_position);
        total_meters += meters;
    });
    (points, total_meters)
}

fn add_positions(value: (i64, i64), other: (i64, i64)) -> (i64, i64) {
    (value.0 + other.0, value.1 + other.1)
}