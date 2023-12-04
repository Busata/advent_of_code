use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = read_to_string(file_path).unwrap();

    let re = Regex::new(r"(\d+)|([^0-9])").unwrap();

    let mut part_numbers: Vec<i32> = Vec::new();
    let mut current_idx = 0;

    let engine_grid: Vec<Vec<String>> =  input.split("\r\n").fold(Vec::new(), |mut rows, line| {
        let mut row = Vec::new();

        for captures in re.captures_iter(line) {
            let part = captures.get(0).unwrap().as_str();

            match part.parse::<i32>() {
                Ok(number) => {
                    part_numbers.push(number);

                    for _ in 0..part.len() {
                        row.push(current_idx.to_string());
                    }

                    current_idx += 1;

                },
                Err(_) => {
                    row.push(part.to_string());
                }
            }
        }

        rows.push(row);
        rows
    });

    let mut schematic = EngineSchematic::new(engine_grid, part_numbers);

   let result =  schematic.calculate_part1();
   let result2 =  schematic.calculate_part2();

    println!("{} {}", result, result2);
}

struct EngineSchematic {
    part_numbers: Vec<i32>,
    grid: Vec<Vec<String>>,
    symbols: HashMap<i32, HashSet<String>>,
}

impl EngineSchematic {
    fn new(grid: Vec<Vec<String>>, part_numbers: Vec<i32>) -> Self {
        Self {
            grid,
            part_numbers,
            symbols: HashMap::new()
        }
    }

    fn calculate_part1(&mut self) -> i32 {
       let rows = self.grid.len();

        let mut unique_parts = Vec::new();

       for row_idx in 0..rows {
           let row = self.grid[row_idx].clone();
           for(col_idx, part) in row.iter().enumerate() {
               match part.parse::<usize>() {
                   Ok(_) => {

                   },
                   _ => {
                       if part == "." {
                           continue;
                       }

                      let numbers:Vec<i32> = self.find_part_number(row_idx, col_idx).iter().map(|idx| self.part_numbers[*idx as usize]).collect();

                       for x in numbers {
                           unique_parts.push(x);
                       }
                   }
               }


           }
       }

        unique_parts.iter().sum()
    }

    fn calculate_part2(&mut self) -> i32 {
       let rows = self.grid.len();

        let mut unique_parts = Vec::new();

       for row_idx in 0..rows {
           let row = self.grid[row_idx].clone();
           for(col_idx, part) in row.iter().enumerate() {
               match part.parse::<usize>() {
                   Ok(_) => {

                   },
                   _ => {
                       if part != "*" {
                           continue;
                       }

                      let numbers:Vec<i32> = self.find_part_number(row_idx, col_idx).iter().map(|idx| self.part_numbers[*idx as usize]).collect();

                       if numbers.len() != 2 {
                           continue;
                       }

                       unique_parts.push(numbers[0] * numbers[1]);
                   }
               }


           }
       }

        unique_parts.iter().sum()
    }

    fn find_part_number(&self, row_idx: usize, col_idx: usize) -> HashSet<i32> {
        let mut coordinates: Vec<(usize, usize)> = Vec::new();

        for i in -1i32..2i32 {
            for j in -1i32.. 2i32 {
                if i == 0 && j == 0 {
                    continue;
                }

                if (row_idx as i32 + i) >= 0 && (col_idx as i32 + j) >= 0 {
                    coordinates.push(((row_idx as i32 + i) as usize , (col_idx as i32 + j) as usize));
                }
            }
        }

        let mut symbols = Vec::new();

        for coordinate in coordinates {
            if coordinate.0 >= self.grid.len() {
                continue
            }

            let row = self.grid[coordinate.0].clone();

            if coordinate.1 >= row.len() {
                continue;
            }

            let element = row[coordinate.1].clone();
            symbols.push(element);
        }

        symbols.into_iter().filter_map(|x| x.parse::<i32>().ok()).collect()

    }
}