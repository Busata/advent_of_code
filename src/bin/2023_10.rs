use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::read_to_string;

static GRID_SIZE_COLS: usize = 11;
static GRID_SIZE_ROWS: usize = 9;


struct BFS {
    queue: VecDeque<(usize, usize)>,
    visited: Vec<Vec<bool>>,
    steps: Vec<Vec<i32>>,
}

impl BFS {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            visited: vec![vec![false; cols]; rows],
            steps: vec![vec![-1; cols]; rows],
        }
    }

    fn run(&mut self, start_tile: (usize, usize), grid: &Vec<Vec<char>>) {
        self.queue.push_back(start_tile);
        self.visited[start_tile.0][start_tile.1] = true;
        self.steps[start_tile.0][start_tile.1] = 0;

        let directions: Vec<(i32, i32)> = vec!((0, 1), (0, -1), (-1, 0), (1, 0));

        while !self.queue.is_empty() {
            let cell = self.queue.pop_front().unwrap();

            let cell_symbol = grid[cell.0][cell.1];

            for dir in &directions {
                if !self.can_move_into_direction(cell_symbol, dir) {
                    continue;
                }

                let target_cell: (i32, i32) = (cell.0 as i32 + dir.0, cell.1 as i32 + dir.1);

                if self.is_valid_move(target_cell, dir, &self.visited, &grid) {
                    self.queue.push_back((target_cell.0 as usize, target_cell.1 as usize));
                    self.visited[target_cell.0 as usize][target_cell.1 as usize] = true;
                    self.steps[target_cell.0 as usize][target_cell.1 as usize] = self.steps[cell.0][cell.1] + 1;
                } else {}
            }
        }
    }


    fn can_move_into_direction(&self, symbol: char, dir: &(i32, i32)) -> bool {
        match symbol {
            '-' => match dir {
                (0, 1) => true,
                (0, -1) => true,
                _ => false
            },
            '|' => match dir {
                (-1, 0) => true,
                (1, 0) => true,
                _ => false
            },
            'J' => match dir { //connecting (-1,0) and (0,-1).
                (-1, 0) => true,
                (0, -1) => true,
                _ => false
            },
            'F' => match dir { //connecting (1,0) and (0,1).
                (1, 0) => true,
                (0, 1) => true,
                _ => false
            },
            'L' => match dir { //connecting (-1,0) and (0,1).
                (-1, 0) => true,
                (0, 1) => true,
                _ => false
            },
            '7' => match dir { //connecting (1,0) and (0,-1).
                (1, 0) => true,
                (0, -1) => true,
                _ => false
            },
            'S' => true,
            _ => false
        }
    }


    fn is_valid_move(&self, target_cell: (i32, i32), dir: &(i32, i32), visited: &Vec<Vec<bool>>, grid: &Vec<Vec<char>>) -> bool {
        if target_cell.0 < 0 || target_cell.0 >= GRID_SIZE_ROWS as i32 || target_cell.1 < 0 || target_cell.1 >= GRID_SIZE_COLS as i32 {
            return false;
        }

        if visited[target_cell.0 as usize][target_cell.1 as usize] == true {
            return false;
        }

        let target_symbol = grid[target_cell.0 as usize][target_cell.1 as usize];

        if target_symbol == '.' {
            return false;
        }

        //Can these symbols be entered from a given direction.
        let valid_symbol_move = match target_symbol {
            '-' => match dir {
                (0, 1) => true,
                (0, -1) => true,
                _ => false
            },
            '|' => match dir {
                (-1, 0) => true,
                (1, 0) => true,
                _ => false
            },
            'J' => match dir { //connecting (-1,0) and (0,-1).
                (1, 0) => true,
                (0, 1) => true,
                _ => false
            },
            'F' => match dir { //connecting (1,0) and (0,1).
                (-1, 0) => true,
                (0, -1) => true,
                _ => false
            },
            'L' => match dir { //connecting (-1,0) and (0,1).
                (1, 0) => true,
                (0, -1) => true,
                _ => false
            },
            '7' => match dir { //connecting (1,0) and (0,-1).
                (-1, 0) => true,
                (0, 1) => true,
                _ => false
            },
            _ => false
        };

        valid_symbol_move
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; GRID_SIZE_COLS]; GRID_SIZE_ROWS];

    for (row, line) in input.split("\r\n").enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = c;
        }
    }

    let starting_tile = find_starting_position(&grid);
    println!("Starting tile = {:?}", starting_tile);

    let mut bfs = BFS::new(GRID_SIZE_ROWS, GRID_SIZE_COLS);
    bfs.run(starting_tile, &grid);


    let result = bfs.steps.iter().flat_map(|x| x).max().unwrap();
    println!("Part 1: {}", result);

    let mut surrounded: Vec<(usize, usize)> = Vec::new();

    for col in 0..GRID_SIZE_COLS {
        let mut intersections_east = 0;
        let mut intersections_west = 0;

        for row in 0..GRID_SIZE_ROWS {
            let element = grid[row][col];
            let loop_element = bfs.steps[row][col] >= 0;

            if loop_element {
                if element == '-' || element == 'L' || element =='F' || element=='S' {
                    intersections_east += 1;
                }
                if element == '-' || element == '7' || element == 'J'|| element=='S' {
                    intersections_west += 1;
                }

            } else {
                if intersections_east.min(intersections_west) & 1 == 1 {
                    surrounded.push((row, col));
                }
            }
        }
    }

    println!("Part 2: {}", surrounded.len());

    let display: HashMap<&str, &str> = HashMap::from([("F", "┌"), ("L", "└"), ("7", "┐"), ("J", "┘"), ("|", "│"), ("-", "─"), ("S", "S"), (".", " ")]);

    for (row, row_elements) in bfs.steps.iter().enumerate() {
        for (col, col_elements) in row_elements.iter().enumerate() {
            if *col_elements >= 0 {
                print!("\x1b[93m{}\x1b[0m", display.get(grid[row][col].to_string().as_str()).unwrap());
            } else if surrounded.contains(&(row, col)) {
                print!("i");
            } else {
                print!("o");
            }
        }
        println!();
    }
}


fn find_starting_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (row, sub_list) in grid.iter().enumerate() {
        for (col, c) in sub_list.iter().enumerate() {
            if *c == 'S' {
                return (row, col);
            }
        }
    }

    (0, 0)
}