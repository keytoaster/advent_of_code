use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use regex::Regex;

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

fn _crate(input: &str) -> IResult<&str, char> {
    delimited(tag("["), anychar, tag("]"))(input)
}

fn _empty(input: &str) -> IResult<&str, ()> {
    // Just discard.
    let (input, _) = tag("   ")(input)?;
    Ok((input, ()))
}

fn elem(input: &str) -> IResult<&str, Option<char>> {
    terminated(alt((map(_crate, |c| Some(c)), map(_empty, |_| None))), opt(tag(" ")))(input)
}

fn parse_initial_state(input: &mut io::Lines<io::BufReader<File>>) -> State {
    let mut stacks = Vec::new();

    for line in input {
        let line = line.unwrap();
        println!("line: {:?}", line);

        if line.starts_with(" 1 ") {
            break;
        }
        
        // Number of stacks is not known in advance and input lines have different lengths.
        let num_stacks = line.len() / 4 + 1;
        if num_stacks > stacks.len() {
            stacks.resize(num_stacks, Vec::new());
        }

        let mut cur = &line[..];
        let mut cur_stack = 0;
        while let Ok((remaining, e)) = elem(&cur) {
            cur = remaining;

            if let Some(elem) = e {
                stacks[cur_stack].push(elem);
            }

            cur_stack += 1;
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    State { stacks }
}

fn run<F>(input: &mut io::Lines<io::BufReader<File>>, step_fn: F) -> Result<String, Box<dyn Error>> 
where F: Fn(usize, usize, usize, &mut State) {
    let mut iter = input.into_iter();

    let mut state = parse_initial_state(&mut iter);

    // Expected empty line in input.
    iter.next();

    for line in iter {
        let line = line.unwrap();
        
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let caps = re.captures(&line).unwrap();

        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

        step_fn(from, to, num, &mut state);
    }

    let mut result = String::with_capacity(state.stacks.len());
    for stack in state.stacks {
        result.push(*(&stack).last().unwrap());
    }

    Ok(result)
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<String, Box<dyn Error>> {
    run(&mut input, |from, to, num, state| {
        for _ in 0..num {
            let popped = state.stacks[from].pop().unwrap();
            state.stacks[to].push(popped);
        }
    })
}

/// Part 2

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<String, Box<dyn Error>> {
    run(&mut input, |from, to, num, state| {
        let range_to_drain = (state.stacks[from].len() - num)..;
        // .collect() is used here because drain() keeps a mutable reference to the stack which in turn
        // keeps a mutable reference to the overall stacks Vec (through Index). That prevents getting
        // an immutable ref to state.stacks[from] and a mutable ref to state.stacks[to] at the same time.
        // Alternative in order to avoid collect(): Use split(). But that requires some arithmetic on the
        // indexes and reduces the readability here.
        let drained = state.stacks[from].drain(range_to_drain).collect::<Vec<_>>();
        state.stacks[to].extend(drained);
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day05")?;
    // let lines = read_input("src/input/day05")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day05")?;
    // let lines = read_input("src/input/day05")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
