use std::fs;

const FILEPATH: &str = "./input.txt";
const TEST_FILEPATH: &str = "./test_input.txt";

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut stacks = get_stacks(false);
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");

    for line in input.lines() {
        let (count, from, to) = parse_line(line);

        for _ in 0..count {
            stacks[from]
                .pop()
                .and_then(|to_move| Some(stacks[to].push(to_move)));
        }
    }

    let top_crates = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Answer 1 = {}", top_crates);
}

fn part_2() {
    let mut stacks = get_stacks(false);
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");

    for line in input.lines() {
        let (count, from, to) = parse_line(line);
        let length = stacks[from].len();
        let mut to_move = stacks[from]
            .drain((length - count as usize)..)
            .collect::<Vec<_>>();
        stacks[to].append(&mut to_move);
    }

    let top_crates = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Answer 2 = {}", top_crates);
}

fn get_stacks(use_test_data: bool) -> Vec<Vec<char>> {
    if use_test_data == true {
        return vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    }

    vec![
        vec!['R', 'P', 'C', 'D', 'B', 'G'],
        vec!['H', 'V', 'G'],
        vec!['N', 'S', 'Q', 'D', 'J', 'P', 'M'],
        vec!['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M'],
        vec!['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S'],
        vec!['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S'],
        vec!['B', 'Z', 'M', 'H', 'F', 'T', 'Q'],
        vec!['C', 'M', 'D', 'B', 'F'],
        vec!['F', 'C', 'Q', 'G'],
    ]
}

fn parse_line(line: &str) -> (u32, usize, usize) {
    let split = line.split(" ").collect::<Vec<_>>();
    let count = split[1].parse::<u32>().unwrap();
    let from = split[3].parse::<u32>().unwrap();
    let to = split[5].parse::<u32>().unwrap();
    (count, from as usize - 1, to as usize - 1)
}
