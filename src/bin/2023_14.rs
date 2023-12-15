use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let rows = input.split("\r\n").collect::<Vec<&str>>();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; rows[0].len()]; rows.len()];

    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, col) in row.chars().enumerate() {
            grid[row_idx][col_idx] = col;
        }
    }

    let mut recurring: HashMap<i32, usize> = HashMap::new();

    let cycles = 1000000000;
    for x in 0..cycles {
        println!("Cycles left {}: ", cycles - x);

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        let calculate_beams = calculate_beams(&mut grid);
        recurring.insert(x,calculate_beams);

        println!("{:?}", recurring); // Value was
    }


    let calculate_beams = calculate_beams(&mut grid);

    //TODO find the pattern

    println!("Part 2: {}", calculate_beams);

}

fn calculate_beams(grid: &mut Vec<Vec<char>>) -> usize {
    let mut calculate_beams = 0;

    for (row_idx, row) in grid.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate().rev() {
            if *col == 'O' {
                calculate_beams += row.len() - row_idx;
            }
        }
    }
    calculate_beams
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for (row_idx, row) in grid.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '.' || *col == '#' {
                continue;
            }

            let mut current_row = row_idx;

            while can_move_north(current_row, col_idx, &grid) {
                grid[current_row][col_idx] = '.';
                current_row -= 1;
                grid[current_row][col_idx] = 'O';
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for (row_idx, row) in grid.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '.' || *col == '#' {
                continue;
            }

            let mut current_col = col_idx;

            while can_move_west(row_idx, current_col, &grid) {
                grid[row_idx][current_col] = '.';
                current_col -= 1;
                grid[row_idx][current_col] = 'O';
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    for (row_idx, row) in grid.clone().iter().enumerate().rev() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '.' || *col == '#' {
                continue;
            }

            let mut current_row = row_idx;

            while can_move_south(current_row, col_idx, &grid) {
                grid[current_row][col_idx] = '.';
                current_row += 1;
                grid[current_row][col_idx] = 'O';
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for (row_idx, row) in grid.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate().rev() {
            if *col == '.' || *col == '#' {
                continue;
            }

            let mut current_col = col_idx;

            while can_move_east(row_idx, current_col, &grid) {
                grid[row_idx][current_col] = '.';
                current_col += 1;
                grid[row_idx][current_col] = 'O';
            }
        }
    }
}

fn print_grid(grid: &mut Vec<Vec<char>>) {
    for (row_idx, row) in grid.clone().iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!();
    }
    println!();

}

fn can_move_north(row_idx: usize, col_idx: usize, grid: &Vec<Vec<char>>) -> bool {
    if row_idx >= 1 && grid[row_idx - 1][col_idx] == '.' {
        return true;
    } else {
        return false;
    }
}

fn can_move_south(row_idx: usize, col_idx: usize, grid: &Vec<Vec<char>>) -> bool {
    if row_idx < grid.len() - 1 && grid[row_idx + 1][col_idx] == '.' {
        return true;
    } else {
        return false;
    }
}

fn can_move_west(row_idx: usize, col_idx: usize, grid: &Vec<Vec<char>>) -> bool {
    if col_idx >= 1 && grid[row_idx][col_idx - 1] == '.' {
        return true;
    } else {
        return false;
    }
}

fn can_move_east(row_idx: usize, col_idx: usize, grid: &Vec<Vec<char>>) -> bool {
    if col_idx < grid[0].len() - 1 && grid[row_idx][col_idx + 1] == '.' {
        return true;
    } else {
        return false;
    }
}

