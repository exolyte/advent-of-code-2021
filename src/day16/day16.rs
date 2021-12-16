use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::{env, panic};

const LTYPE_PACKET_COUNT: bool = true;
const LTYPE_PACKET_SIZE: bool = false;
const PTYPE_LITERAL: i64 = 4;

trait Packet {
    fn eval(&self) -> i64;
    fn packet_size(&self) -> i32;
    fn total_version(&self) -> i32;
}

struct LiteralPacket {
    version: i32,
    _ptype: i32,
    value: i64,
    packet_size: i32,
}

impl LiteralPacket {
    fn new(bits: &Vec<bool>) -> Self {
        let version = BitVector::bits_to_decimal(&bits[0..3]) as i32;
        let ptype = BitVector::bits_to_decimal(&bits[3..6]) as i32;
        let mut bitnumber = Vec::new();
        let mut i = 6;
        loop {
            bitnumber.extend(&bits[i + 1..i + 5]);
            if bits[i] == false {
                break;
            }
            i += 5;
        }
        let value = BitVector::bits_to_decimal(&bitnumber);

        let packet_size = (i + 5) as i32;
        LiteralPacket {
            version,
            _ptype: ptype,
            value,
            packet_size,
        }
    }
}

impl Packet for LiteralPacket {
    fn eval(&self) -> i64 {
        self.value
    }
    fn packet_size(&self) -> i32 {
        self.packet_size
    }
    fn total_version(&self) -> i32 {
        self.version
    }
}

struct OperatorPacket {
    version: i32,
    ptype: i32,
    _ltype: bool,
    _length: i32,
    subpackets: Vec<Box<dyn Packet>>,
    packet_size: i32,
}

impl OperatorPacket {
    fn new(bits: &Vec<bool>) -> Self {
        let version = BitVector::bits_to_decimal(&bits[0..3]) as i32;
        let ptype = BitVector::bits_to_decimal(&bits[3..6]) as i32;
        let ltype = bits[6];
        let length_end = match ltype {
            false => 22,
            true => 18,
        };
        let length = BitVector::bits_to_decimal(&bits[7..length_end]) as i32;

        let mut subpackets = Vec::new();
        let mut packet_start = length_end;
        if ltype == LTYPE_PACKET_COUNT {
            for _ in 0..length {
                let packet = create_packet(&bits[packet_start..].to_vec());
                packet_start += packet.packet_size() as usize;
                subpackets.push(packet);
            }
        } else if ltype == LTYPE_PACKET_SIZE {
            loop {
                let packet = create_packet(&bits[packet_start..].to_vec());
                packet_start += packet.packet_size() as usize;
                subpackets.push(packet);
                if packet_start as i32 > (length - 1 + length_end as i32) {
                    break;
                }
            }
        }
        let packet_size = packet_start as i32;
        OperatorPacket {
            version,
            ptype,
            _ltype: ltype,
            _length: length,
            subpackets,
            packet_size,
        }
    }
}

impl Packet for OperatorPacket {
    fn eval(&self) -> i64 {
        match self.ptype {
            0 => self.subpackets.iter().map(|p| p.eval()).sum(),
            1 => self.subpackets.iter().map(|p| p.eval()).product(),
            2 => self.subpackets.iter().map(|p| p.eval()).min().unwrap(),
            3 => self.subpackets.iter().map(|p| p.eval()).max().unwrap(),
            5 => (self.subpackets[0].eval() > self.subpackets[1].eval()) as i64,
            6 => (self.subpackets[0].eval() < self.subpackets[1].eval()) as i64,
            7 => (self.subpackets[0].eval() == self.subpackets[1].eval()) as i64,
            _ => panic!("Invalid packet type"),
        }
    }
    fn packet_size(&self) -> i32 {
        self.packet_size
    }
    fn total_version(&self) -> i32 {
        let mut version = self.version;
        for packet in &self.subpackets {
            version += packet.total_version();
        }
        version
    }
}

struct BitVector;

impl BitVector {
    fn bits_to_decimal(bits: &[bool]) -> i64 {
        let mut number = 0;
        let rev: Vec<&bool> = bits.iter().rev().collect();
        for i in 0..rev.len() {
            number += BitVector::bit_to_int(*rev[i]) as i64 * 2_i64.pow(i as u32);
        }
        number
    }

    fn bit_to_int(bit: bool) -> i32 {
        match bit {
            true => 1,
            false => 0,
        }
    }

    fn char_to_bits(c: char) -> Vec<bool> {
        let mut bv = Vec::new();
        let mut decimal = match c {
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
            _ => c.to_digit(10).unwrap(),
        };
        let mut power = 8;
        loop {
            if decimal >= power {
                bv.push(true);
                decimal -= power;
            } else {
                bv.push(false);
            }
            if power == 1 {
                break;
            }
            power /= 2;
        }
        bv
    }
}

fn create_packet(bits: &Vec<bool>) -> Box<dyn Packet> {
    let ptype = BitVector::bits_to_decimal(&bits[3..6]);
    match ptype {
        PTYPE_LITERAL => Box::new(LiteralPacket::new(bits)),
        _ => Box::new(OperatorPacket::new(bits)),
    }
}

fn calculate_version(bits: &Vec<bool>) -> i32 {
    let packet = create_packet(bits);
    packet.total_version()
}

fn evaluate_packet(bits: &Vec<bool>) -> i64 {
    let packet = create_packet(bits);
    packet.eval()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut bits = Vec::new();
    for c in lines[0].chars() {
        bits.extend(BitVector::char_to_bits(c));
    }

    println!("Part 1: {}", calculate_version(&bits));
    println!("Part 2: {}", evaluate_packet(&bits));
}
