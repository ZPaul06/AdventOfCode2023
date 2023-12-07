use std::fs::File;
use std::io::Read;
use std::iter::zip;

pub fn run() {
    let mut file = File::open("src/inputs/input6").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let (mut prod, mut sum) = (1, 0);

    let mut lines = input.split("\n");
    let mut races = zip(lines.next().unwrap().split_whitespace().collect::<Vec<_>>(), lines.next().unwrap().split_whitespace().collect::<Vec<_>>());

    races.next();

    let (mut t_c, mut d_c) = (String::from(""), String::from(""));

    for (time, distance) in races {
        t_c += time;
        d_c += distance;
        let (t, d) = (time.parse::<usize>().unwrap(), distance.parse::<usize>().unwrap());
        let mut beating = 0;

        for i in 0..=t {
            if i*(t-i) > d {
                beating += 1;
            }
        }

        prod *= beating;
    }

    for i in 0..=t_c.parse::<usize>().unwrap() {
        if i*(t_c.parse::<usize>().unwrap()-i) > d_c.parse::<usize>().unwrap() {
            sum += 1;
        }
    }

    println!("--6-- Part One: {}\n--6-- Part Two: {}", prod, sum);
}