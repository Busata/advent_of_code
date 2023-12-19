use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

mod helpers;

use helpers::Direction;
use crate::helpers::{add_direction, Grid2D};

#[derive(PartialEq, Eq)]
enum GridType {
    PASSED,
    ACTIVATED,
    UNTOUCHED,
}



fn main() {
    let file_path = "input/2023_16.txt";
    let input = read_to_string(file_path).unwrap();

    let grid = Grid2D::<char>::from_string(input);

    let mut beams_to_check: Vec<Beam> = Vec::new();

    //top
    for x in 0..grid.width() {
        beams_to_check.push(Beam::new((-1, x as i32), Direction::Down));
    }
    //bottom
    for x in 0..grid.width() {
        beams_to_check.push(Beam::new((grid.height(), x), Direction::Up));
    }

    //left
    for x in 0..grid.height() {
        beams_to_check.push(Beam::new((x as i32, -1), Direction::Right));
    }

    //right
    for x in 0..grid.height() {
        beams_to_check.push(Beam::new((grid.width(), x), Direction::Left));
    }


    let part1: usize = calculate_energized_fields(&grid, Beam::new((0, -1), Direction::Right));

    let part2: usize = beams_to_check.iter().map(|&beam| {
        calculate_energized_fields(&grid, beam)
    }).max().unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn calculate_energized_fields(mirror_grid: &Grid2D<char>, beam: Beam) -> usize {
    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(beam.clone());
    let mut visited_cells: HashMap<(i32, i32), GridType> = HashMap::new();

    while !beams.is_empty() {
        let mut active_beam = beams.pop_front().unwrap();

        while !active_beam.done {
            let target_position = add_direction(active_beam.position, active_beam.direction);

            if !mirror_grid.is_within_bounds(&target_position) {
                active_beam.done = true;
                break;
            }

            active_beam.position = target_position;

            let next_cell = &mirror_grid.get_cell(target_position.0,target_position.1);
            let cell_type = visited_cells.get(&target_position).unwrap_or(&GridType::UNTOUCHED);

            match next_cell {
                '|' => {
                    if active_beam.direction.value().1.abs() == 1 {
                        if *cell_type != GridType::ACTIVATED {
                            active_beam.direction = Direction::Down;
                            beams.push_back(Beam::new(target_position, Direction::Up));
                            visited_cells.insert(target_position, GridType::ACTIVATED);
                        } else {
                            active_beam.done = true;
                        }
                    } else {
                        visited_cells.insert(target_position, GridType::PASSED);
                    }
                }
                '-' => {
                    if active_beam.direction.value().0.abs() == 1 {
                        if *cell_type != GridType::ACTIVATED {
                            active_beam.direction = Direction::Right;
                            beams.push_back(Beam::new(target_position, Direction::Left));
                            visited_cells.insert(target_position, GridType::ACTIVATED);
                        } else {
                            active_beam.done = true;
                        }
                    } else {
                        visited_cells.insert(target_position, GridType::PASSED);
                    }
                }
                '/' | '\\' => {
                    let direction = determine_direction(*next_cell, active_beam.direction);
                    active_beam.direction = direction;
                    visited_cells.insert(target_position, GridType::PASSED);
                }
                _ => {
                    visited_cells.insert(target_position, GridType::PASSED);
                }
            }
        }
    }

    visited_cells.values().filter(|&x| *x != GridType::UNTOUCHED).count()
}

fn determine_direction(symbol: char, direction: Direction) -> Direction {
    match symbol {
        '/' => match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
        '\\' => match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
        _ => direction.clone()
    }
}


fn is_within_grid(position: &(i32, i32), grid: &Vec<Vec<char>>) -> bool {
    if position.0 >= 0 && position.0 < grid.len() as i32 &&
        position.1 >= 0 && position.1 < grid[0].len() as i32 {
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    position: (i32, i32),
    direction: Direction,
    done: bool,
}

impl Beam {
    fn new(position: (i32, i32), direction: Direction) -> Self {
        Self {
            position,
            direction,
            done: false,
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>, x1: &HashMap<(i32, i32), GridType>) {
    for (row_idx, row) in grid.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            let position = (row_idx as i32, col_idx as i32);
            if x1.contains_key(&position) {
                print!("#");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
    println!();
}