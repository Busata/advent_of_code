use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::future::Future;
use std::hash::{Hash, Hasher};
use priority_queue::PriorityQueue;

mod helpers;

use crate::helpers::{Direction, Grid2D};


fn main() {
    let file_path = "input/2023_17.txt";
    let input = read_to_string(file_path).unwrap();

    let mut grid = Grid2D::from_string(input);


    let start = (0, 0);
    let goal = (grid.height() - 1, grid.width() - 1);


    let part1 = dijkstra_search(&grid, start, |n| n.position == goal, get_part_1_neighbours);
    let part2 = dijkstra_search(&grid, start, |n| n.position == goal && n.steps_taken >= 4, get_part_2_neighbours);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}

#[derive(Hash, Clone, Eq,PartialEq, PartialOrd, Copy, Debug)]
struct Node {
    position: (i32, i32),
    direction: Direction,
    steps_taken: i32
}

impl Node {
    fn new(position: (i32, i32), direction: Direction, steps_taken: i32) -> Self {
        Self {
            position, direction, steps_taken
        }
    }
}

fn dijkstra_search(grid: &Grid2D, start: (i32, i32), goal: impl Fn(&Node) -> bool,  valid_neighbours: fn(&Grid2D, &Node) -> Vec<Node>) -> i32 {
    let mut frontier: PriorityQueue<Node, Reverse<i32>> = PriorityQueue::new();
    frontier.push(Node::new(start.clone(), Direction::Right, 0), Reverse(0));
    frontier.push(Node::new(start.clone(), Direction::Down, 0), Reverse(0));

    let mut cost_so_far: HashMap<Node, i32> = HashMap::new();

    cost_so_far.insert(Node::new(start.clone(), Direction::Right, 0), 0);
    cost_so_far.insert(Node::new(start.clone(), Direction::Down, 0), 0);


    while !frontier.is_empty() {
        let current = frontier.pop().unwrap().0;

        if goal(&current) {
            return *cost_so_far.get(&current).unwrap();
        }

        for neigbour in valid_neighbours(grid, &current) {
            let new_cost = cost_so_far.get(&current).unwrap() + grid.cost(&neigbour.position);

            if !cost_so_far.contains_key(&neigbour) || new_cost < *cost_so_far.get(&neigbour).unwrap() {
                cost_so_far.insert(neigbour, new_cost);
                frontier.push(neigbour, Reverse(new_cost));
            }
        }
    }

    println!("Goal not found");

    return 0;

}

fn get_part_1_neighbours(grid: &Grid2D, current: &Node) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();

    for(position, direction) in grid.get_neighbours(&current.position) {
        if direction == current.direction.opposite() {
            continue;
        }

        if current.direction != direction {
            neighbours.push(Node::new(position, direction, 1));
        } else if current.steps_taken < 3 {
            neighbours.push(Node::new(position, direction, current.steps_taken + 1));
        }
    }

    neighbours
}


fn get_part_2_neighbours(grid: &Grid2D, current: &Node) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();

    for(position, direction) in grid.get_neighbours(&current.position) {
        if direction == current.direction.opposite() {
            continue;
        }
        if current.direction != direction && current.steps_taken >= 4 {
            neighbours.push(Node::new(position, direction, 1));
        } else if current.direction == direction && current.steps_taken < 10 {
            neighbours.push(Node::new(position, direction, current.steps_taken + 1));
        }
    }
    neighbours
}



