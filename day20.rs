#![feature(test)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate test;

#[allow(dead_code)]
fn solution(path: &str) -> (u32, u32) {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut positions = Vec::new();
    let mut coord = (0, 0);

    let mut distance_field = HashMap::new();
    distance_field.insert(coord, 0u32);

    let mut delta_coord = HashMap::new();
    delta_coord.insert('N', (0, -1));
    delta_coord.insert('S', (0, 1));
    delta_coord.insert('E', (-1, 0));
    delta_coord.insert('W', (1, 0));

    let reader = BufReader::new(file);
    let mut lc = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                assert_eq!(lc, 0);
                for ch in line.chars() {
                    match ch {
                        '(' => {
                            positions.push(coord);
                        },
                        ')' => {
                            coord = positions.pop().unwrap();
                        },
                        '|' => {
                            coord = *positions.last().unwrap();
                        },
                        'N' | 'E' | 'S' | 'W' => {
                            let mut new_pos = *delta_coord.get(&ch).unwrap();
                            new_pos.0 += coord.0;
                            new_pos.1 += coord.1;
                            let curr_dist = *distance_field.get(&coord).unwrap();
                            if distance_field.contains_key(&new_pos) {
                                let dist = std::cmp::min(*distance_field.get(&new_pos).unwrap(), curr_dist + 1);
                                distance_field.insert( new_pos, dist);
                            } else {
                                distance_field.insert(new_pos, curr_dist + 1);
                            }
                            coord = new_pos;
                        },
                        _ => {}
                    }
                }
                lc += 1;
            }
            Err(e) => println!("err: {}", e)
        }
    }

    (*distance_field.values().max().unwrap(), distance_field.values().fold(0, |sum, x| if *x >= 1000 { sum + 1 } else { sum }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
        use solution;
        assert_eq!(solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt").0, 3);
    }

    #[test]
    fn test_part1_ex1() {
        use solution;
        assert_eq!(solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\test2.txt").0, 10);
    }

    #[test]
    fn test_part1_ex2() {
        use solution;
        assert_eq!(solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\test3.txt").0, 18);
    }

    #[test]
    fn test_part1_ex3() {
        use solution;
        assert_eq!(solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\test4.txt").0, 23);
    }

    #[test]
    fn test_part1_ex4() {
        use solution;
        assert_eq!(solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\test5.txt").0, 31);
    }
}

fn main() {
    println!("result: {:?}", solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt"));
}
