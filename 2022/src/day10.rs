use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use nom::branch::alt;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::character::complete::i32;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Instruction {
    Addx(i32),
    Noop,
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, num) = preceded(tag("addx "), i32)(input)?;
    Ok((input, Instruction::Addx(num)))
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_addx, parse_noop))(input)
}


struct Machine {
    cycle_number: u32,
    current_instruction: Option<Instruction>,
    current_instruction_remaining_cycles: u32,
    reg_x: i32,
}

enum State {
    Done,
    InProgress,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            cycle_number: 1,
            current_instruction: None,
            current_instruction_remaining_cycles: 0,
            reg_x: 1,
        }
    }

    fn feed(&mut self, i: Instruction) {
        self.current_instruction = Some(i);

        self.current_instruction_remaining_cycles = match self.current_instruction.as_ref().unwrap() {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        };
    }

    fn tick(&mut self) -> State {
        self.cycle_number += 1;

        self.current_instruction_remaining_cycles -= 1;

        match self.current_instruction_remaining_cycles {
            0 => {
                match self.current_instruction.as_ref().unwrap() {
                    Instruction::Addx(num) => self.reg_x += num,
                    Instruction::Noop => {},
                }

                self.current_instruction = None;

                State::Done
            },
            _ => State::InProgress,
        }
    }
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let mut m = Machine::new();
    let (_, instr) = parse_instruction(&input.next().unwrap().unwrap()).or(Err("could not parse instruction"))?;
    m.feed(instr);
    let mut stop = false;

    let mut total_signal_strength: i32 = 0;

    loop {
        if stop {
            break;
        }

        match m.tick() {
            State::Done => {
                if let Some(line) = input.next() {
                    let line = line?;
                    let (_, instr) = parse_instruction(&line).or(Err("could not parse instruction"))?;

                    m.feed(instr);
                } else {
                    stop = true;
                }
            }
            State::InProgress => {
            }
        }

        if m.cycle_number >= 20 {
            if (m.cycle_number - 20) % 40 == 0 {
                let signal = m.reg_x * i32::try_from(m.cycle_number).unwrap();
                println!("during {} cycle: reg_x = {}, score = {}", m.cycle_number, m.reg_x, signal);
                total_signal_strength += signal;
            }
        }
    }

    Ok(total_signal_strength)
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn Error>> {
    let mut m = Machine::new();
    let (_, instr) = parse_instruction(&input.next().unwrap().unwrap()).or(Err("could not parse instruction"))?;
    m.feed(instr);
    let mut stop = false;

    let mut crt_line = String::with_capacity(40);

    loop {
        if stop {
            break;
        }

        let crt_position: i32 = i32::try_from((m.cycle_number - 1) % 40).unwrap();

        if crt_position >= m.reg_x - 1 && crt_position <= m.reg_x + 1 {
            crt_line.push('#');
        } else {
            crt_line.push('.');
        }

        if crt_position % 40 == 39 {
            println!("{}", crt_line);
            crt_line.clear();
        }

        match m.tick() {
            State::Done => {
                if let Some(line) = input.next() {
                    let line = line?;
                    let (_, instr) = parse_instruction(&line).or(Err("could not parse instruction"))?;

                    m.feed(instr);
                } else {
                    stop = true;
                }
            }
            State::InProgress => {
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    // let lines = read_input("src/input/day10_example")?;
    let lines = read_input("src/input/day10")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day10")?;
    println!("Part 2:");
    part2(lines)?;

    Ok(())
}
