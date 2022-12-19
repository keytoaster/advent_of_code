use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<usize, Box<dyn Error>> {

    
    Ok(0)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<usize, Box<dyn Error>> {

    
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day13")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day13")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
