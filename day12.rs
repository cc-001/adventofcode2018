#![feature(test)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

#[macro_use]
extern crate test;

const PATTERN_LENGTH: usize = 5;

struct Note {
    num: i32,
    pattern: u8,
    output: bool
}

struct Problem {
    state: Vec<bool>,
    zero: usize,
    notes: Vec<Note>
}

impl Problem {
    pub fn num_plants(&self) -> u32 {
        self.state.iter().fold(0, |sum, val| if *val { sum + 1 } else { sum })
    }

    pub fn get_window(&self, pos: i32) -> u8 {
        let len = self.state.len() as i32;
        let index = pos + self.zero as i32;

        let p0:u8 = if (index - 2) >= len || (index - 2) < 0 { 0 } else { self.state[(index - 2) as usize] as u8 };
        let p1:u8 = if (index - 1) >= len || (index - 1) < 0 { 0 } else { self.state[(index - 1) as usize] as u8 };
        let p2:u8 = if index >= len || index < 0 { 0 } else { self.state[index as usize] as u8 };
        let p3:u8 = if (index + 1) >= len || index < 0 { 0 } else { self.state[(index + 1) as usize] as u8 };
        let p4:u8 = if (index + 2) >= len || index < 0 { 0 } else { self.state[(index + 2) as usize] as u8 };

        (p0 | (p1 << 1) | (p2 << 2) | (p3 << 3) | (p4 << 4))
    }

    pub fn set(state: &mut Vec<bool>, zero: &mut usize, pos: i32, value: bool) {
        let tmp = *zero as i32 + pos;
        if tmp < 0 {
            let to_insert = tmp.abs() as usize;
            for _x in 0..to_insert {
               state.insert(0, false);
            }
            *zero += to_insert;
        }

        let index = *zero as i32 + pos;
        assert!(index >= 0);
        while (state.len() as i32) < (index + 1) {
            state.push(false);
        }
        assert!(index < state.len() as i32);
        *state.get_mut(index as usize).unwrap() = value;
    }

    pub fn state_to_string(&self) -> String {
        let mut result = String::new();
        for x in &self.state {
            if *x { result.push('#'); } else { result.push('.'); }
        }
        result
    }

    pub fn sum_pots(&self) -> i32 {
        let mut pot = -(self.zero as i32) + 1;
        let mut result = 0;
        for val in &self.state {
            if *val {
                result += pot;
            }
            pot += 1;
        }
        result
    }
}

fn sum_pots_pattern(pattern: &Vec<bool>, first: usize) -> usize {
    let mut pot = first;
    let mut result = 0;
    for val in pattern {
        if *val {
            result += pot
        }
        pot += 1
    }
    result
}

fn parse_input(path:&str) -> Problem {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut problem = Problem { state: vec![false; 3], zero: 4, notes: Vec::new() };
    let reader = BufReader::new(file);
    let mut line_count = 0;
    let mut num = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line_count == 0 {
                    let a = scan_fmt!(&line, "initial state: {}", String);
                    for char in a.unwrap().chars() {
                        if char == '#' {
                            problem.state.push(true);
                        } else {
                            problem.state.push(false);
                        }
                    }

                } else if line_count >= 2 {
                    let (a, b) = scan_fmt!(&line, "{} => {}", String, char);
                    let pattern_str = a.unwrap();
                    assert_eq!(pattern_str.len(), PATTERN_LENGTH);
                    let mut pattern = 0u8;
                    let mut bit = 0u8;
                    for char in pattern_str.chars() {
                        if char == '#' {
                            pattern |= 1u8 << bit;
                        }
                        bit += 1;
                    }

                    let output: bool;
                    if b.unwrap() == '#' {
                        output = true;
                    } else {
                        output = false;
                    }

                    problem.notes.push(Note {num: num, pattern: pattern, output: output });
                    num += 1;
                }
                line_count += 1;
            }
            Err(e) => println!("err: {}", e)
        }
    }
    problem
}

fn compute_generation(problem: &Problem) -> (Vec<bool>, usize) {
    let len = problem.state.len() as i32;
    let mut result = Vec::new();
    let mut zero = problem.zero;
    let mut pot = -(problem.zero as i32) + 1;
    while pot < len {
        let window = problem.get_window(pot);
        let mut found = false;
        for note in &problem.notes {
            if window == note.pattern {
                found = true;
                if note.output {
                    Problem::set(&mut result, &mut zero, pot, true);
                } else {
                    Problem::set(&mut result, &mut zero, pot, false);
                }
                break;
            }
        }
        if !found {
            Problem::set(&mut result, &mut zero, pot, false);
        }
        pot += 1;
    }

    while !result.last().unwrap() {
        result.pop();
    }
    (result, zero)
}

fn part1(path: &str, n:u32) -> i32 {
    let mut problem = parse_input(path);
    println!("0: {} plants: {}", problem.state_to_string(), problem.num_plants());
    for x in 1..=n {
        let gen = compute_generation(&problem);
        problem.state = gen.0;
        problem.zero = gen.1;
        println!("{}: {} zero: {} num_plants: {}", x, problem.state_to_string(), problem.zero, problem.num_plants());
    }
    problem.sum_pots()
}

fn part2(path: &str, n:u32) -> usize {
    let mut repeat: HashSet<Vec<bool>> = HashSet::new();
    let mut problem = parse_input(path);

    let mut repeat_gen = 0;
    let mut first = 0;
    let mut pattern = Vec::new();
    for x in 1..=n {
        let gen = compute_generation(&problem);
        problem.state = gen.0;
        problem.zero = gen.1;

        let mut trimmed = problem.state.clone();
        for i in 0..trimmed.len() {
            if trimmed[i] {
                first = i;
                break;
            }
        }
        trimmed.drain(0..first);

        if repeat.contains(&trimmed) {
            repeat_gen = x;
            pattern = trimmed.clone();
            first -= problem.zero;
            break;
        } else {
            repeat.insert(trimmed);
        }
    }

    println!("repeat_gen: {} first: {}", repeat_gen, first);
    sum_pots_pattern(&pattern, (50000000000 - repeat_gen as usize) + first + 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex_0() {
    }
}

fn main() {
    //println!("result: {}", part1(r"C:\Users\lgascoigne\IdeaProjects\advent\input.txt", 100));
    println!("result: {}", part2(r"C:\Users\lgascoigne\IdeaProjects\advent\input.txt", 8000));
}
