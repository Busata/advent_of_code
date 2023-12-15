use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_path = "input/2023_15.txt";
    let input = read_to_string(file_path).unwrap();

    let part1 = calculate_hash(&input);

    println!("Part 1: {}", part1);

    let part2 = calculate_hashmap(input);

    println!("Part 2: {}", part2);
}

fn calculate_hash(input: &String) -> u32 {
    let part1 = input.split(',').map(|sequence| {
        let sum = sequence.chars().filter(|&x| x != '\n').fold(0, |acc, x| {
            let value = x as u32;

            return ((acc + value) * 17) % 256
        });

        sum
    }).sum::<u32>();
    part1
}

fn calculate_hashmap(input: String) -> u32 {
    let boxes = input.split(',').fold(HashMap::new(), |mut boxes, sequence| {
        let label: String = sequence.chars().take_while(|&x| x != '-' && x != '=').collect();
        let instruction: String = sequence.chars().skip_while(|&x| x != '-' && x != '=').collect();

        let box_number = calculate_hash(&label);


        match instruction.chars().next().unwrap() {
            '=' => {
                let x1 = boxes.entry(box_number).or_insert(Vec::new());

                let box_label = format!("{} {}", label, instruction[1..].parse::<usize>().unwrap());
                if let Some(idx) = x1.iter().position(|x: &String| x.clone().starts_with(&label)) {
                    x1[idx] = box_label.clone();
                } else {
                    x1.push(box_label);
                }
            }
            '-' => {
                boxes.entry(box_number).and_modify(|labels| {
                    if let Some(idx) = labels.iter().position(|x| x.clone().starts_with(&label)) {
                        labels.remove(idx);
                    }
                });
            },
            _ => {}
        }

        boxes
    });


    return boxes.iter().map(|(key, value)| {

       value.iter().map(|lens| {
            return lens.split_once(' ').unwrap().1.parse::<u32>().unwrap();
        }).enumerate().map(|(idx, number)| {

            let pt1 = key + 1;
            let pt2 = idx as u32 + 1 ;
            let pt3 = number;

            pt1 * pt2 * pt3

        }).sum::<u32>()

    }).sum::<u32>()

}