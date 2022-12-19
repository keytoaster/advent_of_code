mod day18_parser_unsafe;

use day18_parser_unsafe::SnailfishNumber;
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

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    // let (_, mut acc) = SnailfishNumber::from(&input.next().unwrap()?).unwrap();
    // println!("{:?}", acc);

    // for line in input {
    //     let line = line?;

    //     let (_, number) = SnailfishNumber::from(&line).unwrap();

    //     acc = &acc + &number;

    //     // println!("After add: {:?}", acc);
    // }

    // Ok(acc.magnitude())
    Ok(0)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    // let mut numbers = Vec::new();

    // for line in input {
    //     let line = line?;
    //     let (_, number) = SnailfishNumber::from(&line).unwrap();
    //     numbers.push(number);
    // }

    // let mut largest_mag = 0;

    // for i in 0..numbers.len() {
    //     for j in 0..numbers.len() {
    //         if i == j {
    //             continue;
    //         }

    //         let mag = (&numbers[i] + &numbers[j]).magnitude();

    //         if mag > largest_mag {
    //             println!(
    //                 "Better solution: {} using {:?} and {:?}",
    //                 mag, numbers[i], numbers[j]
    //             );
    //             largest_mag = mag;
    //         }
    //     }
    // }

    // Ok(largest_mag)
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day18")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day18")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
