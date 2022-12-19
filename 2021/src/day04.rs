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

fn parse_input(mut input: io::Lines<io::BufReader<File>>) -> Result<(Vec<u32>, Vec<[[(u32, bool); 5]; 5]>), Box<dyn Error>> {
    let called_numbers = input.next().unwrap()?;
    let called_numbers = called_numbers.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
    
    let mut boards = Vec::new();

    // Parse all the boards.
    loop {
        if let Some(expected_empty_line) = input.next() {
            assert_eq!(expected_empty_line?, "");

            let mut board = [[(0, false); 5]; 5];
            for i in 0..5 {
                let single_row = input.next().unwrap()?;
                let mut single_row = single_row.split_whitespace().map(|s| s.parse::<u32>().unwrap());

                for j in 0..5 {
                    board[i][j] = (single_row.next().unwrap(), false);
                }
            }
            boards.push(board);
        } else {
            break;
        }
    }

    Ok((called_numbers, boards))
}

fn mark_number(board: &mut [[(u32, bool); 5]; 5], number: u32) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j].0 == number {
                board[i][j].1 = true;
            }
        }
    }
}

fn board_has_won(board: &[[(u32, bool); 5]; 5]) -> bool {
    for i in 0..5 {
        if (board[i][0].1 && board[i][1].1 && board[i][2].1 && board[i][3].1 && board[i][4].1) ||
           (board[0][i].1 && board[1][i].1 && board[2][i].1 && board[3][i].1 && board[4][i].1) {
            return true;
        }
    }

    false
}

fn sum_unmarked(board: &[[(u32, bool); 5]; 5]) -> u32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !board[i][j].1 {
                sum += board[i][j].0;
            }
        }
    }
    sum
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let (called_numbers, mut boards) = parse_input(input)?;

    for called_number in called_numbers {
        for board in &mut boards {
            mark_number(board, called_number);

            if board_has_won(&board) {
                return Ok(sum_unmarked(&board) * called_number);
            }
        }
    }

    Err("all numbers processed, but no board won".into())
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let (called_numbers, mut boards) = parse_input(input)?;

    for called_number in called_numbers {
        for board in &mut boards {
            mark_number(board, called_number);
        }

        if boards.len() > 1 {
            boards.retain(|board| !board_has_won(board));
        } else {
            if board_has_won(&boards[0]) {
                return Ok(sum_unmarked(&boards[0]) * called_number);
            }
        }
    }

    Err("solution conditions not met".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day04")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day04")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
