use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::sequence::tuple;
use nom::IResult;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Range {
    a: u32,
    b: u32,
}

fn parse_run(input: &str) -> IResult<&str, (Range, Range)> {
    let (input, (r1_a, _, r1_b, _, r2_a, _, r2_b)) =
        tuple((u32, tag("-"), u32, tag(","), u32, tag("-"), u32))(input)?;
    Ok((
        input,
        (Range { a: r1_a, b: r1_b }, Range { a: r2_a, b: r2_b }),
    ))
}

fn do_run(range1: &Range, range2: &Range) -> bool {
    (range2.a >= range1.a && range2.b <= range1.b) || (range1.a >= range2.a && range1.b <= range2.b)
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    Ok(input.filter_map(|line| {
        parse_run(&line.unwrap()).and_then(
            |(_, (range1, range2))| {
                Ok(do_run(&range1, &range2) as u32)
            }).ok()
    }).sum())
}

/// Part 2

fn do_run2(range1: &Range, range2: &Range) -> bool {
    // Ranges are overlapping unless one is entirely left or right of the other.
    !((range2.a < range1.a && range2.b < range1.a) || (range2.a > range1.b && range2.b > range1.b))
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    Ok(input.filter_map(|line| {
        parse_run(&line.unwrap()).and_then(
            |(_, (range1, range2))| {
                Ok(do_run2(&range1, &range2) as u32)
            }).ok()
    }).sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day04")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day04")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
