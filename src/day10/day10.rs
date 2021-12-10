use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

#[derive(Clone, Copy, PartialEq)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

enum BracketState {
    Open,
    Closed,
}

struct Bracket(BracketType, BracketState);

impl FromStr for Bracket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Bracket(BracketType::Round, BracketState::Open)),
            "[" => Ok(Bracket(BracketType::Square, BracketState::Open)),
            "{" => Ok(Bracket(BracketType::Curly, BracketState::Open)),
            "<" => Ok(Bracket(BracketType::Angle, BracketState::Open)),
            ")" => Ok(Bracket(BracketType::Round, BracketState::Closed)),
            "]" => Ok(Bracket(BracketType::Square, BracketState::Closed)),
            "}" => Ok(Bracket(BracketType::Curly, BracketState::Closed)),
            ">" => Ok(Bracket(BracketType::Angle, BracketState::Closed)),
            _ => Err(ParseError),
        }
    }
}

impl BracketType {
    fn get_corrupt_score(&self) -> i32 {
        match self {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }

    fn get_complete_score(&self) -> i32 {
        match self {
            BracketType::Round => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        }
    }
}

struct Line(Vec<Bracket>);

impl Line {
    fn get_vec(&self) -> &Vec<Bracket> {
        let Line(brackets) = self;
        brackets
    }

    fn get_corrupted_bracket(&self) -> Option<BracketType> {
        let mut stack: Vec<BracketType> = Vec::new();
        for Bracket(btype, bstate) in self.get_vec() {
            match *bstate {
                BracketState::Open => stack.push(*btype),
                BracketState::Closed => {
                    let top = stack.pop();
                    match top {
                        Some(b) => {
                            if b != *btype {
                                return Some(*btype);
                            }
                        }
                        None => return Some(*btype),
                    }
                }
            }
        }
        None
    }

    fn get_missing_brackets(&self) -> Option<Vec<BracketType>> {
        let mut stack: Vec<BracketType> = Vec::new();
        for Bracket(btype, bstate) in self.get_vec() {
            match *bstate {
                BracketState::Open => stack.push(*btype),
                BracketState::Closed => {
                    let top = stack.pop();
                    match top {
                        Some(b) => {
                            if b != *btype {
                                return None;
                            }
                        }
                        None => return None,
                    }
                }
            }
        }
        stack.reverse();
        Some(stack)
    }
}

fn calculate_corrupt_score(lines: &Vec<Line>) -> i32 {
    let mut sum = 0;
    for line in lines {
        match line.get_corrupted_bracket() {
            Some(b) => sum += b.get_corrupt_score(),
            None => (),
        }
    }
    sum
}

fn calculate_complete_score(lines: &Vec<Line>) -> i64 {
    let mut sums: Vec<i64> = Vec::new();
    for line in lines {
        let mut sum = 0;
        match line.get_missing_brackets() {
            Some(bs) => {
                for b in bs {
                    sum = (sum * 5) + b.get_complete_score() as i64;
                }
            }
            None => (),
        }
        if sum != 0 {
            sums.push(sum);
        }
    }
    sums.sort();
    sums[sums.len() / 2]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Line> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<Bracket>().unwrap())
                .collect::<Vec<Bracket>>()
        })
        .map(|v| Line(v))
        .collect();

    println!("Part 1: {}", calculate_corrupt_score(&lines));
    println!("Part 2: {}", calculate_complete_score(&lines));
}
