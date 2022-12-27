use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::u64;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::sequence::{preceded, terminated};
use nom::IResult;

fn read_input<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn monkey_id(input: &str) -> IResult<&str, u64> {
    let (input, (_, id, _)) = tuple((tag("Monkey "), u64, tag(":")))(input)?;
    Ok((input, id))
}

fn monkey_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("  Starting items: "), separated_list1(tag(", "), u64))(input)
}

fn monkey_operation_multiply(input: &str) -> IResult<&str, Operation> {
    let (input, num) = preceded(tag("* "), u64)(input)?;
    Ok((input, Operation::Multiply(num)))
}

fn monkey_operation_add(input: &str) -> IResult<&str, Operation> {
    let (input, num) = preceded(tag("+ "), u64)(input)?;
    Ok((input, Operation::Add(num)))
}

fn monkey_operation_square(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* old")(input)?;
    Ok((input, Operation::Square))
}

fn monkey_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("  Operation: new = old "),
        alt((
            monkey_operation_multiply,
            monkey_operation_add,
            monkey_operation_square,
        )),
    )(input)
}

fn monkey_test(input: &str) -> IResult<&str, u64> {
    preceded(tag("  Test: divisible by "), u64)(input)
}

fn monkey_test_cond_true_target(input: &str) -> IResult<&str, u64> {
    preceded(tag("    If true: throw to monkey "), u64)(input)
}

fn monkey_test_cond_false_target(input: &str) -> IResult<&str, u64> {
    preceded(tag("    If false: throw to monkey "), u64)(input)
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    test_cond_true_target: usize,
    test_cond_false_target: usize,
    inspections: u64,
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (_id, starting_items, op, test, test_true, test_false, _)) = tuple((
        terminated(monkey_id, line_ending),
        terminated(monkey_starting_items, line_ending),
        terminated(monkey_operation, line_ending),
        terminated(monkey_test, line_ending),
        terminated(monkey_test_cond_true_target, line_ending),
        terminated(monkey_test_cond_false_target, opt(line_ending)),
        opt(line_ending),
    ))(input)?;

    Ok((
        input,
        Monkey {
            items: starting_items,
            operation: op,
            test: test,
            test_cond_true_target: usize::try_from(test_true).unwrap(),
            test_cond_false_target: usize::try_from(test_false).unwrap(),
            inspections: 0
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    many1(parse_monkey)(input)
}

fn run_round(monkeys: &mut Vec<Monkey>, post_op: impl Fn(u64) -> u64) {
    for monkey_id in 0..monkeys.len() {
        while let Some(item) = monkeys[monkey_id].items.pop() {
            monkeys[monkey_id].inspections += 1;

            let new_worry_level = post_op(match monkeys[monkey_id].operation {
                Operation::Multiply(num) => item * num,
                Operation::Add(num) => item + num,
                Operation::Square => item * item,
            });

            let target_monkey_id = if new_worry_level % monkeys[monkey_id].test == 0 {
                monkeys[monkey_id].test_cond_true_target
            } else {
                monkeys[monkey_id].test_cond_false_target
            };
            monkeys[target_monkey_id].items.push(new_worry_level);
        }
    }
}

fn run(mut monkeys: Vec<Monkey>, num_rounds: u64, post_op: impl Fn(u64) -> u64)-> Result<u64, Box<dyn Error>> {
    for _ in 0..num_rounds {
        run_round(&mut monkeys, &post_op);
    }
    
    let mut monkeys_inspections: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    monkeys_inspections.sort();

    Ok(monkeys_inspections[monkeys_inspections.len() - 1] * monkeys_inspections[monkeys_inspections.len() - 2])
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let (_, monkeys) = parse_monkeys(&input).map_err(|e| e.to_string())?;

    run(monkeys, 20, |x| x / 3)
}

/// Part 2

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let (_, monkeys) = parse_monkeys(&input).map_err(|e| e.to_string())?;
    
    let mod_product = monkeys.iter().map(|monkey| monkey.test).fold(1, |acc, d| acc * d);
    
    run(monkeys, 10_000, |x| x % mod_product)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    // let input = read_input("src/input/day11_example")?;
    let input = read_input("src/input/day11")?;
    println!("Part 1: {}", part1(&input)?);

    // TODO: Refactor how to find the file.
    // let input = read_input("src/input/day11_example")?;
    let input = read_input("src/input/day11")?;
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}
