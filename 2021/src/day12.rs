use std::cell::RefCell;
use std::collections::{HashSet, VecDeque, HashMap};
use std::env;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Graph {
    v: HashMap<String, Rc<Node>>,
}

impl Graph {
    fn new() -> Graph {       
        Graph { v: HashMap::new() }
    }

    fn contains(&self, name: &str) -> bool {
        self.v.contains_key(name)
    }

    fn get_node(&self, name: &str) -> Rc<Node> {
        Rc::clone(&self.v[name])
    }

    fn add_node(&mut self, name: &str) {
        self.v.insert(name.into(), Rc::new(Node::new(name.into())));
    }

    fn add_edge(&self, v1: Rc<Node>, v2: Rc<Node>) {
        v1.neighbors.borrow_mut().insert(Rc::clone(&v2));
        v2.neighbors.borrow_mut().insert(Rc::clone(&v1));
    }
}

#[derive(Eq)]
struct Node {
    name: String,
    neighbors: RefCell<HashSet<Rc<Node>>>,
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name: name,
            neighbors: RefCell::new(HashSet::new()),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}

// TODO: Working with Nodes rather than just the name strings directly seems pointless. Remove this.
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn parse_input<'a>(input: io::Lines<io::BufReader<File>>) -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::new();

    for line in input {
        let line = line?;
        let mut line = line.split('-');

        let v1 = line.next().unwrap();
        let v2 = line.next().unwrap();

        if !graph.contains(v1) {
            graph.add_node(v1);
        }

        if !graph.contains(v2) {
            graph.add_node(v2);
        }

        let v1_node = graph.get_node(v1);
        let v2_node = graph.get_node(v2);

        graph.add_edge(v1_node, v2_node);
    }

    Ok(graph)
}

struct PathInfo<'a> {
    path_so_far: &'a Vec<String>,
    small_cave_joker_used: bool,
}

fn is_small_cave(n: &Rc<Node>) -> bool {
    ('a'..='z').contains(&n.name.chars().next().unwrap())
}

fn is_node_eligible_part1(path_info: &PathInfo, n: &Rc<Node>) -> bool {
    if is_small_cave(n) && path_info.path_so_far.contains(&n.name) {
        return false;
    }
    true
}

fn is_node_eligible_part2(path_info: &PathInfo, n: &Rc<Node>) -> bool {
    if path_info.small_cave_joker_used
        && is_small_cave(n)
        && path_info.path_so_far.contains(&n.name)
    {
        return false;
    }
    true
}

fn paths(
    g: &Graph,
    path_info: &PathInfo,
    current_node: Rc<Node>,
    is_node_eligible: fn(&PathInfo, &Rc<Node>) -> bool,
) -> Vec<Vec<String>> {
    let mut p = Vec::new();

    for n in &*current_node.neighbors.borrow() {
        if n.name == "start" {
            continue;
        }
        if !is_node_eligible(&path_info, n) {
            continue;
        }

        let mut new_path = path_info.path_so_far.clone();
        new_path.push(n.name.clone());

        let new_path_info = PathInfo {
            path_so_far: &new_path,
            small_cave_joker_used: path_info.small_cave_joker_used || (is_small_cave(n) && path_info.path_so_far.contains(&n.name)),
        };

        if n.name == "end" {
            p.push(new_path);
        } else {
            p.append(&mut paths(
                g,
                &new_path_info,
                Rc::clone(n),
                is_node_eligible,
            ));
        }
    }

    p
}

fn part1(
    input: io::Lines<io::BufReader<File>>,
    is_node_eligible: fn(&PathInfo, &Rc<Node>) -> bool,
) -> Result<usize, Box<dyn Error>> {
    let graph = parse_input(input)?;
    let start_node = graph.get_node("start");

    let mut start_path = Vec::new();
    start_path.push("start".into());

    let path_info = PathInfo {
        path_so_far: &start_path,
        small_cave_joker_used: false,
    };

    let p = paths(&graph, &path_info, start_node, is_node_eligible);

    // for path in &p {
    //     println!("{:?}", path);
    // }

    Ok(p.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello AOC 2021!");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day12")?;
    println!("Part 1: {}", part1(lines, is_node_eligible_part1)?);

    // TODO: Refactor how to find the file.
    let lines = read_input("src/input/day12")?;
    println!("Part 2: {}", part1(lines, is_node_eligible_part2)?);

    Ok(())
}
