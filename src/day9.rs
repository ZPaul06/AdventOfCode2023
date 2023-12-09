use std::fs::File;
use std::io::Read;

fn extrapolate(history: &Vec<i32>) -> i32 {
    let mut change = vec![];
    let mut zeros = true;
    for i in 1..history.len() {
        if history[i] - history[i - 1] != 0 {zeros = false}
        change.push(history[i] - history[i - 1]);
    }
    if zeros {
        return history[history.len() - 1]
    } else {
        return history[history.len() - 1] + extrapolate(&change)
    }
}

pub fn run() {
    let mut file = File::open("src/inputs/input9").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let (mut sum1, mut sum2) = (0, 0);

    for line in input.split("\n") {
        let mut history = line.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        sum1 += extrapolate(&history);
        history.reverse();
        sum2 += extrapolate(&history);
    }

    println!("--9-- Part One: {}\n--9-- Part Two: {}", sum1, sum2);
}