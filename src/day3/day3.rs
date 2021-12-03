use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

const BIT_ARRAY_SIZE: usize = 12;

#[derive(Debug)]
struct ParseError;

struct BitVector([bool; BIT_ARRAY_SIZE]);

impl BitVector {
    fn reverse(&self) -> BitVector {
        BitVector(self.get_array().map(|b| !b))
    }

    fn get_array(&self) -> &[bool; BIT_ARRAY_SIZE] {
        let BitVector(array) = self;
        array
    }

    fn to_string(&self) -> String {
        self.get_array()
            .iter()
            .map(|b| match b {
                true => '1',
                false => '0',
            })
            .collect()
    }

    fn to_decimal(&self) -> i32 {
        i32::from_str_radix(&self.to_string(), 2).unwrap()
    }
}

impl FromStr for BitVector {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: Vec<bool> = s
            .chars()
            .map(|c| match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err(ParseError),
            })
            .collect::<Result<_, _>>()?;
        Ok(BitVector(match bits.try_into() {
            Ok(b) => b,
            Err(_) => return Err(ParseError),
        }))
    }
}

fn get_bit_counts(vectors: &Vec<&BitVector>) -> [i32; BIT_ARRAY_SIZE] {
    let mut bit_counts = [0; BIT_ARRAY_SIZE];
    for vector in vectors {
        let BitVector(array) = vector;
        for (i, bit) in array.iter().enumerate() {
            match bit {
                true => bit_counts[i] += 1,
                false => bit_counts[i] -= 1,
            }
        }
    }
    bit_counts
}

fn calculate_power(vectors: &Vec<BitVector>) -> i32 {
    let bit_counts = get_bit_counts(&vectors.iter().collect());
    let gamma = BitVector(bit_counts.map(|i| i > 0));
    let epsilon = gamma.reverse();
    gamma.to_decimal() * epsilon.to_decimal()
}

fn calculate_ls_components(vectors: &Vec<BitVector>, reverse: bool) -> i32 {
    let mut filtered_vectors: Vec<&BitVector> = vectors.iter().collect();
    for i in 0..BIT_ARRAY_SIZE {
        let bit_counts = get_bit_counts(&filtered_vectors);
        let mut most_common_bit = bit_counts[i] >= 0;
        if reverse {
            most_common_bit = !most_common_bit
        }
        filtered_vectors = filtered_vectors
            .into_iter()
            .filter(|bv| bv.get_array()[i] == most_common_bit)
            .collect();
        if filtered_vectors.len() == 1 {
            return filtered_vectors[0].to_decimal();
        }
    }
    panic!("no point where there was only 1 vector left!")
}

fn calculate_life_support(vectors: &Vec<BitVector>) -> i32 {
    let oxygen = calculate_ls_components(vectors, false);
    let co2 = calculate_ls_components(vectors, true);
    oxygen * co2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|l| l.unwrap().parse::<BitVector>().unwrap())
        .collect();
    println!("Part 1: {}", calculate_power(&input));
    println!("Part 2: {}", calculate_life_support(&input));
}
