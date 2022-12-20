use std::collections::{VecDeque, HashSet, HashMap};
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

fn run(input: &str, size: usize) -> Result<usize, Box<dyn Error>> {
    let mut set: HashMap<char, u32> = HashMap::new();
    let mut buf: VecDeque<char> = VecDeque::with_capacity(size);

    let mut pos = 0;

    for c in input.chars() {
        pos += 1;

        if buf.len() == size {
            let gone = &buf.pop_front().unwrap();

            match set.get(gone).unwrap() {
                1 => set.remove(gone),
                n => set.insert(*gone, n - 1),
            };
        }

        match set.get(&c) {
            Some(n) => set.insert(c, n + 1),
            None => set.insert(c, 1)
        };

        buf.push_back(c);

        if set.len() == size {
            return Ok(pos);
        }
    }

    Err("Could not find marker".into())    
}

#[test]
fn run_a() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)?, 7);
    Ok(())
}

#[test]
fn run_b() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)?, 5);
    Ok(())
}

#[test]
fn run_c() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg", 4)?, 6);
    Ok(())
}

#[test]
fn run_d() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)?, 10);
    Ok(())
}

#[test]
fn run_e() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)?, 11);
    Ok(())
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<usize, Box<dyn Error>> {
    run(&input.next().unwrap().unwrap(), 4)
}
/// Part 2

#[test]
fn run2_a() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)?, 19);
    Ok(())
}

#[test]
fn run2_b() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)?, 23);
    Ok(())
}

#[test]
fn run2_c() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg", 14)?, 23);
    Ok(())
}

#[test]
fn run2_d() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)?, 29);
    Ok(())
}

#[test]
fn run2_e() -> Result<(), Box<dyn Error>> {
    assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)?, 26);
    Ok(())
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<usize, Box<dyn Error>> {
    run(&input.next().unwrap().unwrap(), 14)
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day06")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day06")?;
    // let lines = read_input("src/input/day06")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
