use std::env;
use std::fs::read_to_string;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let histories :Vec<Vec<Vec<i64>>> = input.split("\r\n").map(|history_line| {
        let mut differences: Vec<Vec<i64>> = vec!(
            history_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
        );

        while !last_line_all_zeroes(&mut differences) {
            let difference: Vec<i64> = differences.last().unwrap().iter()
                .zip(differences.last().unwrap().iter().skip(1))
                .map(|pair| pair.1 - pair.0)
                .collect();
            differences.push(difference);
        }
        differences
    }).collect();

    let part1 = histories.iter().map(|differences| {
        differences.iter()
            .map(|x| x.last().unwrap())
            .sum::<i64>()
    }).sum::<i64>();

    println!("Part 1: {:?}", part1);

    let part2 = histories.iter().map(|differences| {
        differences.iter()
            .rev()
            .map(|x| x.first().unwrap())
            .fold(0, |sum, value| {
                *value - sum
            })
    }).sum::<i64>();
    println!("Part 2: {:?}", part2);
}

fn last_line_all_zeroes(differences: &mut Vec<Vec<i64>>) -> bool {
    differences.last().unwrap().iter().all(|x| *x == 0)
}