use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use regex::Regex;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<String, Box<dyn Error>> {
    todo!()
}

/// Part 2

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<String, Box<dyn Error>> {
    todo!()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day06_example")?;
    // let lines = read_input("src/input/day06")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day06_example")?;
    // let lines = read_input("src/input/day06")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
