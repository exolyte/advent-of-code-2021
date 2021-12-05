use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_COORDINATE: usize = 1000;

struct Line {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Line {
    fn mark_world_horiz(&self, world: &mut World) {
        if self.x1 != self.x2 && self.y1 != self.y2 {
            return;
        }
        let x1 = min(self.x1, self.x2);
        let x2 = max(self.x1, self.x2);
        let y1 = min(self.y1, self.y2);
        let y2 = max(self.y1, self.y2);
        for x in x1..=x2 {
            for y in y1..=y2 {
                world.mark_coordinate(x, y);
            }
        }
    }

    fn mark_world_angle(&self, world: &mut World) {
        if self.x1 == self.x2 || self.y1 == self.y2 {
            return;
        }
        let n = (self.x1 as i32 - self.x2 as i32).abs() as usize;
        for i in 0..=n {
            let x;
            let y;
            if self.x1 < self.x2 {
                x = self.x1 + i;
                if self.y1 < self.y2 {
                    y = self.y1 + i;
                } else {
                    y = self.y1 - i;
                }
            } else {
                x = self.x2 + i;
                if self.y2 < self.y1 {
                    y = self.y2 + i;
                } else {
                    y = self.y2 - i;
                }
            }
            world.mark_coordinate(x, y);
        }
    }
}

struct World {
    coordinates: Vec<Vec<i32>>,
}

impl World {
    fn mark_coordinate(&mut self, x: usize, y: usize) {
        self.coordinates[x][y] += 1;
    }

    fn marked_count(&self) -> i32 {
        let mut count = 0;

        for row in &self.coordinates {
            count += row.iter().filter(|&&h| h >= 2).count();
        }
        count as i32
    }
}

fn get_marked_count(lines: &Vec<Line>, world: &mut World, all: bool) -> i32 {
    for line in lines {
        line.mark_world_horiz(world);
        if all {
            line.mark_world_angle(world)
        }
    }
    world.marked_count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let input_lines = reader.lines().map(|l| l.unwrap());
    let mut lines = Vec::new();
    for line in input_lines {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        let idxs: Vec<Vec<usize>> = points
            .iter()
            .map(|p| p.split(",").map(|c| c.parse::<usize>().unwrap()).collect())
            .collect();
        lines.push(Line {
            x1: idxs[0][0],
            x2: idxs[1][0],
            y1: idxs[0][1],
            y2: idxs[1][1],
        })
    }

    let mut world = World {
        coordinates: vec![vec![0; MAX_COORDINATE]; MAX_COORDINATE],
    };
    println!("Part 1: {}", get_marked_count(&lines, &mut world, false));
    world = World {
        coordinates: vec![vec![0; MAX_COORDINATE]; MAX_COORDINATE],
    };
    println!("Part 2: {}", get_marked_count(&lines, &mut world, true));
}
