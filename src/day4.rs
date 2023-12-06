use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn run() {
    let mut file = File::open("src/inputs/input4").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let (mut sum1, mut sum2) = (0,0);

    let mut cards = vec![];
    let mut map = HashMap::<usize, (u32, u32)>::new();

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

        cards.push(index);
        map.insert(index, (c, 1));

        sum1 += if c > 0 {i32::pow(2, c-1)} else {0};
    }

    let indices = cards.len();

    while !cards.is_empty() {
        let mut new_cards = vec![];
        sum2 += cards.len();

        for card in cards.iter() {
            let (winning, amount) = map.get(card).unwrap();
            for i in 1usize..=(*winning as usize) {
                if card + i < indices {
                    new_cards.push(card + i);
                    let (c, new) = *map.get(&(card + i)).unwrap();

                    map.insert(card + i, (c, new + 1));
                }
            }
        }

        cards = new_cards.clone();
    }

    println!("--4-- Part One: {}\n--4-- Part Two: {}", sum1, sum2);
}