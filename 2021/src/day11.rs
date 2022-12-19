use std::collections::{VecDeque, HashSet};
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

fn energize(i: usize, j: usize, field: &mut [[u32; 10]; 10], to_process: &mut VecDeque<(usize, usize)>) {
    field[i][j] += 1;
    if field[i][j] == 10 {
        to_process.push_back((i, j));
    }
}

fn parse_input(mut input: io::Lines<io::BufReader<File>>) -> [[u32; 10]; 10] {
    let mut field = [[0; 10]; 10];

    for i in 0..10 {
        let line = input.next().unwrap().unwrap();
        let mut line = line.chars();
        for j in 0..10 {
            field[i][j] = line.next().unwrap().to_digit(10).unwrap();
        }
    }

    field
}

fn run_step(field: &mut [[u32; 10]; 10]) {
    let mut to_process = VecDeque::new();

    for i in 0..10 {
        for j in 0..10 {
            energize(i, j, field, &mut to_process);
        }
    }

    while let Some((i, j)) = to_process.pop_front() {
        if i != 0 {
            energize(i-1, j, field, &mut to_process);
        }
        if i != 9 {
            energize(i+1, j, field, &mut to_process);
        }
        if j != 0 {
            energize(i, j-1, field, &mut to_process);
        }
        if j != 9 {
            energize(i, j+1, field, &mut to_process);
        }

        if i != 0 && j != 0 {
            energize(i-1, j-1, field, &mut to_process);
        }
        if i != 0 && j != 9 {
            energize(i-1, j+1, field, &mut to_process);
        }
        if i != 9 && j != 0 {
            energize(i+1, j-1, field, &mut to_process);
        }
        if i != 9 && j != 9 {
            energize(i+1, j+1, field, &mut to_process);
        }
    }
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut field = parse_input(input);

    let mut flashes = 0;
    
    for step in 1..101 {
        run_step(&mut field);        

        for i in 0..10 {
            for j in 0..10 {
                if field[i][j] > 9 {
                    field[i][j] = 0;
                    flashes += 1;
                }
            }
        }

        println!("After step {}", step);
        for i in 0..10 {
            for j in 0..10 {
                print!("{}", field[i][j]);
            }
            println!()
        }
    }

    println!("Flashes: {}", flashes);
    Ok(0)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut field = parse_input(input);

    let mut step = 1;

    loop {
        run_step(&mut field);        

        let mut all_flashing = true;
        for i in 0..10 {
            for j in 0..10 {
                if field[i][j] > 9 {
                    field[i][j] = 0;
                } else {
                    all_flashing = false;
                }
            }
        }

        println!("After step {}", step);
        for i in 0..10 {
            for j in 0..10 {
                print!("{}", field[i][j]);
            }
            println!()
        }

        if all_flashing {
            break;
        }

        step += 1;
    }

    println!("Step: {}", step);
    Ok(step)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day11")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day11")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}


struct Graph {
    // v: HashSet<Node>,
    v: HashSet<u32>,
}