use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::env;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut board = [[0; 1000]; 1000];

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)")?;

    for line in input {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();

        let (x1, y1, x2, y2) = (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                                captures.get(4).unwrap().as_str().parse::<usize>().unwrap());
        
        println!("{}, {} -> {}, {}", x1, y1, x2, y2);
        if x1 == x2 {
            println!("Horizontal line detected");
            
            // Horizontal line
            for i in min(y1, y2)..=max(y1, y2) {
                board[x1][i] += 1;
            }
        } else if y1 == y2 {
            println!("Vertical line detected");
            // Vertical line
            for i in min(x1, x2)..=max(x1, x2) {
                board[i][y1] += 1;
            }
        }

        for i in 0..15 {
            println!("{:?}", &board[i][0..15]);
        }
    }

    let mut count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if board[i][j] >= 2 {
                count += 1;
            }
        }
    }

    Ok(count)
}

// Still a nightly-only experimental at the time of writing: https://github.com/rust-lang/rust/issues/89492
fn usize_abs_diff(a: usize, b:usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut board = [[0; 1000]; 1000];

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)")?;

    for line in input {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();

        let (x1, y1, x2, y2) = (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                                captures.get(4).unwrap().as_str().parse::<usize>().unwrap());
        
        println!("{}, {} -> {}, {}", x1, y1, x2, y2);
        if x1 == x2 {
            // Horizontal line
            for i in min(y1, y2)..=max(y1, y2) {
                board[x1][i] += 1;
            }
        } else if y1 == y2 {
            // Vertical line
            for i in min(x1, x2)..=max(x1, x2) {
                board[i][y1] += 1;
            }
        } else {
            let length = usize_abs_diff(x1, x2);
            let go_right = x2 > x1;
            let go_down = y2 > y1;

            for i in 0..=length {
                let x_index = if go_right {
                    x1 + i
                } else {
                    x1 - i
                };

                let y_index = if go_down {
                    y1 + i
                } else {
                    y1 - i
                };

                board[x_index][y_index] += 1;
            }
        }

        for i in 0..15 {
            println!("{:?}", &board[i][0..15]);
        }
    }

    let mut count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if board[i][j] >= 2 {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day05")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day05")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
