use std::env;
use std::fs::read_to_string;
use memoize::memoize;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let lines = input.split("\r\n").collect::<Vec<&str>>();

    let total = lines.par_iter().enumerate().map(|(idx, line)| {
        let (spring_conditions, damaged_springs) = line.split_once(' ').unwrap();

        let adjusted_spring_conditions = std::iter::repeat(spring_conditions).take(5).collect::<Vec<&str>>().join("?");
        let adjusted_damaged_springs = std::iter::repeat(damaged_springs).take(5).collect::<Vec<&str>>().join(",");

        let damaged_spring_counts: Vec<usize> = adjusted_damaged_springs.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        generate_variation_count(adjusted_spring_conditions.chars().collect::<Vec<char>>(), damaged_spring_counts.clone())
    }).sum::<i64>();

    println!("Part 2 {}", total);
}

#[memoize]
fn generate_variation_count(springs: Vec<char>, group_sizes: Vec<usize>) -> i64 {
    if springs.is_empty() {
        if group_sizes.is_empty() {
            return 1;
        }
        return 0;
    }

    let first_char = springs[0];

    return match first_char {
        '.' => generate_variation_count(springs[1..].to_vec(), group_sizes),
        '?' => generate_variation_count(std::iter::once('.').chain(springs[1..].iter().cloned()).collect(), group_sizes.clone())
            + generate_variation_count(std::iter::once('#').chain(springs[1..].iter().cloned()).collect(), group_sizes.clone()),
        '#' => {
            if group_sizes.is_empty() {
                return 0;
            }

            let damaged_springs = group_sizes[0];

            if damaged_springs > springs.len() || !springs[..damaged_springs].iter().all(|x| *x == '#' || *x == '?') {
                return 0;
            }

            let new_groups = group_sizes[1..].to_vec();

            if damaged_springs == springs.len() {
                return if new_groups.is_empty() { 1 } else { 0 };
            } else if springs[damaged_springs] == '.'  || springs[damaged_springs] == '?' {
                return generate_variation_count(springs[damaged_springs + 1..].to_vec(), new_groups.clone());
            } else {
                return 0;
            }
        }
        _ => panic!("Unknown character encountered")
    };
}