use std::fs;

const FILEPATH: &str = "./input.txt";

#[derive(Debug, PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn from_str(value: &str) -> Self {
        match value {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => panic!("Unknown value"),
        }
    }

    pub fn compare(opponent: Play, player: Play) -> Result {
        if opponent == player {
            return Result::Draw;
        }

        if opponent == Play::Rock && player == Play::Scissors {
            return Result::Defeat;
        }

        if opponent == Play::Paper && player == Play::Rock {
            return Result::Defeat;
        }

        if opponent == Play::Scissors && player == Play::Paper {
            return Result::Defeat;
        }

        Result::Win
    }
}

#[derive(Debug, Clone, Copy)]
enum Result {
    Win,
    Defeat,
    Draw,
}

impl Result {
    pub fn from_str(value: &str) -> Self {
        match value {
            "X" => Result::Defeat,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown value {}", value),
        }
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");
    let lines = input.lines();

    let mut total_score = 0;

    for line in lines {
        let split: Vec<_> = line.split(" ").collect();
        let opponent = Play::from_str(split[0]);
        let player = Play::from_str(split[1]);

        let result = Play::compare(opponent, player);
        let score = get_score(player, result);
        total_score += score;
    }

    println!("Answer 1 = {}", total_score);
}

fn part_2() {
    let input = fs::read_to_string(FILEPATH).expect("Unable to read file");
    let lines = input.lines();

    let mut total_score = 0;

    for line in lines {
        let split: Vec<_> = line.split(" ").collect();
        let opponent = Play::from_str(split[0]);
        let required_result = Result::from_str(split[1]);
        let play = get_required_play(opponent, required_result);

        let score = get_score(play, required_result);
        total_score += score;
    }

    println!("Answer 2 = {}", total_score);
}

fn get_score(play: Play, result: Result) -> i32 {
    let mut total = 0;

    match play {
        Play::Rock => total += 1,
        Play::Paper => total += 2,
        Play::Scissors => total += 3,
    }

    match result {
        Result::Win => total += 6,
        Result::Draw => total += 3,
        Result::Defeat => {}
    }

    total
}

fn get_required_play(opponent: Play, required_result: Result) -> Play {
    match opponent {
        Play::Rock => match required_result {
            Result::Win => return Play::Paper,
            Result::Draw => return Play::Rock,
            Result::Defeat => return Play::Scissors,
        },
        Play::Paper => match required_result {
            Result::Win => return Play::Scissors,
            Result::Draw => return Play::Paper,
            Result::Defeat => return Play::Rock,
        },
        Play::Scissors => match required_result {
            Result::Win => return Play::Rock,
            Result::Draw => return Play::Scissors,
            Result::Defeat => return Play::Paper,
        },
    }
}
