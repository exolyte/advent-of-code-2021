use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

enum FoldDirection {
    Vertical,
    Horizontal,
}

struct Fold {
    dir: FoldDirection,
    n: i32,
}

impl FromStr for Fold {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split("=").collect();
        let dir = match split[0] {
            "x" => FoldDirection::Horizontal,
            "y" => FoldDirection::Vertical,
            _ => return Err(ParseError),
        };
        let n = split[1].parse::<i32>().unwrap();
        Ok(Fold { dir, n })
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn fold_line(&mut self, fold: &Fold) {
        let n = match fold.dir {
            FoldDirection::Horizontal => &mut self.x,
            FoldDirection::Vertical => &mut self.y,
        };
        if *n < fold.n {
            return;
        }
        *n = fold.n - (*n - fold.n);
    }
}

fn first_fold(mut points: Vec<Point>, folds: &Vec<Fold>, first_only: bool) -> i32 {
    for fold in folds {
        for point in &mut points {
            point.fold_line(fold);
        }
        if first_only {
            break;
        }
    }
    let uniques: HashSet<Point> = HashSet::from_iter(points.into_iter());
    let length = uniques.len() as i32;
    if !first_only {
        let mut grid = [["."; 39]; 6];
        for p in uniques {
            grid[p.y as usize][p.x as usize] = "■";
        }
        for line in grid {
            println!("{:?}", line);
        }
    }
    length
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut input_lines = reader.lines().map(|l| l.unwrap()).into_iter();
    let mut points = Vec::new();

    loop {
        let line = input_lines.next().unwrap();
        if line == "" {
            break;
        }
        let split: Vec<i32> = line
            .split(",")
            .collect::<Vec<_>>()
            .iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        points.push(Point {
            x: split[0],
            y: split[1],
        })
    }

    let mut folds = Vec::new();
    for line in input_lines {
        let split: Vec<&str> = line.split(" along ").collect();
        folds.push(split[1].parse::<Fold>().unwrap());
    }

    let points2 = points.clone();
    println!("Part 1: {}", first_fold(points, &folds, true));
    println!("Part 2: {}", first_fold(points2, &folds, false));
}
