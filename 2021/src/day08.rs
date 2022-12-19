use itertools::Itertools;
use std::collections::HashMap;
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

fn sorted_signal(signal: &str) -> String {
    signal.chars().sorted().collect::<String>()
}

fn solve(line: &Vec<String>, signal_to_digit: &mut HashMap<String, i8>) -> Result<(), Box<dyn Error>> {
     // Maps digit to signal.
     const EMPTY_STRING: String = String::new();
     let mut digit_to_signal = [EMPTY_STRING; 10];

     let mut unknown_signals_5 = Vec::new();
     let mut unknown_signals_6 = Vec::new();

     for i in 0..10 {
         let signal = sorted_signal(&line[i]);

         match signal.len() {
             2 => {
                 signal_to_digit.insert(signal.clone(), 1);
                 digit_to_signal[1] = signal;
             }
             4 => {
                 signal_to_digit.insert(signal.clone(), 4);
                 digit_to_signal[4] = signal;
             }
             3 => {
                 signal_to_digit.insert(signal.clone(), 7);
                 digit_to_signal[7] = signal;
             }
             7 => {
                 signal_to_digit.insert(signal.clone(), 8);
                 digit_to_signal[8] = signal;
             }
             5 => {
                 unknown_signals_5.push(signal);
             }
             6 => {
                 unknown_signals_6.push(signal);
             }
             _ => return Err("unexpected signal".into()),
         };
     }

     for i in 0..3 {
         let candidate = unknown_signals_6[i].clone();
         if !digit_to_signal[7].chars().all(|c| candidate.contains(c)) {
             signal_to_digit.insert(candidate.clone(), 6);
             digit_to_signal[6] = candidate;
             unknown_signals_6.remove(i);
             break;
         }
     }

     for i in 0..3 {
         let candidate = unknown_signals_5[i].clone();
         if candidate.chars().all(|c| digit_to_signal[6].contains(c)) {
             signal_to_digit.insert(candidate.clone(), 5);
             digit_to_signal[5] = candidate;
             unknown_signals_5.remove(i);
             break;
         }
     }

     if unknown_signals_5[0]
         .chars()
         .filter(|c| !digit_to_signal[5].contains(*c))
         .count()
         == 2
     {
         digit_to_signal[2] = unknown_signals_5.remove(0);
         digit_to_signal[3] = unknown_signals_5.remove(0);

         signal_to_digit.insert(digit_to_signal[2].clone(), 2);
         signal_to_digit.insert(digit_to_signal[3].clone(), 3);
     } else {
         digit_to_signal[2] = unknown_signals_5.remove(1);
         digit_to_signal[3] = unknown_signals_5.remove(0);

         signal_to_digit.insert(digit_to_signal[2].clone(), 2);
         signal_to_digit.insert(digit_to_signal[3].clone(), 3);
     }

     if digit_to_signal[5]
         .chars()
         .all(|c| unknown_signals_6[0].contains(c))
     {
         digit_to_signal[9] = unknown_signals_6.remove(0);
         digit_to_signal[0] = unknown_signals_6.remove(0);

         signal_to_digit.insert(digit_to_signal[9].clone(), 9);
         signal_to_digit.insert(digit_to_signal[0].clone(), 0);
     } else {
         digit_to_signal[9] = unknown_signals_6.remove(1);
         digit_to_signal[0] = unknown_signals_6.remove(0);

         signal_to_digit.insert(digit_to_signal[9].clone(), 9);
         signal_to_digit.insert(digit_to_signal[0].clone(), 0);
     }

     Ok(())
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;

    for line in input {
        let line = line?;
        let line: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();

        // Maps signal string (e.g. "ace") to digit (e.g. 7)
        let mut signal_to_digit: HashMap<String, i8> = HashMap::new();

        solve(&line, &mut signal_to_digit)?;
        println!("{:?}", signal_to_digit);

        for signal in 0..4 {
            let signal = sorted_signal(&line[11 + signal]);
            println!("{} = {}", signal, signal_to_digit[&signal]);

            if [1, 4, 7, 8].contains(&signal_to_digit[&signal]) {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    for line in input {
        let line = line?;
        let line: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();

        // Maps signal string (e.g. "ace") to digit (e.g. 7)
        let mut signal_to_digit: HashMap<String, i8> = HashMap::new();

        solve(&line, &mut signal_to_digit)?;
        println!("{:?}", signal_to_digit);

        let mut value: i32 = 0;

        for signal in 0..4 {
            let signal = sorted_signal(&line[11 + signal]);
            value = value * 10 + &signal_to_digit[&signal].into();
        }
        println!("{}", value);
        count += value;
    }

    Ok(count.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day08")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day08")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
