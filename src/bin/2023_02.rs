use std::{cmp, env};
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let input = read_to_string(file_path).unwrap();

    let total = parse_games(input);

    let max_cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let total_part_1 = total.iter()
        .filter(|game| game.max.blue <= max_cubes.blue && game.max.green <= max_cubes.green && game.max.red <= max_cubes.red)
        .map(|game| game.id)
        .sum::<i32>();

    let total_part_2 = total.iter()
        .map(|game| game.max.green * game.max.red * game.max.blue)
        .sum::<i32>();

    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);
}

struct Game {
    id: i32,
    moves: Vec<Cubes>,
    max: Cubes,
}

impl Game {

    fn new(id: i32) -> Self {
        Self {
            id,
            moves: Vec::new(),
            max: Cubes::new()
        }
    }

    fn add_move(&mut self, cube_move: Cubes) {
        self.max.red = cmp::max(self.max.red, cube_move.red);
        self.max.green = cmp::max(self.max.green, cube_move.green);
        self.max.blue = cmp::max(self.max.blue, cube_move.blue);

        self.moves.push(cube_move);
    }
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Cubes {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn set_colour(&mut self, colour: &str, count: i32) {
        match colour {
            "blue" => self.blue += count,
            "red" => self.red += count,
            "green" => self.green += count,
            _ => {}
        }
    }
}


fn parse_games(input: String) -> Vec<Game> {
    input.split("\n").fold(Vec::new(), |mut games, game_line| {
        let (game, game_info): (&str, &str) = game_line.split_once(":").unwrap();

        let (_, game_id) = game.split_once(" ").unwrap();

        let mut game = Game::new(game_id.parse().unwrap());

        for single_game in game_info.split(";") {
            let draws = single_game.split(',');

            let mut cubes_for_draw = Cubes::new();

            for draw in draws {
                let (count_str, colour) = draw.trim().split_once(" ").unwrap();
                let count = count_str.parse::<i32>().unwrap();

                cubes_for_draw.set_colour(colour, count);
            }
            game.add_move(cubes_for_draw);
        }

        games.push(game);
        games
    })
}
