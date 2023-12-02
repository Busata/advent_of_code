    use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let input =  read_to_string(file_path).unwrap();

    let total = part_2(input);


    println!("{}", total);

}


fn part_1(input: String) -> i32 {
    input.split("\n").fold(0, |sum, game_line| {
        let (game, game_info): (&str, &str) = game_line.split_once(":").unwrap();

        let game_id: i32 = game.split_once(" ").unwrap().1.parse::<i32>().unwrap();

        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for single_game in game_info.split(";") {
            let draws = single_game.split(',');

            for draw in draws {
                let (count_str, colour) = draw.trim().split_once(" ").unwrap();

                let count = count_str.parse::<i32>().unwrap();

                match colour {
                    "blue" => {
                        if count > max_blue {
                            max_blue = count;
                        }
                    },
                    "red" => {
                        if count > max_red {
                            max_red = count;
                        }
                    }
                    "green" => {
                        if count > max_green {
                            max_green = count;
                        }
                    },
                    _ => {}
                }
            }
        }

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum + game_id
        } else {
            sum
        }
    })
}
fn part_2(input: String) -> i32 {
    input.split("\n").fold(0, |sum, game_line| {
        let (game, game_info): (&str, &str) = game_line.split_once(":").unwrap();

        let game_id: i32 = game.split_once(" ").unwrap().1.parse::<i32>().unwrap();

        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for single_game in game_info.split(";") {
            let draws = single_game.split(',');

            for draw in draws {
                let (count_str, colour) = draw.trim().split_once(" ").unwrap();

                let count = count_str.parse::<i32>().unwrap();

                match colour {
                    "blue" => {
                        if count > max_blue {
                            max_blue = count;
                        }
                    },
                    "red" => {
                        if count > max_red {
                            max_red = count;
                        }
                    }
                    "green" => {
                        if count > max_green {
                            max_green = count;
                        }
                    },
                    _ => {}
                }
            }
        }

            sum + (max_red * max_blue * max_green)
    })
}