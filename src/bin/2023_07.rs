use std::cmp::Ordering;
use std::collections::{HashMap};
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = read_to_string(file_path).unwrap();

    let mut plays = input.split("\r\n").map(|line| {
        let (hand, bid_part) = line.split_once(' ').unwrap();

        return (hand, bid_part.parse::<i32>().unwrap());
    }).collect::<Vec<(&str, i32)>>();

    plays.sort_by(order_hands_part1);
    let part1 = calculate_result(&plays);

    plays.sort_by(order_hands_part2);
    let part2 = calculate_result(&plays);

    plays.iter().for_each(|pair| {
        println!("{} {}", pair.0, pair.1);
    });

    println!("{}", part1);
    println!("{}", part2);
}

fn upgrade_card_with_joker(hand: &str, value_map: &HashMap<char, usize>) -> String {
    let character_map = build_character_map(hand);

    if character_map.keys().filter(|x| **x == 'J').count() == 0 {
        return hand.to_string();
    }

    let option = character_map
        .iter()
        .filter(|(char, value)| **char != 'J')
        .max_by(|a, b| {
           a.1.cmp(&b.1)
        })
        .map(|(k, _v)| k);

    match option {
        Some(c) => hand.clone().replace("J", &c.to_string()),
        None => hand.to_string()
    }
}

fn order_hands_part1(first: &(&str, i32), other: &(&str, i32)) -> Ordering {
    let values = vec!('A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2');
    let value_map: HashMap<char, usize> = values.iter().cloned().zip((1..=values.len()).rev()).collect();

    let play_comparison = get_hand_value(first.0).cmp(&get_hand_value(other.0));

    if play_comparison == Ordering::Equal {
        compare_hand(first.0, other.0, &value_map)
    } else {
        play_comparison
    }
}

fn order_hands_part2(first: &(&str, i32), other: &(&str, i32)) -> Ordering {
    let values = vec!('A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J');
    let value_map: HashMap<char, usize> = values.iter().cloned().zip((1..=values.len()).rev()).collect();


    let play_comparison = get_hand_value_with_joker(first.0, &value_map).cmp(&get_hand_value_with_joker(other.0, &value_map));

    if play_comparison == Ordering::Equal {
        compare_hand(first.0, other.0, &value_map)
    } else {
        play_comparison
    }
}

fn get_hand_value_with_joker(hand: &str, value_map: &HashMap<char, usize> ) -> i32 {
    let improved_card = &upgrade_card_with_joker(hand, value_map);
    return get_hand_value(improved_card);
}

fn get_hand_value(hand: &str) -> i32 {
    let character_map = build_character_map(hand);
    if is_five_of_kind(&character_map) { 7 }
    else if is_four_of_kind(&character_map) { 6 }
    else if is_full_house(&character_map) { 5 }
    else if is_three_of_a_kind(&character_map) { 4 }
    else if is_two_pair(&character_map) { 3 }
    else if is_one_pair(&character_map) { 2 }
    else { 1 }
}


fn is_five_of_kind(character_map: &HashMap<char, i32>) -> bool {
    return character_map.values().count() == 1;
}

fn is_four_of_kind(character_map: &HashMap<char, i32>) -> bool {
    character_map.values().filter(|count| **count == 4).count() == 1
}

fn is_full_house(character_map: &HashMap<char, i32>) -> bool {
    character_map.values().filter(|x| **x == 2).count() == 1 &&
        character_map.values().filter(|x| **x == 3).count() == 1
}

fn is_three_of_a_kind(character_map: &HashMap<char, i32>) -> bool {
    character_map.values().filter(|x| **x == 3).count() == 1
}

fn is_two_pair(character_map: &HashMap<char, i32>) -> bool {
    character_map.values().filter(|count| **count == 2).count() == 2
}

fn is_one_pair(character_map: &HashMap<char, i32>) -> bool {
    character_map.values().filter(|count| **count == 2).count() == 1
}

fn is_high_card(character_map: &HashMap<char, i32>) -> bool {
    character_map.values().count() == 5
}

fn compare_hand(hand: &str, other_hand: &str, x: &HashMap<char, usize>) -> Ordering {
    let zip = hand.chars().zip(other_hand.chars());

    for (first, second) in zip {
        if first == second {
            continue;
        } else {
            let a = x.get(&first).unwrap();
            let b = x.get(&second).unwrap();
            return a.cmp(b);
        }
    }

    Ordering::Equal
}

fn build_character_map(hand: &str) -> HashMap<char, i32> {
    let characters = hand.chars().fold(HashMap::new(), |mut map, char| {
        if !map.contains_key(&char) {
            map.insert(char, 1);
        } else {
            map.insert(char, map.get(&char).unwrap() + 1);
        }
        map
    });
    characters
}

fn calculate_result(plays: &Vec<(&str, i32)>) -> i32 {
    plays.clone().into_iter().enumerate().fold(0, |result, (i, (hand, bid))| {
        let rank: i32 = i as i32 + 1;

        result + (rank * bid)
    })
}