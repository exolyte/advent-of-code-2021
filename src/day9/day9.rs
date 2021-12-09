use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Point {
    height: i32,
}

struct Field(Vec<Vec<Point>>);

impl Field {
    fn get_vec(&self) -> &Vec<Vec<Point>> {
        let Field(points) = self;
        points
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let field = self.get_vec();
        let value = field[x][y].height;
        if x != 0 {
            if field[x - 1][y].height <= value {
                return false;
            }
        }
        if x != field.len() - 1 {
            if field[x + 1][y].height <= value {
                return false;
            }
        }
        if y != 0 {
            if field[x][y - 1].height <= value {
                return false;
            }
        }
        if y != field[0].len() - 1 {
            if field[x][y + 1].height <= value {
                return false;
            }
        }
        true
    }

    fn basin_part_size(&self, x: usize, y: usize, handled_points: &mut Vec<Vec<bool>>) -> i32 {
        let field = self.get_vec();
        if handled_points[x][y] || field[x][y].height == 9 {
            return 0;
        }
        let mut size = 1;
        handled_points[x][y] = true;
        if x != 0 {
            size += self.basin_part_size(x - 1, y, handled_points);
        }
        if x != field.len() - 1 {
            size += self.basin_part_size(x + 1, y, handled_points);
        }
        if y != 0 {
            size += self.basin_part_size(x, y - 1, handled_points);
        }
        if y != field[0].len() - 1 {
            size += self.basin_part_size(x, y + 1, handled_points);
        }
        size
    }
}

fn get_total_risk_level(field: &Field) -> i32 {
    let mut risk_level = 0;
    for (x, row) in field.get_vec().iter().enumerate() {
        for (y, point) in row.iter().enumerate() {
            if field.is_low_point(x, y) {
                risk_level += point.height + 1;
            }
        }
    }
    risk_level
}

fn get_basin_sizes(field: &Field) -> i32 {
    let size = field.get_vec()[0].len();
    let mut handled = vec![vec![false; size]; size];
    let mut basin_sizes = Vec::new();
    for (x, row) in field.get_vec().iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if field.is_low_point(x, y) {
                basin_sizes.push(field.basin_part_size(x, y, &mut handled));
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let field = Field(
        reader
            .lines()
            .map(|l| l.unwrap())
            .map(|s| {
                s.chars()
                    .map(|n| n.to_digit(10).unwrap())
                    .map(|d| Point { height: d as i32 })
                    .collect()
            })
            .collect(),
    );

    println!("Part 1: {}", get_total_risk_level(&field));
    println!("Part 2: {}", get_basin_sizes(&field));
}
