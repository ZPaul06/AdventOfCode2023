use std::fs::File;
use std::io::Read;
use regex::Regex;

fn replace(number: &str) -> &str{
    match number {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => number
    }
}

pub fn run() {
    let mut file = File::open("src/inputs/input1").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let (mut sum1, mut sum2) = (0, 0);

    for line in input.split("\n") {
        let raw = Regex::new(r"[A-Za-z]").unwrap().replace_all(line, "");
        let digits = raw.trim();
        sum1 += [&digits[0..1], &digits[digits.len()-1..digits.len()]].concat().parse::<i32>().unwrap();

        let matches: Vec<_> = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap().find_iter(line).collect();
        let (first, last) = (replace(matches.first().unwrap().as_str()), replace(matches.last().unwrap().as_str()));
        sum2 += [first, last].concat().parse::<i32>().unwrap();
    }

    println!("--1-- Part One: {}\n--1-- Part Two: {}", sum1, sum2);
}