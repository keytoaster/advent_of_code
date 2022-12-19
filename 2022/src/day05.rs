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

struct State {
    stacks: Vec<Vec<char>>,
}

fn parse_initial_state(input: &mut io::Lines<io::BufReader<File>>) -> State {

    stacks = Vec::new();

    
    let mut iter = input.into_iter().peekable();

    let len = println!("peek: {:?}", iter.peek().unwrap().as_ref().unwrap().len() / 4 + 1);




    for i in iter {
        println!("line: {:?}", i);
    }
    
    

    let stacks = Vec::new();
    // stacks.push

    State { stacks }
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let state = parse_initial_state(&mut input);

    Ok(0)
}

/// Part 2

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    todo!()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day05_example")?;
    // let lines = read_input("src/input/day05")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day05_example")?;
    // let lines = read_input("src/input/day05")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
