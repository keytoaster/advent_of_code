use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::env;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut prev = u32::MAX;
    let mut num_increases = 0;

    for line in input {
        let line = line?.parse::<u32>().unwrap();
        if line > prev {
            num_increases += 1;
        }
        prev = line;
    }

    Ok(num_increases)
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut prevs = [u32::MAX; 3];
    let mut prevs_pointer = 0;
    let mut prevs_sum;
    
    let mut num_increases = 0;

    // Unroll the first 3 loop iterations to save a branch.
    prevs[0] = input.next().ok_or("Not enough inputs")??.parse::<u32>().unwrap();
    prevs[1] = input.next().ok_or("Not enough inputs")??.parse::<u32>().unwrap();
    prevs[2] = input.next().ok_or("Not enough inputs")??.parse::<u32>().unwrap();
    prevs_sum = prevs[0] + prevs[1] + prevs[2];

    for line in input {
        let line = line?.parse::<u32>().unwrap();

        let cur_sum = prevs_sum - prevs[prevs_pointer] + line;

        if cur_sum > prevs_sum {
            num_increases += 1;
        }

        prevs[prevs_pointer] = line;
        prevs_pointer = (prevs_pointer + 1) % 3;
        prevs_sum = cur_sum;
    }
    
    Ok(num_increases)
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day01")?;
    println!("Part 1: Num increases: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day01")?;
    println!("Part 2: Num increases: {}", part2(lines)?);

    Ok(())
}
