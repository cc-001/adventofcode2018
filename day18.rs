#![feature(test)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate test;

#[macro_use]
extern crate scan_fmt;

#[derive(Default)]
struct Area {
    rows: Vec<Vec<char>>,
    work_row: Vec<char>,
    next: Vec<Vec<char>>
}

impl Area {
    pub fn print(&self) {
        let mut string = String::new();
        for row in &self.rows {
            string.clear();
            for ch in row {
                string.push(*ch);
            }
            println!("{}", string);
        }
    }

    pub fn sample(&self, x: usize, y: usize, width: usize, height: usize, result: &mut Vec<char>) {
        result.clear();
        let len = self.rows[0].len() as i32;
        let xmin = std::cmp::max(x as i32 - width as i32, 0) as usize;
        let xmax = std::cmp::min(x as i32 + width as i32, len - 1) as usize;
        let ymin = std::cmp::max(y as i32 - height as i32, 0) as usize;
        let ymax = std::cmp::min(y as i32 + height as i32, len - 1) as usize;
        for i in ymin..=ymax {
            for j in xmin..=xmax {
                if !(x == j && y == i) {
                    result.push(self.rows[i][j]);
                }
            }
        }
    }

    pub fn count_cells(src: &Vec<char>, key: char) -> usize {
        src.iter().fold(0usize,|sum, x| if *x == key { sum + 1 } else { sum })
    }

    pub fn count_all_cells(&self, key: char) -> usize {
        self.rows.iter().fold(0usize, |sum, x| sum + Area::count_cells(x, key))
    }

    pub fn tick(&mut self) {
        let mut y = 0;
        let mut samples= Vec::with_capacity(8);
        self.next.clear();
        for row in &self.rows {
            self.work_row.clear();
            let mut x = 0;
            for ch in row {
                self.sample(x, y, 1, 1, &mut samples);
                match ch {
                    '.' => {
                        if Area::count_cells(&samples, '|') >= 3 {
                            self.work_row.push('|');
                        } else {
                            self.work_row.push(*ch);
                        }
                    },
                    '|' => {
                        if Area::count_cells(&samples, '#') >= 3 {
                            self.work_row.push('#');
                        } else {
                            self.work_row.push(*ch);
                        }
                    },
                    '#' => {
                        if Area::count_cells(&samples, '#') < 1 || Area::count_cells(&samples, '|') < 1 {
                            self.work_row.push('.');
                        } else {
                            self.work_row.push(*ch);
                        }
                    },
                    _ => panic!("invalid cell {}, {} {}", x, y, ch)
                }
                x += 1;
            }
            self.next.push(self.work_row.clone());
            y += 1;
        }
    }

    pub fn update_from_next(&mut self) {
        let num_rows = self.rows.len();
        for x in 0..num_rows {
            self.rows[x] = self.next[x].clone();
        }
    }

    pub fn to_string(&self) -> String {
        self.rows.iter().fold(String::new(), |mut s, x| {
            for y in x { s.push(*y); }
            s })
    }
}

fn parse(path: &str, dim: usize) -> Area {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut area = Area { rows: Vec::with_capacity(dim), work_row: Vec::with_capacity(dim), next: Vec::with_capacity(dim) };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut row = Vec::with_capacity(dim);
                for ch in line.chars() {
                    row.push(ch);
                }
                area.rows.push(row);
            }
            Err(e) => println!("err: {}", e)
        }
    }
    area
}

fn part1(path: &str, dim: usize, iters: usize, print: bool) -> usize {
    let mut area = parse(path, dim);

    if print {
        println!("Initial state:");
        area.print();
    }

    for x in 1..=iters {
        area.tick();
        area.update_from_next();
        if print {
            println!("");
            if x == 1 { println!("After{} minute:", x); } else { println!("After {} minutes:", x); }
            area.print();
            println!("str: {}", area.to_string());
        }
    }

    area.count_all_cells('|') * area.count_all_cells('#')
}

fn part2(path: &str, dim: usize) -> i32 {
    let mut area = parse(path, dim);
    let mut hm = HashMap::new();
    let mut repeat = 0;
    let mut cycle_start = 0;
    let mut res_counts = vec![0];
    for x in 1..10000 {
        area.tick();
        area.update_from_next();
        let state = area.to_string();
        if hm.contains_key(&state) {
            repeat = x;
            cycle_start = *hm.get(&state).unwrap();
            println!("repeat: {} cycle_start: {}", repeat, cycle_start);
            break;
        }
        hm.insert(state, x);
        res_counts.push(area.count_all_cells('|') * area.count_all_cells('#'));
    }
    
    let loop_length = repeat - cycle_start;
    println!("loop_length: {}", loop_length);
    let index = ((1000000000 - cycle_start) % loop_length) + cycle_start;
    res_counts[index as usize] as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt", 10, 10, true), 1147);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt", 50, 10, true), 483840);
    }
}

fn main() {
    println!("result: {}", part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt", 50));
}
