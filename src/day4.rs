use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn run() {
    let mut file = File::open("src/inputs/input4").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let mut sum = 0;

    for (index, line) in input.split("\n").enumerate() {
        let raw = Regex::new(r"Card [0-9]*: ").unwrap().replace(line, "");
        let mut split = raw.trim().split("|");
        let (winning_numbers, numbers): (Vec<_>, Vec<_>) = (split.next().unwrap().split_whitespace().collect(), split.next().unwrap().split_whitespace().collect());

        let mut c = 0;

        for i in &numbers {
            if winning_numbers.contains(i) {
                c += 1;
            }
        }

        sum += if c > 0 {i32::pow(2, c-1)} else {0};
    }

    println!("--4-- Part One: {}", sum);
}