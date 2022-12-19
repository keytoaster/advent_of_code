use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::env;
use regex::Regex;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut horizontal = 0;
    let mut depth = 0;
    let re = Regex::new(r"(\w+) (\d+)").unwrap();


    for line in input {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let units = caps.get(2).unwrap().as_str().parse::<u32>()?;

        match command {
            "forward" => {
                horizontal += units;
            }
            "down" => {
                depth += units;
            }
            "up" => {
                depth -= units;
            }
            _ => {
                return Err("unsupported command".into());
            }
        }
    }

    Ok(horizontal * depth)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let re = Regex::new(r"(\w+) (\d+)").unwrap();


    for line in input {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let units = caps.get(2).unwrap().as_str().parse::<u32>()?;

        match command {
            "forward" => {
                horizontal += units;
                depth += aim * units;
            }
            "down" => {
                aim += units;
            }
            "up" => {
                aim -= units;
            }
            _ => {
                return Err("unsupported command".into());
            }
        }
    }

    Ok(horizontal * depth)
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day02")?;
    println!("Part 1: Num increases: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day02")?;
    println!("Part 2: Num increases: {}", part2(lines)?);

    Ok(())
}
