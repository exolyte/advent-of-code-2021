//Warning: this is very, very bad code, written while sleep deprived
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::slice::Iter;
use std::str::FromStr;
use std::{env};
use std::iter::FromIterator;

#[derive(Debug)]
struct SignalParseError;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl FromStr for Signal {
    type Err = SignalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Signal::A),
            "b" => Ok(Signal::B),
            "c" => Ok(Signal::C),
            "d" => Ok(Signal::D),
            "e" => Ok(Signal::E),
            "f" => Ok(Signal::F),
            "g" => Ok(Signal::G),
            _ => Err(SignalParseError),
        }
    }
}

impl Signal {
    fn iterator() -> Iter<'static, Signal> {
        static SIGNALS: [Signal; 7] = [
            Signal::A,
            Signal::B,
            Signal::C,
            Signal::D,
            Signal::E,
            Signal::F,
            Signal::G,
        ];
        SIGNALS.iter()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Segment {
    UpperLeft,
    LowerLeft,
    UpperMiddle,
    CenterMiddle,
    LowerMiddle,
    UpperRight,
    LowerRight,
}

struct SignalPattern {
    pattern: Vec<Signal>,
}

impl SignalPattern {
    fn signal_count(&self) -> i32 {
        self.pattern.len() as i32
    }

    fn has_unique_signal_count(&self) -> bool {
        match self.signal_count() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        }
    }

    fn contains_all_signals(&self, other: &SignalPattern) -> bool {
        for signal in other.pattern.iter() {
            if !self.pattern.contains(signal) {
                return false;
            }
        }
        true
    }

    fn get_digit(&self, map: &HashMap<Signal, Segment>) -> i32 {
        let mut segments = HashSet::new();
        for signal in &self.pattern {
            segments.insert(*map.get(signal).unwrap());
        }
        let zero = HashSet::from([
            Segment::LowerRight,
            Segment::UpperRight,
            Segment::LowerLeft,
            Segment::UpperLeft,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
        ]);
        let one = HashSet::from([Segment::LowerRight, Segment::UpperRight]);
        let two = HashSet::from([
            Segment::UpperRight,
            Segment::LowerLeft,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
            Segment::CenterMiddle,
        ]);
        let three = HashSet::from([
            Segment::LowerRight,
            Segment::UpperRight,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
            Segment::CenterMiddle,
        ]);
        let four = HashSet::from([
            Segment::LowerRight,
            Segment::UpperRight,
            Segment::UpperLeft,
            Segment::CenterMiddle,
        ]);
        let five = HashSet::from([
            Segment::LowerRight,
            Segment::UpperLeft,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
            Segment::CenterMiddle,
        ]);
        let six = HashSet::from([
            Segment::LowerRight,
            Segment::LowerLeft,
            Segment::UpperLeft,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
            Segment::CenterMiddle,
        ]);
        let seven = HashSet::from([
            Segment::LowerRight,
            Segment::UpperRight,
            Segment::UpperMiddle,
        ]);
        let eight = HashSet::from([
            Segment::LowerRight,
            Segment::UpperRight,
            Segment::LowerLeft,
            Segment::UpperLeft,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
            Segment::CenterMiddle,
        ]);
        let nine = HashSet::from([
            Segment::LowerRight,
            Segment::UpperRight,
            Segment::UpperLeft,
            Segment::UpperMiddle,
            Segment::LowerMiddle,
            Segment::CenterMiddle,
        ]);
        if segments == zero {
            return 0;
        }
        if segments == one {
            return 1;
        }
        if segments == two {
            return 2;
        }
        if segments == three {
            return 3;
        }
        if segments == four {
            return 4;
        }
        if segments == five {
            return 5;
        }
        if segments == six {
            return 6;
        }
        if segments == seven {
            return 7;
        }
        if segments == eight {
            return 8;
        }
        if segments == nine {
            return 9;
        }
        panic!("No digit found!!!")
    }
}

