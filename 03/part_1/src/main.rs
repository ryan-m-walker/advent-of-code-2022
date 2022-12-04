use std::fs;

const FILEPATH: &str = "./input.txt";

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");
    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let half_way = line.len() / 2 as usize;
        let compartment_1 = &line[..half_way];
        let compartment_2 = &line[half_way..];

        let matching_char = find_match(compartment_1, compartment_2).unwrap();
        let priority = get_priority(matching_char);

        sum += priority;
    }

    println!("Answer 1 = {sum}");
}

fn part_2() {
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");
    let lines = input.lines();

    let mut sum = 0;
    let mut group = vec![];

    for line in lines {
        group.push(line);

        if group.len() == 3 {
            let matching_char = find_group_match(group[0], group[1], group[2]).unwrap();
            sum += get_priority(matching_char);
            group = vec![];
        }
    }

    println!("Answer 2 = {sum}");
}

fn find_match(a: &str, b: &str) -> Option<char> {
    for char in a.chars() {
        if b.contains(char) {
            return Some(char);
        }
    }

    None
}

fn find_group_match(a: &str, b: &str, c: &str) -> Option<char> {
    for char in a.chars() {
        if b.contains(char) && c.contains(char) {
            return Some(char);
        }
    }

    None
}

fn get_priority(input: char) -> u32 {
    if input.is_lowercase() {
        input as u32 - 96
    } else {
        (input as u32 - 64) + 26
    }
}
