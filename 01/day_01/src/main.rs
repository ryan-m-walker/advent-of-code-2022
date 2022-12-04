use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut lines = file.lines();

    let mut totals = vec![];
    let mut current_total = 0;

    while let Some(line) = lines.next() {
        if line == "" {
            totals.push(current_total);
            current_total = 0;
        }

        if let Ok(num) = line.parse::<i32>() {
            current_total += num;
        }
    }

    totals.push(current_total);

    totals.sort();
    totals.reverse();

    let mut final_total = 0;
    for i in 0..3 {
        if let Some(value) = totals.get(i) {
            if i == 0 {
                println!("Answer 1 = {}", value);
            }
            final_total += value;
        }
    }

    println!("Answer 2 = {}", final_total);
}