impl FromIterator<Signal> for SignalPattern {
    fn from_iter<T: IntoIterator<Item = Signal>>(iter: T) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(item);
        }
        SignalPattern { pattern: vec }
    }
}

struct SignalIO {
    input: Vec<SignalPattern>,
    output: Vec<SignalPattern>,
    signal_segment: HashMap<Signal, Segment>,
    segment_signal: HashMap<Segment, Signal>,
}

impl SignalIO {
    fn get_unique_digit_count(&self) -> i32 {
        self.output
            .iter()
            .filter(|sp| sp.has_unique_signal_count())
            .count() as i32
    }

    fn get_number(&self, number: i32) -> &SignalPattern {
        match number {
            1 => self.get_numbers_with_length(2)[0],
            2 => self
                .get_numbers_with_length(5)
                .iter()
                .filter(|p| {
                    p.pattern
                        .contains(self.segment_signal.get(&Segment::LowerLeft).unwrap())
                })
                .next()
                .unwrap(),
            4 => self.get_numbers_with_length(4)[0],
            5 => self
                .get_numbers_with_length(5)
                .iter()
                .filter(|p| {
                    !p.pattern
                        .contains(self.segment_signal.get(&Segment::UpperRight).unwrap())
                })
                .next()
                .unwrap(),
            6 => self
                .get_numbers_with_length(6)
                .iter()
                .filter(|p| {
                    !p.pattern
                        .contains(self.segment_signal.get(&Segment::UpperRight).unwrap())
                })
                .next()
                .unwrap(),
            7 => self.get_numbers_with_length(3)[0],
            8 => self.get_numbers_with_length(7)[0],
            9 => self
                .get_numbers_with_length(6)
                .iter()
                .filter(|p| p.contains_all_signals(self.get_number(4)))
                .next()
                .unwrap(),
            _ => panic!("Unknown number!"),
        }
    }

    fn get_numbers_with_length(&self, n: usize) -> Vec<&SignalPattern> {
        self.input
            .iter()
            .filter(|p| p.pattern.len() == n)
            .map(|s| s)
            .collect()
    }

    fn get_lower_right(&self) -> Signal {
        for &signal in Signal::iterator() {
            let mut count = 0;
            for pattern in &self.input {
                if !pattern.pattern.contains(&signal) {
                    count += 1;
                }
            }
            if count == 1 {
                return signal;
            }
        }
        panic!("No lower right found!!!")
    }

    fn get_upper_right(&self, lower_right: Signal) -> Signal {
        for pattern in &self.input {
            if pattern.pattern.len() == 2 {
                for &signal in &pattern.pattern {
                    if signal != lower_right {
                        return signal;
                    }
                }
            }
        }
        panic!("No upper right found!!")
    }

    fn get_upper_middle(&self, lower_right: Signal, upper_right: Signal) -> Signal {
        for pattern in &self.input {
            if pattern.pattern.len() == 3 {
                for &signal in &pattern.pattern {
                    if signal != lower_right && signal != upper_right {
                        return signal;
                    }
                }
            }
        }
        panic!("No upper right found!!")
    }

    fn get_lower_left(&self) -> Signal {
        let nine = self.get_number(9);
        for &signal in Signal::iterator() {
            if !nine.pattern.contains(&signal) {
                return signal;
            }
        }
        panic!("No lower left found!!")
    }

    fn get_lower_middle(&self) -> Signal {
        let eight = self.get_number(8);
        let four = self.get_number(4);
        for &signal in eight.pattern.iter() {
            if !four.pattern.contains(&signal) {
                if signal != *self.segment_signal.get(&Segment::LowerLeft).unwrap()
                    && signal != *self.segment_signal.get(&Segment::UpperMiddle).unwrap()
                {
                    return signal;
                }
            }
        }
        panic!("No lower middle found!!")
    }

