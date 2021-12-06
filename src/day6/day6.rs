use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_DAYS: i32 = 8;

fn get_fish_count(start_fish_map: &HashMap<i32, i64>, days: i32) -> i64 {
    let mut fish_map = start_fish_map;
    let mut new_fish_map: HashMap<i32, i64>;
    for _ in 0..days {
        let mut tmp_fish_map = HashMap::new();

        let zero_count = *fish_map.get(&0).unwrap();
        tmp_fish_map.insert(8, zero_count);
        for day in 1..=MAX_DAYS {
            tmp_fish_map.insert(day - 1, *fish_map.get(&day).unwrap());
        }
        *(tmp_fish_map.get_mut(&6).unwrap()) += zero_count;

        new_fish_map = tmp_fish_map;
        fish_map = &new_fish_map;
    }
    let mut total_fish: i64 = 0;
    for day in 0..=MAX_DAYS {
        total_fish += *fish_map.get(&day).unwrap() as i64;
    }
    total_fish
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let input_lines = reader.lines().map(|l| l.unwrap());
    let mut fishes = Vec::new();
    for line in input_lines {
        fishes = line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
    }

    let mut fish_map = HashMap::new();
    for fish in fishes {
        let count = fish_map.entry(fish).or_insert(0 as i64);
        *count += 1;
    }
    for day in 0..=MAX_DAYS {
        fish_map.entry(day).or_insert(0);
    }
    println!("Part 1: {}", get_fish_count(&fish_map, 80));
    println!("Part 2: {}", get_fish_count(&fish_map, 256));
}
