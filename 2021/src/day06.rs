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

fn part1(mut input: io::Lines<io::BufReader<File>>, days: u32) -> Result<u64, Box<dyn Error>> {
    let mut buckets = [0; 9];

    let fish = input.next().unwrap()?;
    for f in fish.split(',').map(|s| s.parse::<usize>().unwrap()) {
        buckets[f] += 1;        
    }

    for day in 0..days {
        let new_fish = buckets[0];

        for i in 1..buckets.len() {
            buckets[i-1] = buckets[i];
        }

        buckets[6] += new_fish;
        buckets[8] = new_fish;

        println!("After {} days: {:?}", day + 1, buckets);
    }

    Ok(buckets.iter().sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day06")?;
    println!("Part 1: {}", part1(lines, 80)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day06")?;
    println!("Part 2: {}", part1(lines, 256)?);

    Ok(())
}
