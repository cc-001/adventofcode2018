#![feature(test)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    r: u32
}

impl Point {
    pub fn set(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn distance(&self, other: &Point) -> u32 {
        (i32::abs(self.x - other.x) + i32::abs(self.y - other.y) + i32::abs(self.z - other.z)) as u32
    }

    pub fn in_range(&self, other: &Point) -> bool {
        self.distance(other) <= self.r
    }
}

fn parse(path: &str) -> Vec<Point> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut result = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (x, y, z, r) = scan_fmt!(&line, "pos=<{},{},{}>, r={}", i32, i32, i32, u32);
                result.push(Point {x: x.unwrap(), y: y.unwrap(), z: z.unwrap(), r: r.unwrap()});
            },
            Err(e) => println!("err: {}", e)
        }
    }
    result
}

fn get_max(points: &Vec<Point>) -> Point {
    let max = points.iter().max_by_key(|x| x.r).unwrap();
    Point {x: max.x, y: max.y, z: max.z, r: max.r}
}

fn part1(path: &str) -> u32 {
    let mut points = parse(path);
    let max = get_max(&points);
    points.retain(|x| max.in_range(x));
    points.len() as u32
}

fn part2(path: &str) -> u32 {
    let nanobots = parse(path);

    let mut minx = nanobots.iter().min_by_key(|x| x.x).unwrap().x;
    let mut miny = nanobots.iter().min_by_key(|x| x.y).unwrap().y;
    let mut minz = nanobots.iter().min_by_key(|x| x.z).unwrap().z;
    let mut maxx = nanobots.iter().max_by_key(|x| x.x).unwrap().x;
    let mut maxy = nanobots.iter().max_by_key(|x| x.y).unwrap().y;
    let mut maxz = nanobots.iter().max_by_key(|x| x.z).unwrap().z;

    let mut found_best = false;
    let origin = Point{x:0, y:0, z:0, r:0};
    let mut best_cell = Point{x:0, y:0, z:0, r:0};
    let mut grid_size = maxx - minx;
    while grid_size > 0 {
        let mut max_count = 0;

        let mut x = minx;
        while x <= maxx {
            let mut y = miny;
            while y <= maxy {
                let mut z = minz;
                while z <= maxz {
                    let count = nanobots.iter().fold(0, |sum, a| {
                        let d = a.distance(&Point{x, y, z, r:0}) as i32;
                        if (d - a.r as i32) < grid_size { sum + 1 } else { sum }
                    });
                    if max_count < count {
                        max_count = count;
                        best_cell.set(x, y, z);
                        found_best = true;
                    } else if max_count == count {
                        if !found_best || (origin.distance(&Point{x, y, z, r:0}) < best_cell.distance(&origin)) {
                            best_cell.set(x, y, z);
                            found_best = true;
                        }
                    }
                    z += grid_size;
                }
                y += grid_size;
            }
            x += grid_size;
        }

        minx = best_cell.x - grid_size;
        miny = best_cell.y - grid_size;
        minz = best_cell.z - grid_size;
        maxx = best_cell.x + grid_size;
        maxy = best_cell.y + grid_size;
        maxz = best_cell.z + grid_size;

        grid_size = grid_size / 2;
    }

    origin.distance(&best_cell)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt"), 7);
    }
}

fn main() {
    println!("result: {}", part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt"));
}
