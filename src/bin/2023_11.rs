use std::collections::{HashSet};
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

    expand_grid(rows[0].len(), &mut grid);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut unique_pairs = HashSet::new();

    for (i, &item1) in galaxies.iter().enumerate() {
        for &item2 in galaxies.iter().skip(i + 1) {
            unique_pairs.insert((item1, item2));
        }
    }

    println!("Calculating distances for {} pairs", unique_pairs.len());

    let result: i64 = unique_pairs.par_iter().map(|(start_tile, goal_tile)| {
        let steps = (start_tile.0 as i64 - goal_tile.0 as i64).abs() + (start_tile.1 as i64 - goal_tile.1 as i64).abs();

        steps
    }).sum();

    println!("Part 2: {}", result);
}

fn expand_grid(column_size: usize, grid: &mut Vec<Vec<char>>) {
    let insert_count = 1000000 - 1;

    let mut empty_columns: Vec<usize> = Vec::new();

    for col_idx in 0..column_size {
        if grid.iter().map(|row| row[col_idx]).all(|x| x == '.') {
            empty_columns.push(col_idx);
        }
    }

    println!("Replacing {} columns", empty_columns.len());
    for idx in empty_columns.iter().rev() {
        println!("Replacing col {}", idx);
        for row in grid.iter_mut() {
            row.splice(idx..idx, vec!['.'; insert_count]);
        }
    }

    let mut empty_rows: Vec<usize> = Vec::new();
    for (idx, row) in grid.iter().enumerate() {
        if row.iter().all(|x| *x == '.') {
            empty_rows.push(idx);
        }
    }

    println!("Replacing {} rows", empty_rows.len());
    let empty_row = vec!('.'; column_size);

    for idx in empty_rows.iter().rev() {
        println!("Replacing row {}", idx);
        let clones = std::iter::repeat(empty_row.clone()).take(insert_count);
        grid.splice(idx..idx, clones);
    }



}

