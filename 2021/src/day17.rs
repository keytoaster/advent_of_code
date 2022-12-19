use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn steps_to_zone(x: u32, x0: u32, x1: u32) -> Vec<u32> {
//     let steps = 0;
//     let retval = Vec::new();
//     let pos = 0;

//     loop {
//         pos += x;

//         if x > 0 {

//         } else if x < 0 {

//         }

//         if x == 0 {
//             break;
//         }
//     }

//     vec![1, 2, 3]
// }

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<(i32, i32), Box<dyn Error>> {
    let line = input.next().unwrap()?;
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;
    let captures = re.captures(&line).unwrap();

    let x0 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let x1 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let y0 = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let y1 = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

    println!("{}, {}, {}, {}", x0, x1, y0, y1);

    // Just brute-force. Don't assume anything about how the scale or distribution of the input coordinates.

    if x0 < 0 && x1 > 0 {
        panic!("unsupported")
    }

    let mut y;

    if y1 <= 0 {
        // Very extreme lower bound, but gives us a good starting point for brute force.
        // (x0, y) always hits the target area in 1 step, thus y is a solution.
        y = y0 - 1;
    } else {
        y = 0;
    }

    let mut highest_y = i32::MIN;
    let mut best_solution = (0, i32::MIN);

    for y in y..=x1 {
        println!("Trying y = {}", y);

        // First determine if this y can hit the target area in any amount of steps.
        let mut y_pos = 0;
        let mut y_round = y;
        let mut steps = 0;
        let mut highest_pos_for_this_y = y_round;

        loop {
            y_pos += y_round;
            y_round -= 1;
            steps += 1;

            // On our way down and passed the bottom of the target area.
            if y_round < 0 && y_pos < y0 {
                break;
            }

            if y_pos > highest_pos_for_this_y {
                highest_pos_for_this_y = y_pos;
            }

            if y_pos >= y0 && y_pos <= y1 {
                println!("y {} hits in {} steps", y, steps);
                // Check if a corresponding x exists that can hit the target area in the same amount of steps.
                let r;
                if x0 >= 0 {
                    r = 0..=x1;
                } else {
                    r = x1..=0;
                }

                for x in r {
                    let mut x_pos = 0;
                    let mut x_round = x;

                    // This loop could be a closed-form expression, but I can't be bothered.
                    for _ in 0..steps {
                        x_pos += x_round;

                        if x_round > 0 {
                            x_round -= 1;
                        } else if x < 0 {
                            x_round += 1;
                        }
                    }

                    // println!("Trying ({}, {}) = {}, {}", x, y, x_pos, y_pos);

                    if x_pos >= x0 && x_pos <= x1 {
                        println!(
                            "Solution found: {},{}. Highest y_pos: {}",
                            x, y, highest_pos_for_this_y
                        );
                        if highest_pos_for_this_y > highest_y {
                            highest_y = highest_pos_for_this_y;
                            best_solution = (x, y);
                        }
                    }
                }
            }
        }
    }

    Ok(best_solution)
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<usize, Box<dyn Error>> {
    let line = input.next().unwrap()?;
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;
    let captures = re.captures(&line).unwrap();

    let x0 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let x1 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let y0 = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let y1 = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

    println!("{}, {}, {}, {}", x0, x1, y0, y1);

    // Just brute-force. Don't assume anything about how the scale or distribution of the input coordinates.

    if x0 < 0 && x1 > 0 {
        panic!("unsupported")
    }

    let mut y;

    if y1 <= 0 {
        // Very extreme lower bound, but gives us a good starting point for brute force.
        // (x0, y) always hits the target area in 1 step, thus y is a solution.
        y = y0 - 1;
    } else {
        y = 0;
    }

    let mut solutions = HashSet::new();

    for y in y..=x1 {
        // println!("Trying y = {}", y);

        // First determine if this y can hit the target area in any amount of steps.
        let mut y_pos = 0;
        let mut y_round = y;
        let mut steps = 0;
        let mut highest_pos_for_this_y = y_round;

        loop {
            y_pos += y_round;
            y_round -= 1;
            steps += 1;

            // On our way down and passed the bottom of the target area.
            if y_round < 0 && y_pos < y0 {
                break;
            }

            if y_pos > highest_pos_for_this_y {
                highest_pos_for_this_y = y_pos;
            }

            if y_pos >= y0 && y_pos <= y1 {
                println!("y {} hits in {} steps", y, steps);
                // Check if a corresponding x exists that can hit the target area in the same amount of steps.
                let r;
                if x0 >= 0 {
                    r = 0..=x1;
                } else {
                    r = x1..=0;
                }

                for x in r {
                    let mut x_pos = 0;
                    let mut x_round = x;

                    for _ in 0..steps {
                        x_pos += x_round;

                        if x_round > 0 {
                            x_round -= 1;
                        } else if x < 0 {
                            x_round += 1;
                        }
                    }

                    // println!("Trying ({}, {}) = {}, {}", x, y, x_pos, y_pos);

                    if x_pos >= x0 && x_pos <= x1 {
                        println!(
                            "Solution found: {},{}. Highest y_pos: {}",
                            x, y, highest_pos_for_this_y
                        );
                        solutions.insert((x, y));
                    }
                }
            }
        }
    }

    Ok(solutions.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day17")?;
    println!("Part 1: {:?}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day17")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}

