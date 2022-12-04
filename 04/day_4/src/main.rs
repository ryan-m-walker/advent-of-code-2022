use std::fs;

const FILEPATH: &str = "./input.txt";

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");
    let lines = input.lines();

    let mut overlaps = 0;

    for line in lines {
        let pairs = line.split(',').collect::<Vec<_>>();
        let a = pairs[0];
        let b = pairs[1];

        let (a_start, a_end) = parse_range(a);
        let (b_start, b_end) = parse_range(b);

        if a_start <= b_start && a_end >= b_end {
            overlaps += 1;
            continue;
        }

        if b_start <= a_start && b_end >= a_end {
            overlaps += 1;
        }
    }

    println!("Answer 1 = {}", overlaps);
}

fn part_2() {
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");
    let lines = input.lines();

    let mut overlaps = 0;

    for line in lines {
        let pairs = line.split(',').collect::<Vec<_>>();
        let a = pairs[0];
        let b = pairs[1];

        let (a_start, a_end) = parse_range(a);
        let (b_start, b_end) = parse_range(b);

        if a_start <= b_start && a_end >= b_start {
            overlaps += 1;
            continue;
        }

        if b_start <= a_start && b_end >= a_start {
            overlaps += 1;
        }
    }

    println!("Answer 2 = {}", overlaps);
}

fn parse_range(input: &str) -> (u32, u32) {
    let range = input.split('-').collect::<Vec<_>>();
    let start = range[0].parse::<u32>().unwrap();
    let end = range[1].parse::<u32>().unwrap();
    (start, end)
}
