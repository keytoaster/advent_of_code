extern crate nom;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value,
    sequence::tuple, IResult,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

// impl TryFrom<u8> for Shape {
//     type Error = &'static str;

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Shape::Paper),
//             1 => Ok(Shape::Rock),
//             2 => Ok(Shape::Scissors),
//             _ => Err("cannot convert to Shape"),
//         }
//     }
// }

// impl TryFrom<Shape> for u8 {
//     type Error = &'static str;

//     fn try_from(value: Shape) -> Result<Self, Self::Error> {
//         match value {
//             Shape::Paper => Ok(0),
//             Shape::Rock => Ok(1),
//             Shape::Scissors => Ok(2),
//             _ => Err("cannot convert Shape to u8"),
//         }
//     }
// }

#[derive(Debug, Clone)]
struct Round {
    challenge: Shape,
    response: Shape,
}

#[derive(Debug, Clone)]
enum RoundResult {
    WIN,
    LOSS,
    DRAW,
}

fn challenge(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, char('A')),
        value(Shape::Paper, char('B')),
        value(Shape::Scissors, char('C')),
    ))(input)
}

fn response(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, char('X')),
        value(Shape::Paper, char('Y')),
        value(Shape::Scissors, char('Z')),
    ))(input)
}

fn round(input: &str) -> IResult<&str, Round> {
    let (input, (challenge, _, response)) = tuple((challenge, tag(" "), response))(input)?;
    Ok((
        input,
        Round {
            challenge,
            response,
        },
    ))
}

fn eval_round(round: &Round) -> RoundResult {
    if round.challenge == round.response {
        return RoundResult::DRAW;
    }

    if (round.challenge == Shape::Rock && round.response == Shape::Paper)
        || (round.challenge == Shape::Paper && round.response == Shape::Scissors)
        || (round.challenge == Shape::Scissors && round.response == Shape::Rock)
    {
        return RoundResult::WIN;
    }

    RoundResult::LOSS
}

fn run_round(round: &Round) -> u32 {
    let score_outcome = match eval_round(round) {
        RoundResult::WIN => 6,
        RoundResult::DRAW => 3,
        RoundResult::LOSS => 0,
    };
    let score_shape = match round.response {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    score_outcome + score_shape
}

fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    input.try_fold(0, |total_score, line| {
        let line = line?;
        let (_, round) = round(&line).or(Err("could not parse line"))?;
        Ok(total_score + run_round(&round))
    })
}

/// Part 2

#[derive(Debug, Clone)]
struct Round2 {
    challenge: Shape,
    expected_outcome: RoundResult,
}

fn expected_outcome(input: &str) -> IResult<&str, RoundResult> {
    alt((
        value(RoundResult::LOSS, char('X')),
        value(RoundResult::DRAW, char('Y')),
        value(RoundResult::WIN, char('Z')),
    ))(input)
}

fn round2(input: &str) -> IResult<&str, Round2> {
    let (input, (challenge, _, expected_outcome)) =
        tuple((challenge, tag(" "), expected_outcome))(input)?;
    Ok((
        input,
        Round2 {
            challenge,
            expected_outcome,
        },
    ))
}

fn determine_needed_shape(round: &Round2) -> Shape {
    match round.expected_outcome {
        RoundResult::DRAW => round.challenge,
        RoundResult::WIN => match round.challenge {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        RoundResult::LOSS => match round.challenge {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
    }
}

fn run_round2(round: &Round2) -> u32 {
    let score_outcome = match round.expected_outcome {
        RoundResult::WIN => 6,
        RoundResult::DRAW => 3,
        RoundResult::LOSS => 0,
    };

    let score_shape = match determine_needed_shape(round) {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    score_outcome + score_shape
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    input.try_fold(0, |total_score, line| {
        let line = line?;
        let (_, round) = round2(&line).or(Err("could not parse line"))?;
        Ok(total_score + run_round2(&round))
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day02_example")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day02")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
