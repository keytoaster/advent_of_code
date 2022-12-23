use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Line {
    ChangeDir { target: String },
    Directory { name: String },
    File { name: String, size: u32 },
    Ls,
}

fn name(input: &str) -> IResult<&str, &str> {
    is_not(" \r\n")(input)
}

fn change_dir(input: &str) -> IResult<&str, Line> {
    let (_, target) = preceded(tag("$ cd "), name)(input)?;
    Ok((
        "",
        Line::ChangeDir {
            target: target.to_string(),
        },
    ))
}

fn ls(input: &str) -> IResult<&str, Line> {
    let (_, _) = tag("$ ls")(input)?;
    Ok(("", Line::Ls))
}

fn directory(input: &str) -> IResult<&str, Line> {
    let (_, name) = preceded(tag("dir "), name)(input)?;
    Ok((
        "",
        Line::Directory {
            name: name.to_string(),
        },
    ))
}

fn file(input: &str) -> IResult<&str, Line> {
    let (_, (size, name)) = separated_pair(u32, tag(" "), name)(input)?;
    Ok((
        "",
        Line::File {
            name: name.to_string(),
            size,
        },
    ))
}

fn parse_line(input: &str) -> Result<Line, Box<dyn Error>> {
    let (_, line) =
        alt((change_dir, ls, directory, file))(input).or(Err("could not parse line"))?;
    Ok(line)
}

// TODO: These Option fields look more like a protobuf. An Enum would probably be more Rusty.
#[derive(Debug)]
struct AocDirent {
    name: String,
    file: Option<Rc<AocFile>>,
    directory: Option<Rc<AocDir>>,
}

#[derive(Debug)]
struct AocFile {
    size: u32,
}

#[derive(Debug)]
struct AocDir {
    dirents: RefCell<Vec<Rc<AocDirent>>>,
    parent: Option<Rc<AocDir>>,
}

impl AocDir {
    fn new(parent: Option<Rc<AocDir>>) -> AocDir {
        AocDir {
            dirents: RefCell::new(Vec::new()),
            parent: parent,
        }
    }
}

fn parse_input(input: io::Lines<io::BufReader<File>>) -> Result<Rc<AocDirent>, Box<dyn Error>> {

    let root_dirent = Rc::new(AocDirent { 
        name: "/".to_string(),
        file: None,
        directory: Some(Rc::new(AocDir::new(None))),
    });

    let mut cwd = root_dirent.directory.as_ref().unwrap().clone();
    let mut cwd_str = vec![root_dirent.name.clone()];

    for line in input {
        let line = line?;
        let foo = parse_line(&line)?;

        println!("foo: {:?}", foo);

        match &foo {
            Line::ChangeDir { target } => {
                if target == "/" {
                    cwd = root_dirent.directory.as_ref().unwrap().clone();
                    cwd_str = vec![root_dirent.name.clone()];
                }
                else if target == ".." {
                    cwd = cwd.parent.as_ref().unwrap().clone();
                    cwd_str.pop();
                } else {
                    let mut new_dir = None;
                    
                    for dirent in &*cwd.dirents.borrow() {
                        if dirent.name == *target {
                            if let Some(directory) = &dirent.directory {
                                new_dir = Some(directory.clone());
                                cwd_str.push(dirent.name.clone());
                                break;
                            } else {
                                return Err("attempted to cd into non-directory".into());
                            }                            
                        }
                    }

                    if let Some(new_dir) = new_dir {
                        cwd = new_dir;
                    } else {
                        return Err(format!("{} does not exist", target).into());
                    }
                }

                println!("New CWD: {:?}", cwd_str);
            },
            Line::Directory { name } => {
                let new_directory = Rc::new(AocDir::new(Some(cwd.clone())));

                cwd.dirents.borrow_mut().push(Rc::new(AocDirent { 
                    name: name.clone(),
                    file: None,
                    directory: Some(new_directory.clone()),
                }));
            }
            Line::File { name, size } => {
                cwd.dirents.borrow_mut().push(Rc::new(AocDirent { 
                    name: name.clone(),
                    file: Some(Rc::new(AocFile { size: *size })),
                    directory: None,
                }));
            }
            Line::Ls => {
                // Nothing to do. Just stay in this directory, dir entries to follow.
            }
        };
    }

    Ok(root_dirent)
}

// Alternatives considered:
// * Calculate the sizes and the score at the same time in the same walk. But that's messy.
//   Keep the concerns separate, even it means performing re-calculations.
//
// * Store the size of a directory when building the tree. But real filesystems don't do
//   this: The size of a directory is the number of dirents. As a challenge, we try to
//   pretend we're dealing with a real POSIX fs and only do operations that would be possible
//   on those. Even if it makes the AoC puzzle much harder.
//
// * Cache directory sizes in the directory structure to avoid re-calculations. But this is
//   unreasonable for real filesystems: 
//   https://superuser.com/questions/501453/why-doesnt-ext4-cache-directory-size.
//
// * Cache directory sizes in a separate structure, e.g. in a duplicate tree with the same
//   structure, or a hash map. This would probably be the cleanest approach.
//
// But it turns out that the AoC puzzle input is nowhere large enough for any of this to
// be a concern anyway.

fn walk<F>(dirent: &AocDirent, cb: &mut F)
where F: FnMut(&AocDirent)  {
    if let Some(_file) = &dirent.file {
        cb(dirent);
    } else if let Some(directory) = &dirent.directory {
        for child in &*directory.dirents.borrow() {
            walk(child, cb);
        }

        cb(dirent);
    } else {
        panic!("unexpected");
    }   
}

fn dirent_size(dirent: &AocDirent) -> u32 {
    if let Some(file) = &dirent.file {
        return file.size;
    } else if let Some(directory) = &dirent.directory {
        return directory.dirents.borrow().iter().fold(
            0, |total, dir| total + dirent_size(dir));
    } else {
        panic!("unexpected");
    }
}

fn part1(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let root_dirent = parse_input(input)?;

    let mut sum = 0;

    walk(&root_dirent,  &mut|dirent| {
        if dirent.directory.is_some() {
            let size = dirent_size(dirent);
            if size < 100_000 {
                sum += size;
            }
        }
    });

    Ok(sum)
}


/// Part 2

fn part2(input: io::Lines<io::BufReader<File>>) -> Result<u32, Box<dyn Error>> {
    let root_dirent = parse_input(input)?;

    let outermost_size = dirent_size(&root_dirent);

    const TOTAL_DISK_SPACE: u32 = 70_000_000;
    const REQUIRED_FREE_SPACE: u32 = 30_000_000;

    let need_to_delete = REQUIRED_FREE_SPACE - (TOTAL_DISK_SPACE - outermost_size);

    let mut smallest_directory_size = u32::MAX;

    walk(&root_dirent,  &mut|dirent| {
        if dirent.directory.is_some() {
            let size = dirent_size(dirent);
            if size > need_to_delete && size < smallest_directory_size {
                smallest_directory_size = size;
            }
        }
    });

    Ok(smallest_directory_size)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2022!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day07")?;
    // let lines = read_input("src/input/day07")?;
    println!("Part 1: {}", part1(lines)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day07")?;
    // let lines = read_input("src/input/day07")?;
    println!("Part 2: {}", part2(lines)?);

    Ok(())
}
