use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_moves(start_positions: &Vec<i32>) -> i32 {
    let mut positions = start_positions.to_vec();
    positions.sort();
    let center = start_positions.len() / 2;
    let median = positions[center]; //Close enough
    positions.into_iter().map(|p| (median - p).abs()).sum()
}

fn get_moves_2(positions: &Vec<i32>) -> i32 {
    let sum: i32 = positions.iter().sum();
    let average = sum / positions.len() as i32; //Close enough
    positions
        .into_iter()
        .map(|p| (average - p).abs())
        .map(|d| (d * (d + 1)) / 2)
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let positions: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|p| p.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", get_moves(&positions));
    println!("Part 2: {}", get_moves_2(&positions));
}
