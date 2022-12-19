use std::cmp::Reverse;
use std::collections::{VecDeque, BinaryHeap};
use std::env;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
enum Direction {
    UNKNOWN,
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Board {
    risk: Vec<Vec<u32>>,
    min_cost: Vec<Vec<u32>>,
    min_cost_from: Vec<Vec<Direction>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(risk: &Vec<Vec<u32>>, size_multiplier: usize) -> Board {
        let risk_width = risk[0].len();
        let risk_height = risk.len();

        let width = risk_width * size_multiplier;
        let height = risk_height * size_multiplier;

        Board {
            risk: risk.clone(),
            min_cost: vec![vec![u32::MAX; width]; height],
            min_cost_from: vec![vec![Direction::UNKNOWN; width]; height],
            width: width,
            height: height,
        }
    }

    fn get_risk(&self, x: usize, y: usize) -> u32 {
        if x == 0 && y == 0 {
            return 0;
        }

        let steps_x = x / self.risk[0].len();
        let steps_y = y / self.risk.len();

        let real_x = x % self.risk[0].len();
        let real_y = y % self.risk.len();

        (self.risk[real_x][real_y] + steps_x as u32 + steps_y as u32 - 1) % 9 + 1
    }
}

fn parse_input(input: io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
    let mut risks = Vec::new();

    for line in input {
        let line = line.unwrap();

        risks.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    risks
}

fn part1(
    input: io::Lines<io::BufReader<File>>,
    size_multiplier: usize,
) -> Result<u32, Box<dyn Error>> {
    let risks = parse_input(input);

    let mut board = Board::new(&risks, size_multiplier);

    // Priority queue, ordered by path cost.
    // BinaryHeap is usually a max heap, so we use Reverse to get a
    // min heap.
    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0), 0, 0));

    let mut iteration_count = 0;

    while let Some((Reverse(cost), x, y)) = queue.pop() {
        let new_cost = cost + board.get_risk(x, y);
        
        // println!("Processing ({}, {})", x, y);
        if new_cost < board.min_cost[x][y] {
            board.min_cost[x][y] = new_cost;
            
            if x < board.width - 1 {
                queue.push((Reverse(new_cost), x + 1, y));
            }
            if x > 0 {
                queue.push((Reverse(new_cost), x - 1, y));
            }
            if y < board.height - 1 {
                queue.push((Reverse(new_cost), x, y + 1));
            }
            if y > 0 {
                queue.push((Reverse(new_cost), x, y - 1));
            }
        }

        iteration_count += 1;

        // if iteration_count % 1000 == 0 {
        //     println!("Iteration count: {}", iteration_count);
        // }
    }

    Ok(board.min_cost[board.width - 1][board.height - 1])
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day15")?;
    println!("Part 1: {}", part1(lines, 1)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day15")?;
    println!("Part 2: {}", part1(lines, 5)?);

    Ok(())
}
