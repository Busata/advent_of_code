use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let lines = input.split("\r\n").collect::<Vec<&str>>();

    let instructions = lines[0];
    let network = lines.iter().skip(2).map(|x| *x).collect::<Vec<&str>>();

    let mut graph: HashMap<&str, Node> = network.iter().map(|node| {
        let (node_name_part, instructions_part) = node.split_once("=").unwrap();
        let (left_part, right_part) = instructions_part.split_once(", ").unwrap();


        let node_name = node_name_part.trim();
        let left = left_part.replace(" (", "");
        let right = right_part.replace(")", "");

        (node_name, Node::new(node_name.to_string(), left.to_string(), right.to_string()))
    }).collect();

    println!("{:?}", graph);

    //let steps = find_steps_part1(instructions, &mut graph);
    //println!("Steps part 1: {}", steps);

    let steps2 = find_steps_part2(instructions, &mut graph);
    println!("Steps part 2: {}", steps2);

}

fn find_steps_part2(instructions: &str, graph: &mut HashMap<&str, Node>) -> i64 {

    let mut paths: Vec<Path> =  graph.keys().filter(|x| x.ends_with("A")).map(|key| Path { current_node: key.to_string()}).collect();
    let mut steps = 0;

    while !are_paths_done(&paths) {
        for i in instructions.chars() {

            for p in paths.iter_mut() {
                let string = p.current_node.as_str();

                let next_node = match i {
                    'L' => &graph.get(string).unwrap().left,
                    'R' => &graph.get(string).unwrap().right,
                    _ => ""
                };

                p.current_node = next_node.to_string();
            }
            steps += 1;

            if are_paths_done(&paths) {
                break;
            }
        }
        println!("Progress... steps: {}", steps)
    }

    steps
}

fn are_paths_done(paths: &Vec<Path>) -> bool {
    paths.iter().all(|path| path.current_node.ends_with("Z"))
}


fn find_steps_part1(instructions: &str, graph: &mut HashMap<&str, Node>) -> i32 {
    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        for  i in instructions.chars() {
            current_node = match i {
                'L' => &graph.get(current_node).unwrap().left,
                'R' => &graph.get(current_node).unwrap().right,
                _ => ""
            };

            steps += 1;


            if (current_node == "ZZZ") {
                return steps;
            }
        }
    }
    -1
}

struct Path {
    current_node: String
}


#[derive(Debug)]
struct Node {
    key: String,
    left: String,
    right: String,
}


impl Node {
    fn new(key: String, left: String, right: String) -> Self {
        Self {
            key,
            left,
            right,
        }
    }
}
