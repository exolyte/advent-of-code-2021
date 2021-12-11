use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_ENERGY: i32 = 9;
const FIELD_SIZE: usize = 10;

#[derive(Debug)]
struct Octopus {
    energy: i32,
    flashed: bool,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn get_adjacent(&self) -> Vec<Coordinate> {
        let mut adjacent = Vec::new();
        let x = self.x;
        let y = self.y;
        if x != 0 {
            adjacent.push(Coordinate { x: x - 1, y });
            if y != 0 {
                adjacent.push(Coordinate { x: x - 1, y: y - 1 })
            }
            if y != FIELD_SIZE - 1 {
                adjacent.push(Coordinate { x: x - 1, y: y + 1 })
            }
        }
        if x != FIELD_SIZE - 1 {
            adjacent.push(Coordinate { x: x + 1, y });
            if y != 0 {
                adjacent.push(Coordinate { x: x + 1, y: y - 1 })
            }
            if y != FIELD_SIZE - 1 {
                adjacent.push(Coordinate { x: x + 1, y: y + 1 })
            }
        }
        if y != 0 {
            adjacent.push(Coordinate { x, y: y - 1 })
        }
        if y != FIELD_SIZE - 1 {
            adjacent.push(Coordinate { x, y: y + 1 })
        }
        adjacent
    }
}

struct Field {
    octos: [[Octopus; FIELD_SIZE]; FIELD_SIZE],
}

impl Field {
    fn reset_flashed(&mut self) {
        for row in &mut self.octos {
            row.into_iter().for_each(|o| o.flashed = false);
        }
    }

    fn step_increment(&mut self) {
        for row in &mut self.octos {
            row.into_iter().for_each(|o| o.energy += 1);
        }
    }

    fn flash(&mut self, c: &Coordinate) -> Vec<Coordinate> {
        self.octos[c.x][c.y].energy = 0;
        self.octos[c.x][c.y].flashed = true;
        let adjacent = c.get_adjacent();
        for ac in &adjacent {
            if self.octos[ac.x][ac.y].flashed {
                continue;
            }
            self.octos[ac.x][ac.y].energy += 1;
        }
        adjacent
    }

    fn step_flash(&mut self) -> i32 {
        let mut candidates = Vec::new();
        let mut flash_count = 0;
        for x in 0..FIELD_SIZE {
            for y in 0..FIELD_SIZE {
                if self.octos[x][y].energy > MAX_ENERGY {
                    candidates.push(Coordinate { x, y });
                }
            }
        }
        while !candidates.is_empty() {
            let mut new_candidates = Vec::new();
            for c in &candidates {
                if self.octos[c.x][c.y].energy > MAX_ENERGY && !self.octos[c.x][c.y].flashed {
                    flash_count += 1;
                    let adjacent = self.flash(c);
                    new_candidates.extend(adjacent);
                }
            }
            candidates = new_candidates;
        }
        flash_count
    }
}

fn calculate_flashes(field: &mut Field, steps: i32) -> i32 {
    let mut total_flashes = 0;
    for _ in 1..=steps {
        field.step_increment();
        total_flashes += field.step_flash();
        field.reset_flashed();
    }
    total_flashes
}

fn calculate_all_flash(field: &mut Field) -> i32 {
    for i in 1.. {
        field.step_increment();
        let flashes = field.step_flash();
        if flashes == (FIELD_SIZE * FIELD_SIZE) as i32 {
            return i as i32;
        }
        field.reset_flashed();
    }
    panic!("We should never get here");
}

fn get_field(reader: BufReader<File>) -> Field {
    let field: Field = Field {
        octos: reader
            .lines()
            .map(|l| l.unwrap())
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .map(|i| Octopus {
                        energy: i,
                        flashed: false,
                    })
                    .collect::<Vec<Octopus>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[Octopus; FIELD_SIZE]>>()
            .try_into()
            .unwrap(),
    };
    field
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut field = get_field(reader);

    println!("Part 1: {}", calculate_flashes(&mut field, 100));
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut field = get_field(reader);
    println!("Part 2: {}", calculate_all_flash(&mut field));
}
