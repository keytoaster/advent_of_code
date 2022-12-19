use itertools::Itertools;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{VecDeque, HashSet};

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sorted_signal(signal: &str) -> String {
    signal.chars().sorted().collect::<String>()
}

fn check_line(prev_line: &Vec<u32>, cur_line: &Vec<u32>, next_line: &Vec<u32>) -> u32 {
    let mut count = 0;
    let field_width = prev_line.len();

    // Special case: 0-th column.
    if cur_line[0] < prev_line[0] && cur_line[0] < next_line[0] && cur_line[0] < cur_line[1] {
        count += cur_line[0] + 1;
    }

    for i in 1..field_width - 1 {
        if cur_line[i] < prev_line[i]
            && cur_line[i] < next_line[i]
            && cur_line[i] < cur_line[i - 1]
            && cur_line[i] < cur_line[i + 1]
        {
            count += cur_line[i] + 1;
        }
    }

    // Special case: last column.
    if cur_line[field_width - 1] < prev_line[field_width - 1]
        && cur_line[field_width - 1] < next_line[field_width - 1]
        && cur_line[field_width - 1] < cur_line[field_width - 1 - 1]
    {
        count += cur_line[field_width - 1] + 1;
    }

    count
}

fn parse_input(input: io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
    let mut field: Vec<Vec<u32>> = Vec::new();

    for line in input {
        let line = line.unwrap();
        field.push(line.chars().flat_map(|ch| ch.to_digit(10)).collect());
    }

    field
}

fn calc_minima(field: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let width = field[0].len();
    let height = field.len();

    let mut minima: Vec<(usize, usize)> = Vec::new();

    for i in 0..height {
        for j in 0..width {
            let mut minimum = true;

            if i != 0 && field[i][j] >= field[i - 1][j] {
                minimum = false;
            }
            if i != height - 1 && field[i][j] >= field[i + 1][j] {
                minimum = false;
            }
            if j != 0 && field[i][j] >= field[i][j-1] {
                minimum = false;
            }
            if j != width -1 && field[i][j] >= field[i][j+1] {
                minimum = false;
            }

            if minimum {
                minima.push((i, j));
            }
        }
    }

    minima
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let field = parse_input(input);
    let minima = calc_minima(&field);

    let mut count = 0;
    for (i, j) in minima {
        println!("({}, {})", i, j);
        count += field[i][j] + 1;
    }

    Ok(count)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let field = parse_input(input);
    let height = field.len();
    let width = field[0].len();

    let minima = calc_minima(&field);

    let mut basin_sizes: Vec<u32> = Vec::new();

    for &(i, j) in &minima {
        let mut basin_size = 1;
        let mut s: HashSet<(usize, usize)> = HashSet::new();
        let mut d = VecDeque::new();

        d.push_back((i, j));

        println!("processing ({}, {})", i, j);

        while let Some((x, y)) = d.pop_front() {
            println!("found ({}, {}", x, y);
            if x != 0 && field[x - 1][y] != 9 && field[x][y] <= field[x - 1][y] {
                if !s.contains(&(x - 1, y)) {
                    s.insert((x - 1, y));
                    d.push_back((x - 1, y));
                    basin_size += 1;
                }
            }
            if x != height - 1 && field[x + 1][y] != 9 && field[x][y] <= field[x + 1][y] {
                if !s.contains(&(x + 1, y)) {
                    s.insert((x + 1, y));
                    d.push_back((x + 1, y));
                    basin_size += 1;
                }
            }
            if y != 0 && field[x][y - 1] != 9 && field[x][y] <= field[x][y-1] {
                if !s.contains(&(x, y - 1)) {
                    s.insert((x, y - 1));
                    d.push_back((x, y - 1));
                    basin_size += 1;
                }
            }
            if y != width -1 && field[x][y + 1] != 9 && field[x][y] <= field[x][y+1] {
                if !s.contains(&(x, y + 1)) {
                    s.insert((x, y + 1));
                    d.push_back((x, y + 1));
                    basin_size += 1;
                }
            }
        }

        basin_sizes.push(basin_size);
    }

    println!("{:?}", basin_sizes);

    basin_sizes.sort();

    let num_basins = minima.len();

    Ok(basin_sizes[num_basins - 1] * basin_sizes[num_basins - 2] * basin_sizes[num_basins - 3])
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day09")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day09")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
