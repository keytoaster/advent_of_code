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

fn add_bits_single_line(line: &str, counts: &mut Vec<u32>) {
    for (idx, c) in line.chars().enumerate() {
        if c == '1' {
            counts[idx] += 1;
        }
    }
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {

    let line = input.next().unwrap()?;

    let num_bits = line.len();

    let mut counts = vec![0; num_bits];
    let mut total_numbers = 1;

    add_bits_single_line(&line, &mut counts);

    for line in input {
        let line = line?;
        add_bits_single_line(&line, &mut counts);
        total_numbers += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    println!("total lines: {}", total_numbers);
    for (idx, &count) in counts.iter().enumerate() {
        println!("{} counted {} times", idx, count);
        if count > total_numbers / 2 {
            gamma   += 2_u32.pow((counts.len() - 1 - idx).try_into().unwrap());
        } else {
            epsilon += 2_u32.pow((counts.len() - 1 - idx).try_into().unwrap());
        }
    }

    Ok(gamma * epsilon)
}


#[derive(Debug)]
struct TrieNode {
    left: Option<Box<TrieNode>>,
    right: Option<Box<TrieNode>>,
    count: u32,
}
    
impl TrieNode {

    fn new() -> TrieNode {
        TrieNode {
            left: None,
            right: None,
            count: 0,
        }
    }

    fn insert(&mut self, new_elem: &str) -> Result<(), &'static str> {
        self.count += 1;

        if new_elem.len() == 0 {
            return Ok(());
        }

        let mut chars = new_elem.chars();
        let first_char = chars.next().unwrap();
        let rest = chars.as_str();
        
        match first_char {
            '0' => {
                match &mut self.left {
                    None => {
                        let mut new_node = TrieNode {
                            left: None,
                            right: None,
                            count: 0,
                        };
                        new_node.insert(rest)?;
                        self.left = Some(Box::new(new_node));
                    },
                    Some(left) => {
                        left.insert(rest)?;
                    }
                }
            },
            '1' => {
                match &mut self.right {
                    None => {
                        let mut new_node = TrieNode {
                            left: None,
                            right: None,
                            count: 0,
                        };
                        new_node.insert(rest)?;
                        self.right = Some(Box::new(new_node));
                    },
                    Some(right) => {
                        right.insert(rest)?;
                    }
                }
            },
            _ => {
                return Err("unsupported character");
            }
        };

        Ok(())
    }
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {

    let mut trie = TrieNode::new();

    for line in input {
        let line = line?;
        trie.insert(&line)?;
    }

    let mut oxygen_generator_rating = String::new();
    let mut cur = &trie;
    while !(cur.left.is_none() && cur.right.is_none()) {
        let mut go_left = false;
        if let Some(left) = &cur.left {
            if left.count > cur.count / 2 {
                go_left = true;
            }
        }

        if go_left {
            oxygen_generator_rating.push('0');
            // Unwrap is safe because there must be a child given the conditions.
            cur = cur.left.as_ref().unwrap();
        } else {
            oxygen_generator_rating.push('1');
            // Unwrap is safe because there must be a child given the conditions.
            cur = cur.right.as_ref().unwrap();
        }
    }

    let mut co2_scrubber_rating = String::new();
    let mut cur = &trie;
    while !(cur.left.is_none() && cur.right.is_none()) {
        let mut go_left = false;
        if let Some(left) = &cur.left {
            if left.count <= (cur.count / 2) || left.count == 1 {
                go_left = true;
            }
        }

        println!("solution so far: {}", co2_scrubber_rating);
        if go_left {
            co2_scrubber_rating.push('0');
            cur = cur.left.as_ref().unwrap();
            println!("pushed 0, count = {:?}, left = {:?}, right = {:?}", cur.count, cur.left, cur.right);
        } else {
            co2_scrubber_rating.push('1');
            // Unwrap is safe because there must be a right child given the conditions.
            cur = cur.right.as_ref().unwrap();
            println!("pushed 1, count = {:?}, left = {:?}, right = {:?}", cur.count, cur.left, cur.right);
        }
    }

    let oxy_as_int = u32::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2_as_int = u32::from_str_radix(&co2_scrubber_rating, 2).unwrap();
    println!("oxygen_generator_rating: {} = {}", oxygen_generator_rating, oxy_as_int);
    println!("co2_scrubber_rating:     {} = {}", co2_scrubber_rating, co2_as_int);

    Ok(oxy_as_int * co2_as_int)

}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day03")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day03")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
