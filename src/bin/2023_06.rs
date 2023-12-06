use std::env;
use std::fs::read_to_string;
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let (time_line, distance_line) = input.split_once("\r\n").unwrap();

    let times: Vec<i64> = parse_line(time_line);
    let distances:  Vec<i64> = parse_line(distance_line);

    let part_1: i64 = zip(times.clone(), distances.clone()).fold(1, |times_beaten: i64, (time, distance)| {
        times_beaten * calculate_ways_to_beat(time, distance)
    });

    println!("Part 1: {}", part_1);

    let concatenated_time = join_numbers(times);
    let concatenated_distance = join_numbers(distances);

    let part_2 = calculate_ways_to_beat(concatenated_time, concatenated_distance);

    println!("Part 2: {}", part_2);
}

fn join_numbers(distances: Vec<i64>) -> i64 {
    distances.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap()
}

fn calculate_ways_to_beat(time: i64, distance: i64) -> i64 {
    (0..time + 1).filter(|time_held| (time - time_held) * time_held > distance).count() as i64
}

fn parse_line(distance_line: &str) -> Vec<i64> {
    let (_, distance_part) = distance_line.split_once(":").unwrap();
    let distances: Vec<i64> = distance_part.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
    distances
}