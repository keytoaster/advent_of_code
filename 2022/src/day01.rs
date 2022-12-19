use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::env;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use itertools::Itertools;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut cur = 0;
    let mut max = 0;

    for line in input {
        let line = line?;
        println!("line: {}", line);
        if line == "" {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            let num = line.parse::<u32>().unwrap();
            cur += num;
        }
    }
    if cur > max {
        max = cur;
    }

    Ok(max)
}

/// Part 2

struct ConstGenericMinHeap<T: Ord, const N: usize> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord, const N: usize> ConstGenericMinHeap<T, N> {
    fn new() -> ConstGenericMinHeap<T, N> {
        ConstGenericMinHeap {
            heap: BinaryHeap::<Reverse<T>>::new(),
        }
    }

    fn push(&mut self, n: T) {
        self.heap.push(Reverse(n));

        if self.heap.len() > N {
            self.heap.pop();
        }
    }
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let inputs = input.group_by(|line| line.as_ref().unwrap().is_empty());
    let inputs = inputs.into_iter().filter(|(key, _)| *key == false);
    let inputs = inputs.into_iter().map(|(_, group)| {
        group.map(|x| x.unwrap().parse::<u32>().unwrap()).sum::<u32>()
    });

    let mut heap = ConstGenericMinHeap::<u32, 3>::new();

    for input in inputs {
        heap.push(input);
    }

    Ok(heap.heap.iter().map(|Reverse(e)| e).sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day01")?;
    println!("Part 1: Num increases: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day01")?;
    println!("Part 2: Num increases: {}", part2(lines)?);

    Ok(())
}
