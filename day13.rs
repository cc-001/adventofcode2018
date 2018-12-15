#![feature(test)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate test;

#[derive(Copy, Clone, Default)]
struct Cart {
    x: u32,
    y: u32,
    orientation: char,
    next_turn: u8
}

#[derive(Default)]
struct Cell {
    x: u32,
    y: u32,
    contents: char
}

#[derive(Default)]
struct Problem {
    graph: Vec<Vec<Cell>>,
    carts: HashMap<(u32, u32), Cart>
}

impl Problem {
    #[allow(dead_code)]
    pub fn print(&self) {
        let mut string = String::new();
        for line in &self.graph {
            for cell in line {
                match self.carts.get(&(cell.x, cell.y)) {
                    Some(cart) => string.push(cart.orientation),
                    None => string.push(cell.contents)
                }
            }
            println!("{}", string);
            string.clear();
        }
    }

    pub fn tick(&mut self, remove_mode: bool) -> Option<(u32, u32)> {
        let mut carts: Vec<Cart> = Vec::new();
        for x in self.carts.values() {
            carts.push(*x);
        }

        carts.sort_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        let mut dead_cells = HashSet::new();
        for cart in carts.iter_mut() {
            self.carts.remove(&(cart.x, cart.y));
            if remove_mode && dead_cells.contains(&(cart.x, cart.y)) {
                continue;
            }

            let mut next_intersection = cart.orientation;
            match cart.orientation {
                '<' => {
                    cart.x -= 1;
                    match cart.next_turn {
                        0 => { next_intersection = 'v' }
                        2 => { next_intersection = '^' }
                        _ => {}
                    }
                }
                '>' => {
                    cart.x += 1;
                    match cart.next_turn {
                        0 => { next_intersection = '^' }
                        2 => { next_intersection = 'v' }
                        _ => {}
                    }
                }
                '^' => {
                    cart.y -= 1;
                    match cart.next_turn {
                        0 => { next_intersection = '<' }
                        2 => { next_intersection = '>' }
                        _ => {}
                    }
                }
                'v' => {
                    cart.y += 1;
                    match cart.next_turn {
                        0 => { next_intersection = '>' }
                        2 => { next_intersection = '<' }
                        _ => {}
                    }
                }
                _ => panic!("invalid orientation: {}", cart.orientation)
            }

            let row = &self.graph[cart.y as usize];
            let cell = &row[cart.x as usize];
            match cell.contents {
                '/' => {
                    match cart.orientation {
                        '<' => { cart.orientation = 'v' }
                        '>' => { cart.orientation = '^' }
                        '^' => { cart.orientation = '>' }
                        'v' => { cart.orientation = '<' }
                        _ => panic!("invalid orientation: {}", cart.orientation)
                    }
                }
                '\\' => {
                    match cart.orientation {
                        '<' => { cart.orientation = '^' }
                        '>' => { cart.orientation = 'v' }
                        '^' => { cart.orientation = '<' }
                        'v' => { cart.orientation = '>' }
                        _ => panic!("invalid orientation: {}", cart.orientation)
                    }
                }
                '+' => {
                    cart.next_turn = (cart.next_turn + 1) % 3;
                    cart.orientation = next_intersection;
                }
                '|' => {
                    assert_ne!(cart.orientation, '>');
                    assert_ne!(cart.orientation, '<');
                }
                '-' => {
                    assert_ne!(cart.orientation, '^');
                    assert_ne!(cart.orientation, 'v');
                }
                _ => panic!("invalid cell {} at ({}, {})", cell.contents, cell.x, cell.y)
            }

            if remove_mode {
                if self.carts.contains_key(&(cart.x, cart.y)) {
                    dead_cells.insert((cart.x, cart.y));
                    self.carts.remove(&(cart.x, cart.y));
                } else if !dead_cells.contains(&(cart.x, cart.y)) {
                    self.carts.insert((cart.x, cart.y), *cart);
                }
            } else {
                if self.carts.contains_key(&(cart.x, cart.y)) {
                    return Some((cart.x, cart.y));
                } else {
                    self.carts.insert((cart.x, cart.y), *cart);
                }
            }
        }
        None
    }
}

fn parse(path: &str) -> Problem {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut problem = Problem::default();
    let mut y = 0u32;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut x = 0u32;
                let mut tmp = Vec::new();
                for ch in line.chars() {
                    match ch {
                        '<' => {
                            problem.carts.insert((x, y), Cart {x: x, y: y, orientation:ch, next_turn:0});
                            tmp.push(Cell {x: x, y: y, contents: '-'})
                        }
                        '>' => {
                            problem.carts.insert((x, y), Cart {x: x, y: y, orientation:ch, next_turn:0});
                            tmp.push(Cell {x: x, y: y, contents: '-'})
                        }
                        '^' => {
                            problem.carts.insert((x, y), Cart {x: x, y: y, orientation:ch, next_turn:0});
                            tmp.push(Cell {x: x, y: y, contents: '|'})
                        }
                        'v' => {
                            problem.carts.insert((x, y), Cart {x: x, y: y, orientation:ch, next_turn:0});
                            tmp.push(Cell {x: x, y: y, contents: '|'})
                        }
                        _ => {
                            tmp.push(Cell {x: x, y: y, contents: ch})
                        }
                    }
                    x += 1;
                }
                problem.graph.push(tmp);
                y += 1;
            }
            Err(e) => println!("err: {}", e)
        }
    }
    problem
}

#[allow(dead_code)]
fn part1(path: &str) -> Option<(u32, u32)> {
    let mut problem = parse(path);
    let mut panic = 4096;
    while panic > 0 {
        let collision = problem.tick(false);
        if collision.is_some() {
            return Some(collision.unwrap());
        }
        panic -= 1;
    }
    None
}

#[allow(dead_code)]
fn part2(path: &str) -> Option<(u32, u32)> {
    let mut problem = parse(path);
    let mut panic = 1000000;
    while panic > 0 {
        problem.tick(true);
        if problem.carts.len() == 1 {
            let cart = problem.carts.iter().nth(0).unwrap().1;
            return Some((cart.x, cart.y));
        }
        panic -= 1;
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt").unwrap(), (7, 3));
    }

    #[test]
    fn test_part1_extra() {
        // additional test I added because originally passing test_part1_ex0 but input not working
        /*
            +--\
            +  |
        ->--+  |
               |
               |
               |
               |
               |
               |
               ^
        */
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test2.txt").unwrap(), (7, 1));
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt").unwrap(), (64, 57));
    }

    #[test]
    fn test_part2_ex() {
        use part2;
        assert_eq!(part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\test3.txt").unwrap(), (6, 4));
    }

    #[test]
    fn test_part2_input() {
        use part2;
        assert_eq!(part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt").unwrap(), (136, 8));
    }
}

fn main() {
    println!("result: {:?}", part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt").unwrap());
}
