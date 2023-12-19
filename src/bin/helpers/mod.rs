use rayon::iter::Positions;

#[derive(Debug)]
pub struct Grid2D {
    pub data: Vec<Vec<char>>,
}


impl Grid2D {
    pub fn from_string(input: String) -> Self {
        let rows = input.split("\r\n").collect::<Vec<&str>>();
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; rows[0].len()]; rows.len()];


        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in row.chars().enumerate() {
                grid[row_idx][col_idx] = col;
            }
        }

        Self {
            data: grid
        }
    }

    pub fn get_cell(&self, row: i32, col: i32) -> char {
        self.data[row as usize][col as usize]
    }

    pub fn set_cell(&mut self, row: i32, col: i32, symbol: char) {
        self.data[row as usize][col as usize] = symbol;
    }

    pub fn width(&self) -> i32 {
        self.data[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn is_within_bounds(&self, position: &(i32, i32)) -> bool {
        if position.0 >= 0 && position.0 < self.height() &&
            position.1 >= 0 && position.1 < self.width() {
            true
        } else {
            false
        }
    }

    pub fn get_neighbours(&self, position: &(i32, i32)) -> Vec<((i32, i32), Direction)> {
        vec!(
            ((position.0 + 1, position.1), Direction::Down),
            ((position.0 - 1, position.1), Direction::Up),
            ((position.0, position.1 + 1), Direction::Right),
            ((position.0, position.1 - 1), Direction::Left)
        ).iter().filter(|&pos| self.is_within_bounds(&pos.0)).map(|x| *x).collect::<Vec<((i32, i32), Direction)>>()
    }

    pub fn cost(&self, position: &(i32, i32)) -> i32 {
        let x = self.get_cell(position.0, position.1);
        match x {
            _ => x.to_digit(10).unwrap() as i32
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match self{
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}