use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn item_to_score(item: &char) -> u32 {
    if item.is_lowercase() {
        *item as u32 - 'a' as u32 + 1
    } else {
        *item as u32 - 'A' as u32 + 27
    }
}

fn run_rucksack(line: &str) -> u32 {
    let num_items = line.len();
    let comp1: HashSet<char> = line.chars().take(num_items / 2).collect();
    let comp2: HashSet<char> = line.chars().skip(num_items / 2).collect();
    let common_item = (&comp1 & &comp2).into_iter().next().unwrap();
    item_to_score(&common_item)
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    Ok(input.map(|x| run_rucksack(&x.unwrap())).sum())
}

/// Part 2

fn run_rucksack2(rucksacks: Vec<String>) -> u32 {
    let mut iter = rucksacks.iter().map(|r| r.chars().collect::<HashSet<char>>());
    let common_item = iter.next().map(|rucksack| iter.fold(rucksack, |set1, set2| &set1 & &set2)).unwrap().into_iter().next().unwrap();
    item_to_score(&common_item)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    Ok(input.chunks(3).into_iter().map(|rucksacks| {
        run_rucksack2(rucksacks.map(|x| x.unwrap()).collect())
    }).sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day03")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day03")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
