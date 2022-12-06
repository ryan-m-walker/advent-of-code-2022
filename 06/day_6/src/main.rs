use itertools::Itertools;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    let mut marker = vec![];

    for (i, c) in input.chars().enumerate() {
        marker.push(c);

        if marker.len() > 4 {
            // remove first element
            marker.drain(0..1);
        }

        let mut clone: Vec<_> = marker.clone().into_iter().unique().collect();
        clone.dedup();

        if marker.len() == 4 && clone.len() == marker.len() {
            println!("Answer 1 = {}", i + 1);
            return;
        }
    }
}

fn part_2() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    let mut marker = vec![];

    for (i, c) in input.chars().enumerate() {
        marker.push(c);

        if marker.len() > 14 {
            // remove first element
            marker.drain(0..1);
        }

        let mut clone: Vec<_> = marker.clone().into_iter().unique().collect();
        clone.dedup();

        if marker.len() == 14 && clone.len() == marker.len() {
            println!("Answer 2 = {}", i + 1);
            return;
        }
    }
}
