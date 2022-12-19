
mod day16_parser;

use day16_parser::hex2bin;
use day16_parser::packet;
use day16_parser::Packet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_version(p: &Packet) -> u32 {
    match p {
        Packet::Literal { metadata: m, .. } => m.version.into(),
        Packet::Sum {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
        Packet::Product {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
        Packet::Minimum {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
        Packet::Maximum {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
        Packet::GreaterThan {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
        Packet::LessThan {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
        Packet::EqualTo {
            metadata: m,
            packet: pp,
        } => {
            m.version as u32
                + pp.subpackets
                    .iter()
                    .map(|sp| count_version(sp) as u32)
                    .sum::<u32>()
        },
    }
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let line = input.next().unwrap()?;
    let line = hex2bin(&line)?;

    let (_, parsed) = packet(&line).unwrap();

    Ok(count_version(&parsed))
}

fn eval(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { packet: p, .. } => {
            p.value
        },
        Packet::Sum { packet: p, ..} => {
            p.subpackets.iter().map(eval).sum()
        },
        Packet::Product { packet: p, ..} => {
            p.subpackets.iter().fold(1, |acc, i| acc * eval(i))
        },
        Packet::Minimum { packet: p, ..} => {
            p.subpackets.iter().map(eval).min().unwrap()
        },
        Packet::Maximum { packet: p, ..} => {
            p.subpackets.iter().map(eval).max().unwrap()
        },
        Packet::GreaterThan { packet: p, ..} => {
            if eval(&p.subpackets[0]) > eval(&p.subpackets[1]) {
                1
            } else {
                0
            }
        },
        Packet::LessThan { packet: p, ..} => {
            if eval(&p.subpackets[0]) < eval(&p.subpackets[1]) {
                1
            } else {
                0
            }
        },
        Packet::EqualTo { packet: p, ..} => {
            if eval(&p.subpackets[0]) == eval(&p.subpackets[1]) {
                1
            } else {
                0
            }
        },
    }
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    let line = input.next().unwrap()?;
    let line = hex2bin(&line)?;

    let (_, parsed) = packet(&line).unwrap();

    Ok(eval(&parsed))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day16")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day16")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
