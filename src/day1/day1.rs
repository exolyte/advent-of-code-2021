use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn count_increments(numbers: &Vec<i32>) -> i32 {
    let mut increment_count = 0;
    let mut previous_number = numbers[0];
    for number in numbers {
        if *number > previous_number {
            increment_count += 1;
        }
        previous_number = *number;
    }
    increment_count
}

fn count_increments_window(numbers: &Vec<i32>, window_size: usize) -> i32 {
    let mut increment_count = 0;
    let mut previous_sum: i32 = numbers[0..window_size].iter().sum();
    for i in 0..(numbers.len() - 2) {
        let current_sum = numbers[i..i + window_size].iter().sum();
        if current_sum > previous_sum {
            increment_count += 1;
        }
        previous_sum = current_sum;
    }
    increment_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let numbers = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", count_increments(&numbers));
    println!("Part 2: {}", count_increments_window(&numbers, 3));
}
