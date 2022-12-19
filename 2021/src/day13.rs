use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;

// Maybe a type alias would've been better.
struct Board {
    data: Vec<Vec<char>>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Board {
    fn new(x_max: usize, y_max: usize) -> Board {
        Board {
            data: vec![vec!['.'; x_max]; y_max],
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }

    fn resize(&mut self, new_x: usize, new_y: usize) {
        for i in 0..=new_x {
            self.data[i].resize(new_y + 1, 'f');
        }
        self.data.resize(new_x + 1, Vec::new());
    }

    fn print(&self) {
        println!("x: {} to {}", self.x_min, self.x_max);
        println!("y: {} to {}", self.y_min, self.y_max);
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                print!("{}", self.data[x][y]);
            }
            println!("");
        }
    }
}

impl Index<usize> for Board {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &mut io::Lines<io::BufReader<File>>) -> Result<Board, Box<dyn Error>> {
    let mut board = Board::new(1500, 1500);

    while let Some(line) = input.next() {
        let line = line?;

        if line == "" {
            break;
        }

        let mut foo = line.split(',').map(|s| s.parse::<usize>());

        let x = foo.next().unwrap()?;
        let y = foo.next().unwrap()?;

        if x > board.x_max {
            board.x_max = x;
        }
        if y > board.y_max {
            board.y_max = y;
        }

        board[x][y] = '#';
    }

    board.resize(board.x_max, board.y_max);

    Ok(board)
}

fn run_step(
    board: &mut Board,
    line: &str,
) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"^fold along (\w)=(\d+)$")?;

    let c = re.captures(&line).unwrap();

    match &c[1] {
        "x" => {
            let fold = c[2].parse::<usize>()?;

            if fold < (board.x_max - board.x_min) / 2 {
                // fold to the right
                for i in board.x_min..fold {
                    for y in board.y_min..=board.y_max {
                        if board[i][y] == '#' {
                            board[fold + fold - i][y] = '#';
                        }
                    }
                }
                board.x_min = fold + 1;
            } else {
                // fold to the left
                for i in (fold + 1)..=board.x_max {
                    for y in board.y_min..=board.y_max {
                        if board[i][y] == '#' {
                            board[fold - (i - fold)][y] = '#';
                        }
                    }
                }
                board.x_max = fold - 1;
            }
        }
        "y" => {
            let fold = c[2].parse::<usize>()?;

            if fold < (board.y_max - board.y_min) / 2 {
                // fold to the bottom
                for i in board.y_min..fold {
                    for x in board.x_min..=board.x_max {
                        if board[x][i] == '#' {
                            board[x][fold + fold - i] = '#';
                        }
                    }
                }
                board.y_min = fold + 1;
            } else {
                // fold to the top
                for i in (fold + 1)..=board.y_max {
                    for x in board.x_min..=board.x_max {
                        if board[x][i] == '#' {
                            board[x][fold - (i - fold)] = '#';
                        }
                    }
                }
                board.y_max = fold - 1;
            }
        }
        _ => panic!("unexpected character on fold along line"),
    }

    Ok(())
}
fn part1(mut input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let mut board = parse_input(&mut input)?;

    board.print();

    let line = input.next().unwrap()?;
    run_step(&mut board, &line)?;

    println!("step done");
    board.print();

    // }

    Ok(count_dots(&board))
}

fn part2(mut input: io::Lines<io::BufReader<File>>) -> Result<(), Box<dyn Error>> {
    let mut board = parse_input(&mut input)?;

    board.print();

    while let Some(line) = input.next() {
        run_step(&mut board, &line?)?;

        println!("step done");
        board.print();
    }

    Ok(())
}

fn count_dots(board: &Board) -> u32 {
    let mut num = 0;

    for y in board.y_min..=board.y_max {
        for x in board.x_min..=board.x_max {
            if board[x][y] == '#' {
                num += 1;
            }
        }
    }

    num
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day13")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day13")?;
    println!("Part 2:");
    part2(lines)?;

    Ok(())
}
