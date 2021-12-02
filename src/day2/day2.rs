use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct MoveParseError;

enum Move {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        let direction = split[0];
        let units = match split[1].parse::<i32>() {
            Ok(units) => units,
            Err(_) => return Err(MoveParseError),
        };
        match direction {
            "forward" => Ok(Move::Forward(units)),
            "up" => Ok(Move::Up(units)),
            "down" => Ok(Move::Down(units)),
            _ => Err(MoveParseError),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn make_move(&mut self, _move: &Move) {
        match _move {
            Move::Forward(units) => self.x += units,
            Move::Up(units) => self.y += units,
            Move::Down(units) => self.y -= units,
        }
    }

    fn get_result(&self) -> i32 {
        self.x * -self.y
    }
}

struct Submarine {
    pos: Position,
    aim: i32,
}

impl Submarine {
    fn make_move(&mut self, _move: &Move) {
        match _move {
            Move::Forward(units) => {
                self.pos.x += units;
                self.pos.y -= self.aim * units;
            },
            Move::Up(units) => self.aim -= units,
            Move::Down(units) => self.aim += units,
        }
    }

    fn get_result(&self) -> i32 {
        self.pos.get_result()
    }
}

fn calculate_position(moves: &Vec<Move>) -> i32 {
    let mut position = Position { x: 0, y: 0 };
    for _move in moves {
        position.make_move(_move);
    }
    position.get_result()
}

fn calculate_position_part2(moves: &Vec<Move>) -> i32 {
    let mut sub = Submarine {pos: Position {x: 0, y: 0}, aim: 0 };
    for _move in moves {
        sub.make_move(_move);
    }
    sub.get_result()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let moves = reader
        .lines()
        .map(|l| Move::from_str(&l.unwrap()).unwrap())
        .collect();
    println!("Part 1: {}", calculate_position(&moves));
    println!("Part 2: {}", calculate_position_part2(&moves));
}
