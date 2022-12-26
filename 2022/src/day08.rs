use std::cmp;
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

#[derive(Debug)]
struct Tree {
    height: u8,
    /* Part 1. */
    max_from_top: u8,
    max_from_bot: u8,
    max_from_left: u8,
    max_from_right: u8,
    /* Part 2 */
    visibility_top: u8,
    visibility_bot: u8,
    visibility_left: u8,
    visibility_right: u8,
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

fn parse_input(input: io::Lines<io::BufReader<File>>) -> Result<Forest, Box<dyn Error>> {
    Ok(Forest {
        trees: input
            .map(|row| {
                row.unwrap()
                    .chars()
                    .map(|c| Tree {
                        height: u8::try_from(c.to_digit(10).unwrap()).unwrap(),
                        max_from_top: 0,
                        max_from_bot: 0,
                        max_from_left: 0,
                        max_from_right: 0,
                        visibility_top: 0,
                        visibility_bot: 0,
                        visibility_left: 0,
                        visibility_right: 0,
                    })
                    .collect()
            })
            .collect(),
    })
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut forest = parse_input(input)?;
    let forest_height = forest.trees.len();
    let forest_width = forest.trees[0].len();

    for i in 1..forest_height {
        forest.trees[i][0].max_from_left = forest.trees[i][0].height;
        forest.trees[i][forest_width - 1].max_from_right = forest.trees[i][forest_width - 1].height;
    }
    for j in 1..forest_width {
        forest.trees[0][j].max_from_top = forest.trees[0][j].height;
        forest.trees[forest_height - 1][j].max_from_bot = forest.trees[forest_height - 1][j].height;
    }

    for i in 1..forest_height {
        for j in 1..forest_width {
            forest.trees[i][j].max_from_top = cmp::max(
                forest.trees[i - 1][j].max_from_top,
                forest.trees[i][j].height,
            );
            forest.trees[i][j].max_from_left = cmp::max(
                forest.trees[i][j - 1].max_from_left,
                forest.trees[i][j].height,
            );
        }
    }

    for i in (0..forest_height - 1).rev() {
        for j in (0..forest_width - 1).rev() {
            forest.trees[i][j].max_from_bot = cmp::max(
                forest.trees[i + 1][j].max_from_bot,
                forest.trees[i][j].height,
            );
            forest.trees[i][j].max_from_right = cmp::max(
                forest.trees[i][j + 1].max_from_right,
                forest.trees[i][j].height,
            );
        }
    }

    let mut num_trees_visible =
        2 * u32::try_from(forest_height).unwrap() + 2 * u32::try_from(forest_width).unwrap() - 4;

    for i in 1..forest_height - 1 {
        for j in 1..forest_width - 1 {
            if forest.trees[i][j].height > forest.trees[i + 1][j].max_from_bot
                || forest.trees[i][j].height > forest.trees[i - 1][j].max_from_top
                || forest.trees[i][j].height > forest.trees[i][j - 1].max_from_left
                || forest.trees[i][j].height > forest.trees[i][j + 1].max_from_right
            {
                num_trees_visible += 1;
                println!("{},{} is visible", i, j);
                if i == 3 && j == 1 {
                    println!("{} {} {} {}", forest.trees[i + 1][j].max_from_bot, forest.trees[i - 1][j].max_from_top, forest.trees[i][j - 1].max_from_left, forest.trees[i][j + 1].max_from_right);
                }
            }
        }
    }

    Ok(num_trees_visible)
}

/// Part 2

impl Forest {
    
    fn tree_scenic_score(&self, row: usize, col: usize) -> u32 {
        let mut top = 0;
        for i in (0..row).rev() {
            top += 1;
            if self.trees[i][col].height >= self.trees[row][col].height {
                break;
            }
        }
        
        let mut left = 0;
        for j in (0..col).rev() {
            left += 1;
            if self.trees[row][j].height >= self.trees[row][col].height {
                break;
            }
        }

        let mut right = 0;
        for j in col + 1..self.trees[0].len() {
            right += 1;
            if self.trees[row][j].height >= self.trees[row][col].height {
                break;
            }
        }
        
        let mut bot = 0;
        for i in row + 1..self.trees.len() {
            bot += 1;
            if self.trees[i][col].height >= self.trees[row][col].height {
                break;
            }
        }
        
        let score = top * left * right * bot;
        println!("tree ({}, {}) has {}, {}, {}, {} for {}", row, col, top, left, right, bot, score);
        
        score
    }

}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut forest = parse_input(input)?;
    let forest_height = forest.trees.len();
    let forest_width = forest.trees[0].len();

    let highest_scenic_score = 0;

    Ok(forest.trees.iter().enumerate().map(|(i, row)| row.iter().enumerate().map(|(j, _)| forest.tree_scenic_score(i, j)).max().unwrap()).max().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    // let lines = read_input("src/input/day08_example")?;
    let lines = read_input("src/input/day08")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    // let lines = read_input("src/input/day08_example")?;
    let lines = read_input("src/input/day08")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
