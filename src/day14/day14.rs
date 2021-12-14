use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone)]
struct Polymer {
    pairs: HashMap<String, i64>,
}

impl Polymer {
    fn pairs(s: &String) -> Vec<&str> {
        let mut vec = Vec::new();
        for i in 0..(s.len() - 1) {
            vec.push(&s[i..i + 2]);
        }
        vec
    }

    fn new_chain(s: &String, replacements: &HashMap<String, String>) -> String {
        let mut new_s = String::new();
        new_s += &s[0..1];
        new_s += &replacements[s];
        new_s += &s[1..2];
        new_s
    }
}

fn letter_counts(pairs: HashMap<String, i64>) -> HashMap<String, i64> {
    let mut letter_counts = HashMap::new();
    for (pair, count) in pairs {
        let first = &pair[0..1];
        let second = &pair[1..2];
        *letter_counts.entry(first.to_string()).or_insert(0) += count;
        *letter_counts.entry(second.to_string()).or_insert(0) += count;
    }
    for (_, count) in &mut letter_counts {
        *count = (*count as f64 / 2.0).ceil() as i64;
    }
    letter_counts
}

fn do_replacements(mut polymer: Polymer, replacements: &HashMap<String, String>, n: i32) -> i64 {
    for _ in 1..=n {
        let mut new_pairs = HashMap::new();
        for (pair, count) in polymer.pairs {
            let new_chain = Polymer::new_chain(&pair, replacements);
            for new_pair in Polymer::pairs(&new_chain) {
                *new_pairs.entry(new_pair.to_string()).or_insert(0) += count;
            }
        }
        polymer.pairs = new_pairs;
    }
    let letter_counts = letter_counts(polymer.pairs);
    let max = letter_counts
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, v)| v)
        .unwrap();
    let min = letter_counts
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, v)| v)
        .unwrap();
    max - min
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut input_lines = reader.lines().map(|l| l.unwrap()).into_iter();

    let s = input_lines.next().unwrap();
    let pairs = Polymer::pairs(&s);
    let mut map = HashMap::new();
    for pair in pairs {
        *map.entry(pair.to_string()).or_insert(0) += 1;
    }
    let polymer = Polymer { pairs: map };
    input_lines.next();

    let mut replacements = HashMap::new();
    for line in input_lines {
        let split: Vec<&str> = line.split(" -> ").collect();
        replacements.insert(split[0].to_string(), split[1].to_string());
    }
    let polymer2 = polymer.clone();
    println!("Part 1: {}", do_replacements(polymer, &replacements, 10));
    println!("Part 2: {}", do_replacements(polymer2, &replacements, 40));
}
