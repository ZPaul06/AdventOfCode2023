use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::zip;

fn get_type(card: &str, part_two: bool) -> usize {
    let mut chars = card.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();

    if chars.len() == 2 {
        let mut c = card.chars().collect::<Vec<_>>();
        c.sort();
        if c[0] == c[1] && c[3] == c[4]  {
            4
        } else {
            5
        }
    } else if chars.len() == 3 {
        let mut c = card.chars().collect::<Vec<_>>();
        c.sort();
        if c[0] == c[2] || c[1] == c[3] || c[2] == c[4] {
            3
        } else {
            2
        }
    } else if chars.len() == 1 {
        6
    } else {
        6 - chars.len() - 1
    }
}

fn compare_hands(hand_a: &str, hand_b: &str, map: &HashMap<&str, usize>, part_two: bool) -> Ordering {
    let (type_a, type_b) = (&get_type(hand_a, part_two), &get_type(hand_b, part_two));
    if !(part_two && (hand_a.contains("J") || hand_b.contains("J"))) {
        if type_a != type_b {
            type_a.partial_cmp(type_b).unwrap()
        } else {
            for (card_a, card_b) in zip(hand_a.chars(), hand_b.chars()) {
                if card_a != card_b {
                    return map.get(card_a.to_string().as_str()).unwrap().partial_cmp(map.get(card_b.to_string().as_str()).unwrap()).unwrap()
                }
            }
            Ordering::Equal
        }
    } else {
        Ordering::Equal
    }
}

pub fn run() {
    let mut file = File::open("src/inputs/input7").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let map = HashMap::from([("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("T", 10), ("J", 11), ("Q", 12), ("K", 13), ("A", 14)]);
    let map_2 = HashMap::from([("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("T", 10), ("J", 1), ("Q", 12), ("K", 13), ("A", 14)]);

    let mut sum = 0;

    let mut hands_and_bids = input.split("\n").collect::<Vec<_>>().iter().map(|s| s.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut hands_and_bids_2 = hands_and_bids.clone();

    hands_and_bids.sort_by(|h_b_a, h_b_b| compare_hands(h_b_a[0], h_b_b[0], &map, false));
    //hands_and_bids_2.sort_by(|h_b_a, h_b_b| compare_hands(h_b_a[0], h_b_b[0], &map_2, true));

    for (fact, h_b) in hands_and_bids.iter().enumerate() {
        sum += (fact + 1) * h_b[1].parse::<usize>().unwrap();
    }

    println!("--7-- Part One: {}", sum);
}