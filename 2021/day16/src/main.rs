use anyhow::{Error, Result};
use std::io::prelude::*;
use std::io::Cursor;
use std::str;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Packet {
    version: u64,
    type_id: u64,
    packets: Vec<Box<Packet>>,
    literal: Option<u64>,
}

#[derive(Debug)]
struct Stream {
    stream: Cursor<Vec<u8>>,
}

impl FromStr for Stream {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let binary_stream = binary_from_hex(s);
        let stream = Cursor::new(binary_stream.as_bytes().to_vec());
        Ok(Stream { stream })
    }
}
fn binary_from_hex(s: &str) -> String {
    let mut binary = String::new();
    for c in s.chars() {
        binary.push_str(&hex_to_binary(&c.to_string()));
    }
    binary
}

impl Stream {
    fn read(&mut self, buf: &mut [u8]) -> u64 {
        let _ = self.stream.read(buf).unwrap();
        let string = str::from_utf8(buf).unwrap();
        u64::from_str_radix(string, 2).unwrap()
    }

    fn read_literal(&mut self) -> u64 {
        let mut output = String::new();
        let mut more = 1;
        while more == 1 {
            more = self.read(&mut [0; 1]);
            let piece = self.read(&mut [0; 4]);
            output.push_str(&format!("{:04b}", piece));
        }
        u64::from_str_radix(&output, 2).unwrap()
    }

    fn position(&mut self) -> u64 {
        self.stream.position()
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read(&mut [0; 3]);
        let type_id = self.read(&mut [0; 3]);
        let mut packets = vec![];
        let mut literal = None;
        if type_id == 4 {
            literal = Some(self.read_literal());
        } else {
            let length_type_id = self.read(&mut [0; 1]);
            if length_type_id == 0 {
                let total_length = self.read(&mut [0; 15]);
                let start = self.position();
                while self.position() - start < total_length {
                    let packet = self.read_packet();
                    packets.push(Box::new(packet));
                }
            } else {
                let packet_count = self.read(&mut [0; 11]);
                for _ in 0..packet_count {
                    let packet = self.read_packet();
                    packets.push(Box::new(packet));
                }
            }
        }
        Packet {
            version,
            type_id,
            packets,
            literal,
        }
    }
}

fn hex_to_binary(s: &str) -> String {
    format!("{:04b}", u8::from_str_radix(s, 16).unwrap())
}

fn int_from_binary(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

fn parse_hex(hex: &str) -> Packet {
    let mut stream = Stream::from_str(hex).unwrap();
    stream.read_packet()
}

fn part1(packet: Packet) -> u64 {
    let mut version_sum = packet.version;
    if packet.type_id != 4 {
        for p in packet.packets {
            version_sum += part1(*p);
        }
    }
    version_sum
}

fn eval(packet: Packet) -> u64 {
    match packet.type_id {
        0 => packet.packets.iter().map(|p| eval(*p.clone())).sum(),
        1 => packet.packets.iter().map(|p| eval(*p.clone())).product(),
        2 => packet
            .packets
            .iter()
            .map(|p| eval(*p.clone()))
            .min()
            .unwrap(),
        3 => packet
            .packets
            .iter()
            .map(|p| eval(*p.clone()))
            .max()
            .unwrap(),
        4 => packet.literal.unwrap(),
        5 => {
            if eval((*packet.packets[0]).clone()) > eval(*(packet.packets[1]).clone()) {
                1
            } else {
                0
            }
        }
        6 => {
            if eval((*packet.packets[0]).clone()) < eval(*(packet.packets[1]).clone()) {
                1
            } else {
                0
            }
        }
        7 => {
            if eval((*packet.packets[0]).clone()) == eval(*(packet.packets[1]).clone()) {
                1
            } else {
                0
            }
        }
        _ => panic!("unknown type id"),
    }
}

fn part2(packet: Packet) {
    dbg!(eval(packet));
}

fn main() {
    // let test = "D2FE28";
    // assert_eq!(binary_from_hex(test), "110100101111111000101000");
    // assert_eq!(int_from_binary("110"), 6);
    // parse_hex(test);
    // let test2 = "38006F45291200";
    // parse_hex(test2);
    // let test3 = "A0016C880162017C3686B18A3D4780";
    // let packet = parse_hex(test3);
    let input = include_str!("../input.txt");
    let packet = parse_hex(input);
    // dbg!(part1(packet));
    part2(packet);
}