    fn get_upper_left(&self) -> Signal {
        let four = self.get_number(4);
        let candidates: Vec<Signal> = four
            .pattern
            .iter()
            .filter(|&&s| {
                s != *self.segment_signal.get(&Segment::UpperRight).unwrap()
                    && s != *self.segment_signal.get(&Segment::LowerRight).unwrap()
            })
            .map(|&s| s)
            .collect();
        let two = self.get_number(2);
        for candidate in candidates {
            if !two.pattern.contains(&candidate) {
                return candidate;
            }
        }
        panic!("No upper left found!!")
    }

    fn get_center_middle(&self) -> Signal {
        let four = self.get_number(4);
        let candidates: Vec<Signal> = four
            .pattern
            .iter()
            .filter(|&&s| {
                s != *self.segment_signal.get(&Segment::UpperRight).unwrap()
                    && s != *self.segment_signal.get(&Segment::LowerRight).unwrap()
            })
            .map(|&s| s)
            .collect();
        let two = self.get_number(2);
        for candidate in candidates {
            if two.pattern.contains(&candidate) {
                return candidate;
            }
        }
        panic!("No center middle found!!")
    }

    fn get_result(&self) -> i32 {
        let th = self.output[0].get_digit(&self.signal_segment) * 1000;
        let hu = self.output[1].get_digit(&self.signal_segment) * 100;
        let te = self.output[2].get_digit(&self.signal_segment) * 10;
        let on = self.output[3].get_digit(&self.signal_segment) * 1;
        th + hu + te + on
    }
}

fn unique_pattern_amount(io: &Vec<SignalIO>) -> i32 {
    io.into_iter().map(|io| io.get_unique_digit_count()).sum()
}

fn calculate_digits(ios: &mut Vec<SignalIO>) -> i32 {
    let mut sum = 0;
    for io in ios {
        let lower_right = io.get_lower_right();
        io.signal_segment.insert(lower_right, Segment::LowerRight);
        io.segment_signal.insert(Segment::LowerRight, lower_right);
        let upper_right = io.get_upper_right(lower_right);
        io.signal_segment.insert(upper_right, Segment::UpperRight);
        io.segment_signal.insert(Segment::UpperRight, upper_right);
        let upper_middle = io.get_upper_middle(lower_right, upper_right);
        io.signal_segment.insert(upper_middle, Segment::UpperMiddle);
        io.segment_signal.insert(Segment::UpperMiddle, upper_middle);
        let lower_left = io.get_lower_left();
        io.signal_segment.insert(lower_left, Segment::LowerLeft);
        io.segment_signal.insert(Segment::LowerLeft, lower_left);
        let lower_middle = io.get_lower_middle();
        io.signal_segment.insert(lower_middle, Segment::LowerMiddle);
        io.segment_signal.insert(Segment::LowerMiddle, lower_middle);
        let upper_left = io.get_upper_left();
        io.signal_segment.insert(upper_left, Segment::UpperLeft);
        io.segment_signal.insert(Segment::UpperLeft, upper_left);
        let center_middle = io.get_center_middle();
        io.signal_segment
            .insert(center_middle, Segment::CenterMiddle);
        io.segment_signal
            .insert(Segment::CenterMiddle, center_middle);
        sum += io.get_result();
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut io = Vec::new();
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let input_patterns: Vec<SignalPattern> = parts[0]
            .split_whitespace()
            .map(|p| {
                p.chars()
                    .map(|c| Signal::from_str(c.to_string().as_str()).unwrap())
                    .collect()
            })
            .collect();
        let output_digits: Vec<SignalPattern> = parts[1]
            .split_whitespace()
            .map(|p| {
                p.chars()
                    .map(|c| Signal::from_str(c.to_string().as_str()).unwrap())
                    .collect()
            })
            .collect();
        io.push(SignalIO {
            input: input_patterns,
            output: output_digits,
            signal_segment: HashMap::new(),
            segment_signal: HashMap::new(),
        })
    }

    println!("Part 1: {}", unique_pattern_amount(&io));
    println!("Part 2: {}", calculate_digits(&mut io));
}
