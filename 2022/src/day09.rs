use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;
use nom::character::complete::i32;
use std::collections::HashSet;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Action {
    direction: Direction,
    steps: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_action_up(input: &str) -> IResult<&str, Action> {
    let (input, num) = preceded(tag("U "), i32)(input)?;
    Ok((input, Action { direction: Direction::Up, steps: num }))
}

fn parse_action_left(input: &str) -> IResult<&str, Action> {
    let (input, num) = preceded(tag("L "), i32)(input)?;
    Ok((input, Action { direction: Direction::Left, steps: num }))
}

fn parse_action_right(input: &str) -> IResult<&str, Action> {
    let (input, num) = preceded(tag("R "), i32)(input)?;
    Ok((input, Action { direction: Direction::Right, steps: num }))
}

fn parse_action_down(input: &str) -> IResult<&str, Action> {
    let (input, num) = preceded(tag("D "), i32)(input)?;
    Ok((input, Action { direction: Direction::Down, steps: num }))
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((parse_action_up, parse_action_left, parse_action_right, parse_action_down))(input)
}

// We need Pos to be Copy to make initializing [Pos; N] easier.
// Would be cool to rely on Pos: Default, but [T; N] for a const generic N is not Default yet at the time of writing.
// https://rust-lang.github.io/project-const-generics/vision/status_quo/array_default.html
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, Default)]
struct Pos {
    x: i32,
    y: i32,
}

struct State<const N: usize> {
    knots: [Pos; N],
    // Coordinates can become negative (making indexing into a 2d array rather painful),
    // the world is rather sparsely populated, and we need cheap lookups, so a hash set seems
    // like a good trade-off.
    visited_positions: HashSet<Pos>,
}

impl<const N: usize> State<N> {

    fn new() -> State<N> {
        State {
            knots: [Pos { x: 0, y: 0 }; N],
            visited_positions: HashSet::new(),
        }
    }

    fn knots_touching(&self, i: usize, j: usize) -> bool {
        self.knots[j].x >= self.knots[i].x - 1 && self.knots[j].x <= self.knots[i].x + 1 &&
        self.knots[j].y >= self.knots[i].y - 1 && self.knots[j].y <= self.knots[i].y + 1
    }

    fn step_all_knots(&mut self) {
        for i in 1..N {
            let follower = i;
            let leader = i - 1;

            while ! self.knots_touching(leader, follower) {
                if self.knots[follower].x < self.knots[leader].x {
                    self.knots[follower].x += 1;
                } else if self.knots[follower].x > self.knots[leader].x {
                    self.knots[follower].x -= 1;
                }

                if self.knots[follower].y < self.knots[leader].y {
                    self.knots[follower].y += 1;
                } else if self.knots[follower].y > self.knots[leader].y {
                    self.knots[follower].y -= 1;
                }

                if i == N - 1 {
                    self.mark_tail_position_as_visited();
                }
            }
        }
    }

    fn feed(&mut self, action: &Action) {
        for _ in 0..action.steps {
            match action.direction {
                Direction::Up => {
                    self.knots[0].y += 1;
                }
                Direction::Left => {
                    self.knots[0].x -= 1;
                }
                Direction::Right => {
                    self.knots[0].x += 1;
                }
                Direction::Down => {
                    self.knots[0].y -= 1;
                }
            }        
            self.step_all_knots();
        }
    }

    fn mark_tail_position_as_visited(&mut self) {
        self.visited_positions.insert(self.knots[N - 1].clone());
    }

    fn num_unique_visited_positions(&self) -> u32 {
        u32::try_from(self.visited_positions.len()).unwrap()
    }

    // Very hacky.
    fn print(&self) {
        let mut field = [['.'; 50]; 50];
    
        for (i, knot) in self.knots.iter().enumerate() {
            field[usize::try_from(knot.x + 25).unwrap()][usize::try_from(knot.y + 25).unwrap()] = char::from_digit(i.try_into().unwrap(), 10).unwrap();
        }
    
        for i in 0..50 {
            for j in 0..50 {
                print!("{}", field[j][50 - i - 1]);
            }
            println!();
        }
    }
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut state = State::<2>::new();
    state.mark_tail_position_as_visited();

    for line in input {
        let line = line?;
        let (_, action) = parse_action(&line).or(Err("could not parse action"))?;
        state.feed(&action);
    }
    
    Ok(state.num_unique_visited_positions())
}

/// Part 2

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut state = State::<10>::new();
    state.mark_tail_position_as_visited();

    for line in input {
        let line = line?;
        let (_, action) = parse_action(&line).or(Err("could not parse action"))?;
        state.feed(&action);
    }
    
    Ok(state.num_unique_visited_positions())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    // let lines = read_input("src/input/day09_example")?;
    let lines = read_input("src/input/day09")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    // let lines = read_input("src/input/day09_example2")?;
    let lines = read_input("src/input/day09")?;
    println!("Part 2: {}", part2(lines)?);

    // 2386
    Ok(())
}
