use std::fs::File;
use std::io::Read;

fn count(seeds: Vec<u32>, instructions: &Vec<Vec<Vec<u32>>>, locations: &mut Vec<u32>) {
    'seed_loop: for seed in seeds {
        let mut value = seed;
        'map_loop: for instr in instructions {
            'inst_loop: for i in instr {
                if value >= i[1] && value - i[1] < i[2] {
                    value = i[0] + (value - i[1]);
                    continue 'map_loop;
                }
            }
        }
        locations.push(value);
    }
}

pub fn run() {
    let mut file = File::open("src/inputs/input5").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Cannot read file!");

    let mut parts = input.split("\r\n\r\n");
    let (seeds, maps): (Vec<_>, Vec<_>) = (parts.next().unwrap().replace("seeds: ", "").split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect(), parts.collect());
    let mut seeds_c = seeds.clone();

    let mut instructions = vec![];

    for map in maps {
        let mut x = vec![];
        let mut raw = map.split("\r\n");
        raw.next();
        for line in raw {
            x.push(line.split_whitespace().map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>());
        }
        instructions.push(x);
    }

    let (mut locations, mut locations_2) = (vec![], vec![]);

    count(seeds, &instructions, &mut locations);

    println!("--5-- Part One: {}", locations.iter().min().unwrap());

    while !seeds_c.is_empty() {
        let (mut seeds_2, mut locs_2) = (vec![], vec![]);
        for i in 0..seeds_c[1] {
            seeds_2.push(seeds_c[0] + i);
        }
        seeds_c.remove(0);
        seeds_c.remove(0);
        count(seeds_2, &instructions, &mut locs_2);
        let min = *locs_2.iter().min().unwrap();
        locs_2.clear();
        locations_2.push(min);
    }

    println!("--5-- Part Two {}", locations_2.iter().min().unwrap());
}