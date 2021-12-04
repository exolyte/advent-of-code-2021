use std::cell::Cell;
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_NUMBER: i32 = 99;
const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct BingoNumber {
    n: i32,
    hit: Cell<bool>,
}

struct Board<'a> {
    lines: [[&'a BingoNumber; BOARD_SIZE]; BOARD_SIZE],
}

impl Board<'_> {
    fn bingo(&self) -> bool {
        for line in self.lines {
            let bingo = line.iter().all(|bn| bn.hit.get());
            if bingo {
                return bingo;
            }
        }
        for i in 0..BOARD_SIZE {
            let bingo = self.lines.iter().all(|l| l[i].hit.get());
            if bingo {
                return bingo;
            }
        }
        false
    }

    fn get_unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for line in self.lines {
            for bn in line {
                if !bn.hit.get() {
                    sum += bn.n;
                }
            }
        }
        sum
    }
}

fn win_bingo(numbers: &Vec<BingoNumber>, input_numbers: &Vec<usize>, boards: &Vec<Board>) -> i32 {
    for &number in input_numbers {
        numbers[number].hit.set(true);
        let winning_boards: Vec<&Board> = boards.into_iter().filter(|b| b.bingo()).collect();
        if winning_boards.len() == 1 {
            return winning_boards[0].get_unmarked_sum() * number as i32;
        }
    }
    panic!("No (single) winning board!")
}

fn lose_bingo(numbers: &Vec<BingoNumber>, input_numbers: &Vec<usize>, boards: &Vec<Board>) -> i32 {
    let mut leftover_boards: Vec<&Board> = boards.iter().collect();
    let mut winning_result = 0;
    for &number in input_numbers {
        numbers[number].hit.set(true);
        for board in &leftover_boards {
            if board.bingo() {
                winning_result = board.get_unmarked_sum() * number as i32;
            }
        }
        leftover_boards = leftover_boards.into_iter().filter(|b| !b.bingo()).collect();
    }
    winning_result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();
    for n in 0..=MAX_NUMBER {
        numbers.push(BingoNumber {
            n: n,
            hit: Cell::new(false),
        })
    }
    let mut input_lines = reader.lines().map(|l| l.unwrap());
    let input_numbers: Vec<usize> = input_lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    input_lines.next();

    let mut board = Board {
        lines: [[&numbers[0]; BOARD_SIZE]; BOARD_SIZE],
    };
    let mut i = 0;
    let mut boards = Vec::new();
    for line in input_lines {
        if line == "" {
            boards.push(board);
            board = Board {
                lines: [[&numbers[0]; BOARD_SIZE]; BOARD_SIZE],
            };
            i = 0;
            continue;
        }
        let line_numbers = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());
        board.lines[i] = line_numbers
            .map(|n| &numbers[n])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        i += 1;
    }
    boards.push(board);

    println!("Part 1: {}", win_bingo(&numbers, &input_numbers, &boards));
    println!("Part 2: {}", lose_bingo(&numbers, &input_numbers, &boards));
}
