use std::cell::{RefCell, Cell};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

const START_MARKER: &str = "start";
const END_MARKER: &str = "end";

struct Node<'a> {
    name: String,
    large: bool,
    visited_count: Cell<i32>,
    edges: Vec<Rc<RefCell<Node<'a>>>>,
}

impl Node<'_> {
    fn new(name: String, large: bool) -> Rc<RefCell<Node<'static>>> {
        Rc::new(RefCell::new(Node {
            name,
            large,
            visited_count: Cell::new(0),
            edges: Vec::new(),
        }))
    }

    fn paths(&self, allow_double: bool) -> i32 {
        if self.name == START_MARKER && self.visited_count.get() > 0 {
            return 0;
        }
        if self.name == END_MARKER {
            return 1;
        }
        let n = self.visited_count.get();
        self.visited_count.set(n + 1);
        let mut path_count = 0;
        for node in &self.edges {
            if node.borrow().large {
                path_count += node.borrow().paths(allow_double);
            } else {
                let count = node.borrow().visited_count.get();
                if count == 0 {
                    path_count += node.borrow().paths(allow_double);
                } else {
                    if count == 1 && allow_double {
                        path_count += node.borrow().paths(false);
                    }
                }
            }
        }
        self.visited_count.set(n);
        path_count
    }
}

fn get_total_paths(start: &Rc<RefCell<Node>>, allow_double: bool) -> i32 {
    start.borrow().paths(allow_double)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut nodes_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    for line in &lines {
        let split: Vec<String> = line.split("-").map(|s| s.to_string()).collect();
        if !nodes_map.contains_key(&split[0]) {
            let large = &split[0].to_uppercase() == &split[0];
            nodes_map.insert(split[0].clone(), Node::new(split[0].to_string(), large));
        }
        if !nodes_map.contains_key(&split[1]) {
            let large = &split[1].to_uppercase() == &split[1];
            nodes_map.insert(split[1].clone(), Node::new(split[1].to_string(), large));
        }
    }
    for line in &lines {
        let split: Vec<String> = line.split("-").map(|s| s.to_string()).collect();
        let i = nodes_map.get(&split[0]).unwrap();
        let j = nodes_map.get(&split[1]).unwrap();
        i.borrow_mut().edges.push(j.clone());
        j.borrow_mut().edges.push(i.clone());
    }
    let start = nodes_map.get(START_MARKER).unwrap();
    println!("Part 1: {}", get_total_paths(start, false));
    println!("Part 2: {}", get_total_paths(start, true));
}
