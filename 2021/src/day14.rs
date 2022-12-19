use regex::Regex;
use std::collections::HashMap;
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

struct PolymerState {
    polymer: String,
    rules: HashMap<(char, char), char>,
}

fn parse_input(input: &mut io::Lines<io::BufReader<File>>) -> PolymerState {
    let polymer = input.next().unwrap().unwrap();

    let line = input.next().unwrap().unwrap();
    if line != "" {
        panic!("malformed input");
    }

    let mut rules = HashMap::new();

    let re = Regex::new(r"^(\w)(\w) -> (\w)$").unwrap();

    for line in input {
        let line = line.unwrap();

        let c = re.captures(&line).unwrap();
        rules.insert(
            (
                c.get(1).unwrap().as_str().chars().next().unwrap(),
                c.get(2).unwrap().as_str().chars().next().unwrap(),
            ),
            c.get(3).unwrap().as_str().chars().next().unwrap(),
        );
    }

    PolymerState { polymer, rules }
}

fn run_step(state: &PolymerState) -> String {
    let mut new_polymer = String::with_capacity((state.polymer.len() - 1) * 2);
    let mut bases = state.polymer.chars();

    let mut prev_base = bases.next().unwrap();
    new_polymer.push(prev_base);

    for base in bases {
        if let Some(insert) = state.rules.get(&(prev_base, base)) {
            new_polymer.push(*insert);
        }

        new_polymer.push(base);
        prev_base = base;
    }

    new_polymer
}

fn count_bases(state: &PolymerState) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for base in state.polymer.chars() {
        *counts.entry(base).or_insert(0) += 1;
    }

    counts
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut state = parse_input(&mut input);

    for _ in 0..10 {
        state.polymer = run_step(&state);
        println!("{}", state.polymer);
    }

    let counts = count_bases(&state);

    let min = counts.iter().min_by_key(|(_, &v)| v).unwrap();
    let max = counts.iter().max_by_key(|(_, &v)| v).unwrap();

    println!("max: {:?}", max);
    println!("min: {:?}", min);
    Ok(max.1 - min.1)
}

// Part 2

#[derive(Debug)]
struct PolymerState2 {
    bases: HashMap<char, u64>,
    basepairs: HashMap<(char, char), u64>,
    rules: HashMap<(char, char), char>,
}

fn parse_input2(input: &mut io::Lines<io::BufReader<File>>) -> PolymerState2 {
    let mut bases = HashMap::new();
    let mut basepairs = HashMap::new();

    let polymer = input.next().unwrap().unwrap();
    let mut chars = polymer.chars();
    let mut prev_base = chars.next().unwrap();
    bases.insert(prev_base, 1);

    for base in chars {
        *bases.entry(base).or_insert(0) += 1;
        *basepairs.entry((prev_base, base)).or_insert(0) += 1;
        prev_base = base;
    }

    let line = input.next().unwrap().unwrap();
    if line != "" {
        panic!("malformed input");
    }

    let mut rules = HashMap::new();

    let re = Regex::new(r"^(\w)(\w) -> (\w)$").unwrap();

    for line in input {
        let line = line.unwrap();

        let c = re.captures(&line).unwrap();
        rules.insert(
            (
                c.get(1).unwrap().as_str().chars().next().unwrap(),
                c.get(2).unwrap().as_str().chars().next().unwrap(),
            ),
            c.get(3).unwrap().as_str().chars().next().unwrap(),
        );
    }

    PolymerState2 {
        bases,
        basepairs,
        rules,
    }
}

fn run_step2(state: &PolymerState2) -> PolymerState2 {
    let mut new_state = PolymerState2 {
        bases: state.bases.clone(),
        basepairs: HashMap::new(),
        rules: state.rules.clone(),
    };

    for (&(base1, base2), &v) in &state.basepairs {
        if let Some(&insert) = state.rules.get(&(base1, base2)) {
            *new_state.bases.entry(insert).or_insert(0) += v;

            *new_state.basepairs.entry((base1, insert)).or_insert(0) += v;
            *new_state.basepairs.entry((insert, base2)).or_insert(0) += v;
        } else {
            *new_state.basepairs.entry((base1, base2)).or_insert(0) += v;
        }
    }

    new_state
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    let mut state = parse_input2(&mut input);

    for _ in 0..40 {
        state = run_step2(&state);
        println!("{:#?}", state.bases);
    }

    let min = state.bases.iter().min_by_key(|&(_, v)| v).unwrap();
    let max = state.bases.iter().max_by_key(|&(_, v)| v).unwrap();

    println!("max: {:?}", max);
    println!("min: {:?}", min);
    Ok(max.1 - min.1)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day14")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day14")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
