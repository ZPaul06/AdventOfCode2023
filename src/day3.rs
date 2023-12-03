use std::fs::File;
use std::io::Read;
use std::iter::zip;
use regex::Regex;

fn search_for_symbol(lines: &Vec<&str>, width: usize, height: usize,  l_index: i32, index: i32, result: &mut bool) {
    if 0 <= l_index && l_index < height as i32 && 0 <= index && index < width as i32 {
        if ["+", "*", "@", "/", "#", "$", "-", "=", "&", "%"].contains(&&lines[l_index as usize][index as usize..(index + 1) as usize]) {
            *result = true;
        }
    }
}

fn check_gear<'a>(lines: &Vec<&'a str>, width: usize, height: usize,  l_index: usize, index: usize, adjacents: &mut Vec<&'a str>) {
    let regex = Regex::new(r"[0-9]+").unwrap();
    for i in -1..=1 {
        if (l_index as i32) + i >= 0 && (l_index as i32) + i < height as i32 {
            let matches = regex.find_iter(lines[(l_index as i32 + i) as usize]);
            for m in matches {
                if (m.start() < index && index as i32 - m.start() as i32 <= m.len() as i32) || (m.start() >= index && m.start() as i32  - index as i32 <= 1) {
                    adjacents.push(m.as_str());
                }
            }
        }
    }
}

pub fn run() {
    let mut file = File::open("src/inputs/input3").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let lines: Vec<_> = input.lines().collect();
    let (width, height) = (lines[0].len(), lines.len());

    let (mut sum1, mut sum2) = (0, 0);

    for (index, line) in lines.iter().enumerate() {
        let regex = Regex::new(r"[0-9]+|\*").unwrap();
        let matches = regex.find_iter(*line);
        for m in matches {
            if m.as_str() != "*" {
                let mut result = false;
                for i in m.start()..=m.end() + 1 {
                    search_for_symbol(&lines, width, height, (index as i32) - 1, (i as i32) - 1, &mut result);
                    search_for_symbol(&lines, width, height, index as i32, (i as i32) - 1, &mut result);
                    search_for_symbol(&lines, width, height, (index as i32) + 1, (i as i32) - 1, &mut result);
                }
                if result {
                    sum1 += m.as_str().parse::<i32>().unwrap();
                }
            } else {
                let mut adjacents = vec![];
                check_gear(&lines, width, height, index, m.start(), &mut adjacents);
                if adjacents.len() == 2 {
                    sum2 += adjacents[0].parse::<i32>().unwrap() * adjacents[1].parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("--3-- Part One: {}\n--3-- Part Two: {}", sum1, sum2);
}