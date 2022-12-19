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

fn cost1(crab: usize, candidate:usize) -> usize {
    if crab < candidate {
        candidate - crab
    } else {
        crab - candidate
    }
}

fn cost2(crab: usize, candidate:usize) -> usize {
    let steps = if crab < candidate {
        candidate - crab
    } else {
        crab - candidate
    };

    steps * (steps + 1) / 2
}

fn calc_costs(candidate: usize, crabs: &Vec<usize>, cost_func: fn(crab: usize, target: usize) -> usize) -> usize {
    let mut cost = 0;
    for crab in crabs {
        cost += cost_func(*crab, candidate);        
    }
    cost
}

fn part1(mut input: io::Lines<io::BufReader<File>>, cost_func: fn(crab: usize, target: usize) -> usize) -> Result<usize, Box<dyn Error>> {
    let line = input.next().unwrap()?;
    let crabs: Vec<usize> = line.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    let max_position = crabs.iter().max().unwrap().clone();

    let mut solution_cost = usize::MAX;
    let mut left: usize = 0;
    let mut right: usize = max_position;
    
    while left < right {
        let candidate1 = (left + right) / 2;
        let candidate2 = candidate1 + 1;
        
        let cost1 = calc_costs(candidate1, &crabs, cost_func);
        let cost2 = calc_costs(candidate2, &crabs, cost_func);

        if cost1 < cost2 {
            right = candidate1;
            if cost1 < solution_cost {
                solution_cost = cost1;
                println!("Found cheaper solution at position {} for {}", candidate1, solution_cost);
            }
        } else {
            left = candidate2;
            if cost2 < solution_cost {
                solution_cost = cost2;
                println!("Found cheaper solution at position {} for {}", candidate2, solution_cost);
            }
        }
    }
    
    Ok(solution_cost)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day07")?;
    println!("Part 1: {}", part1(lines, cost1)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day07")?;
    println!("Part 2: {}", part1(lines, cost2)?);

    Ok(())
}
