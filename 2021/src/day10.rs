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

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut syntax_error_score = 0;

    for line in input {
        let line = line?;

        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        syntax_error_score += 3;
                        break;
                    }
                },
                ']' => {
                    if stack.pop() != Some('[') {
                        syntax_error_score += 57;
                        break;
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        syntax_error_score += 1197;
                        break;
                    }
                },
                '>' => {
                    if stack.pop() != Some('<') {
                        syntax_error_score += 25137;
                        break;
                    }
                },
                _ => return Err("unexpected character".into()),
            }
        }
    }

    Ok(syntax_error_score)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    let mut completion_scores = Vec::new();

    for line in input {
        let line = line?;

        let mut stack: Vec<char> = Vec::new();

        let mut corrupted = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        corrupted = true;
                        break;
                    }
                },
                ']' => {
                    if stack.pop() != Some('[') {
                        corrupted = true;
                        break;
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        corrupted = true;
                        break;
                    }
                },
                '>' => {
                    if stack.pop() != Some('<') {
                        corrupted = true;
                        break;
                    }
                },
                _ => return Err("unexpected character".into()),
            }
        }

        if corrupted {
            continue;
        }

        if stack.len() != 0 {
            let mut score = 0;
            println!("stack: {:?}", stack);

            while let Some(c) = stack.pop() {
                println!("score: {}", score);
                score *= 5;

                match c {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!("syntax stack is corrupted"),
                }
            }

            completion_scores.push(score);
        }
    }

    completion_scores.sort();
    
    println!("{:?}", completion_scores);
    println!("len: {:?}", completion_scores.len());
    Ok(completion_scores[completion_scores.len()/2])
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day10")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day10")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
