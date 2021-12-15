use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FIELD_SIZE: usize = 100;
const LARGE_MULTIPLIER: usize = 5;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Tile {
    coordinates: (usize, usize),
    cost: i32,
    path_cost: Cell<i32>,
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        other.path_cost.get().cmp(&self.path_cost.get())
    }
}

fn get_adjacent_coordinates((x, y): (usize, usize), field_size: usize) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if x != 0 {
        adjacent.push((x - 1, y));
    }
    if x != field_size - 1 {
        adjacent.push((x + 1, y));
    }
    if y != 0 {
        adjacent.push((x, y - 1));
    }
    if y != field_size - 1 {
        adjacent.push((x, y + 1));
    }
    adjacent
}

fn build_cheapest_paths(field: &Vec<Vec<Tile>>) {
    let field_size = field.len();
    field[field_size - 1][field_size - 1].path_cost.set(0);
    let mut queue = BinaryHeap::new();
    queue.push(&field[field_size - 1][field_size - 1]);
    while !queue.is_empty() {
        let tile = queue.pop().unwrap();
        for (x, y) in get_adjacent_coordinates(tile.coordinates, field_size) {
            let new_cost = tile.path_cost.get() + tile.cost;
            if new_cost < field[x][y].path_cost.get() {
                field[x][y].path_cost.set(new_cost);
                queue.push(&field[x][y]);
            }
        }
    }
}

fn get_cheapest_path(field: &Vec<Vec<Tile>>) -> i32 {
    field[0][0].path_cost.get()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut field: Vec<Vec<Tile>> = Vec::new();
    for i in 0..FIELD_SIZE {
        let mut row_vec = Vec::new();
        for j in 0..FIELD_SIZE {
            row_vec.push(Tile {
                coordinates: (i, j),
                cost: 0,
                path_cost: Cell::new(i32::MAX),
            });
        }
        field.push(row_vec);
    }
    for (x, line) in (&lines).into_iter().enumerate() {
        for (y, cost_char) in line.chars().enumerate() {
            let cost = cost_char.to_digit(10).unwrap();
            field[x][y].cost = cost as i32;
        }
    }

    let mut large_field: Vec<Vec<Tile>> = Vec::new();
    for i in 0..FIELD_SIZE * LARGE_MULTIPLIER {
        let mut large_row_vec = Vec::new();
        for j in 0..FIELD_SIZE * LARGE_MULTIPLIER {
            large_row_vec.push(Tile {
                coordinates: (i, j),
                cost: 0,
                path_cost: Cell::new(i32::MAX),
            });
        }
        large_field.push(large_row_vec);
    }

    for (x, line) in (&lines).into_iter().enumerate() {
        for m in 0..LARGE_MULTIPLIER {
            for (y, cost_char) in line.chars().enumerate() {
                let cost = cost_char.to_digit(10).unwrap() as i32;
                for n in 0..LARGE_MULTIPLIER {
                    let extra_cost = (m + n) as i32;
                    let real_cost;
                    if extra_cost + cost > 9 {
                        real_cost = extra_cost + cost - 9;
                    } else {
                        real_cost = extra_cost + cost;
                    }
                    let real_x = x + m * FIELD_SIZE;
                    let real_y = y + n * FIELD_SIZE;
                    large_field[real_x][real_y].cost = real_cost;
                }
            }
        }
    }

    build_cheapest_paths(&field);
    build_cheapest_paths(&large_field);
    println!("Part 1: {}", get_cheapest_path(&field));
    println!("Part 2: {}", get_cheapest_path(&large_field));
}
