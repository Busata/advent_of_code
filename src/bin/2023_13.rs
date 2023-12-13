use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let lines = input.split("\r\n").collect::<Vec<&str>>();
    let mirrors: Vec<_> = lines.split(|&x| x == "").collect();

    let total = mirrors.iter().flat_map(|&mirror| {
        let rows = mirror.to_vec().iter().map(|v| v.to_string()).collect();

        let columns = (0..mirror[0].len()).map(|x| {
            return mirror.iter().map(|&l| l.chars().nth(x).unwrap()).collect::<String>();
        }).collect::<Vec<_>>();

        [(rows, columns)]
    }).map(|(rows, columns)| {
        let original_row_indices = find_reflection_idx(&rows);
        let original_column_indices = find_reflection_idx(&columns);

        let mut row_smudged_indices = HashSet::new();

        for row_mirror in find_smudge(&rows) {
            row_smudged_indices.extend(find_reflection_idx(&row_mirror));
        }

        for x in original_row_indices {
            row_smudged_indices.remove(&x);
        }

        let mut column_smudged_indices = HashSet::new();
        for column_mirror in find_smudge(&columns) {
            column_smudged_indices.extend(find_reflection_idx(&column_mirror));
        }

        for x in original_column_indices {
            column_smudged_indices.remove(&x);
        }

        let mut acc = 0;

        for x in column_smudged_indices {
            acc += (x + 1);
        }

        for x in row_smudged_indices {
            acc += (x + 1) * 100;
        }

        acc as i32

    }).sum::<i32>();

    println!("Part 2: {}", total);
}

fn find_smudge(mirror: &Vec<String>) -> Vec<Vec<String>> {
    let mut mirrors_without_smudge: Vec<Vec<String>> = Vec::new();
    for (start_idx, start) in mirror.iter().enumerate() {
        for (end_idx, end) in mirror.iter().enumerate() {
            if has_different_characters(start, end, 1) {
                let mut updated_mirror = mirror.clone();
                updated_mirror[start_idx] = updated_mirror[end_idx].clone();

                mirrors_without_smudge.push(updated_mirror);
            }
        }
    }

    mirrors_without_smudge
}

fn has_different_characters(start: &String, end: &String, i: i32) -> bool {
    let mut count = 0;

    for (c1, c2) in start.chars().zip(end.chars()) {
        if c1 != c2 {
            count += 1;
        }
    }

    count == i
}

fn find_reflection_idx(rows: &Vec<String>) -> Vec<usize> {
    let mut reflection_indices: Vec<usize> = Vec::new();
    for (idx1, p1) in rows.iter().enumerate() {
        for (idx2, p2) in rows.iter().enumerate() {
            if p1.trim() == p2.trim() && ((idx1 + 1) == idx2) {
                let mut num1 = (0..=idx1).rev().collect::<Vec<usize>>();
                let mut num2 = (idx1 + 1..rows.len()).collect::<Vec<usize>>();
                if num1.len() > num2.len() {
                    std::mem::swap(&mut num1, &mut num2);
                }

                let reflecting = (num1.iter()).zip(num2.iter()).all(|(&i, &j)| {
                    rows[i] == rows[j]
                });

                if reflecting {
                    reflection_indices.push(idx1);
                } else {
                    continue;
                }
            }
        }
    }

    return reflection_indices;
}