use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use regex::Regex;
use num::integer::lcm;

fn lcm_iter(nums: Vec<usize>) -> usize{
    let mut r = 1;
    for i in 0..nums.len() {
        r = lcm(r, nums[i]);
    }
    return r
}

fn calculate(start: &str, instructions: &Vec<char>, map: &HashMap<&str, (&str, &str)>, part_two: bool) -> usize {
    let (mut step, mut c) = (0, 0);
    let mut next = start;

    loop {
        step += 1;

        let (l, r) = map.get(next).unwrap();
        if instructions[c] == 'L' {
            next = *l;
        } else {
            next = *r;
        }
        if (!part_two && next == "ZZZ") || (part_two && next.rfind("Z") == Some(2)) {
            break
        }

        c = (c + 1) % instructions.len();
    }

    step
}

pub fn run() {
    let mut file = File::open("src/inputs/input8").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let mut map = HashMap::new();

    let mut lines = input.split("\n");
    let instructions = lines.next().unwrap().trim().chars().collect::<Vec<_>>();
    lines.next();
    for line in lines {
        let re = Regex::new(r"[A-Z]{3}").unwrap();
        let mut nodes = re.find_iter(line);
        map.insert(nodes.next().unwrap().as_str(), (nodes.next().unwrap().as_str(), nodes.next().unwrap().as_str()));
    }

    let steps1 = calculate("AAA", &instructions, &map, false);

    let mut steps = vec![];
    let starts = map.keys().map(|s| *s).filter(|k| (*k).rfind("A") == Some(2)).collect::<Vec<_>>();
    for s in starts {
        steps.push(calculate(s, &instructions, &map, true));
    }

    let steps2 = lcm_iter(steps);

    println!("--8-- Part One: {}\n--8-- Part Two: {}", steps1, steps2);
}