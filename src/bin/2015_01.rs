use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = read_to_string(file_path).unwrap();


    let mut floor = 0;
    let mut basement_reached = false;
    for (idx, char) in  input.chars().enumerate() {

        floor += get_floor_change(char);

        if !basement_reached &&  floor == -1 {
            println!("Part 2: {}", idx + 1);
            basement_reached = true;
        }
    }

    println!("Part 1: {}", floor);

}

fn get_floor_change(instruction: char) -> i32 {
    let floor_change = match instruction {
        '(' => 1,
        ')' => -1,
        _ => 0
    };
    floor_change
}