use std::collections::{HashMap};
use std::env;
use std::fs::read_to_string;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = read_to_string(file_path).unwrap();


    let mut mappings: HashMap<&str, SeedMapper> = HashMap::new();
    let mut seeds: Vec<u64> = Vec::new();

    let mut last_source_key = "";


    input.split("\r\n").map(|line| line.trim()).filter(|line| !line.is_empty())
        .for_each(|line| {
            let characters: Vec<char> = line.chars().collect();

            if line.starts_with("seeds") {
                seeds = line.split_once(":").unwrap().1.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
            } else if !characters[0].is_digit(10) {
                let (source, target) = line.trim().split_once("-to-").unwrap();
                let target = target.replace(" map:", "");

                mappings.insert(source, SeedMapper {
                    target_key: target,
                    ranges: Vec::new(),
                });
                last_source_key = source;
            } else {
                let numbers: Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
                let mapping = mappings.get_mut(last_source_key).unwrap();

                mapping.add_range(numbers[1], numbers[0], numbers[2]);
            }
        });

    let lowest_location: u64 = seeds.iter().map(|seed| {
        find_location(&mappings, *seed)
    }).min().unwrap();

    println!("Part 1: {}", lowest_location);

    let mut ranges: Vec<(u64, u64)> = seeds.chunks(2).map(|pair| (pair[0], pair[0] + pair[1])).collect();

    println!("{:?}", ranges);

    let lowest_location: Vec<u64> = ranges.par_iter().flat_map(|range| {
        range.0..range.1
    }).map(|seed| {
        find_location(&mappings, seed)
    }).collect();

    println!("Part 2: {}", lowest_location.iter().min().unwrap());
}

fn find_location(mappings: &HashMap<&str, SeedMapper>, seed: u64) -> u64 {
    let mut next_map = "seed";
    let mut next_number = seed;

    while next_map != "location" {
        let seed_mapper = mappings.get(next_map).unwrap();

        next_map = &seed_mapper.target_key;
        next_number = seed_mapper.find_number(&next_number);
    };

    next_number
}

#[derive(Debug)]
struct SeedMapper {
    target_key: String,
    ranges: Vec<SeedRange>,
}

impl SeedMapper {
    fn find_number(&self, number: &u64) -> u64 {
        self.ranges.iter().find(|range| range.contains(*number)).map_or(*number, |range| {
            range.calculate(*number)
        })
    }

    fn add_range(&mut self, source: u64, destination: u64, range: u64) {
        self.ranges.push(SeedRange {
            source,
            destination,
            range,
        })
    }
}

#[derive(Debug)]
struct SeedRange {
    source: u64,
    destination: u64,
    range: u64,
}

impl SeedRange {
    fn contains(&self, number: u64) -> bool {
        (number >= self.source) && (number <= (self.source + self.range - 1))
    }
    fn calculate(&self, number: u64) -> u64 {
        let diff = number - self.source;
        self.destination + diff
    }
}