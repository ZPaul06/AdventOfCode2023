use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn run() {
    let mut file = File::open("src/inputs/input2").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let (mut sum1, mut sum2) = (0, 0);
    let conditions = [(" red", 12), (" green", 13), (" blue", 14)];

    for (id, line) in input.split("\n").enumerate() {
        let raw = Regex::new(r"Game [0-9]*: ").unwrap().replace(line, "");
        let game = raw.trim();
        let (mut possible, mut rgb) = (true, [0,0,0]);
        for set in game.split("; ") {
            for amount in set.split(", ") {
                for (index, (c, a_c)) in conditions.iter().enumerate() {
                    if amount.contains(c) {
                        let a = amount.replace(c, "").parse::<i32>().unwrap();
                        rgb[index] = if a > rgb[index] {a} else {rgb[index]};
                        possible = if amount.replace(c, "").parse::<i32>().unwrap() > *a_c {false} else {possible};
                    }
                }
            }
        }
        if possible {
            sum1 += id + 1;
        }
        sum2 += rgb[0] * rgb[1] * rgb[2];
    }

    println!("--2-- Part One: {}\n--2-- Part Two: {}", sum1, sum2);
}