use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// https://adventofcode.com/2018/day/1#part1
#[allow(dead_code)]
fn part1() {
    let path = "C:\\Users\\Igascoigne\\advent2018\\dec_01_01\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                sum += line.parse::<i32>().unwrap();
            }
            Err(e) => println!("err: {}", e)
        }
    }
    println!("sum: {}", sum);
}

// https://adventofcode.com/2018/day/1#part2
#[allow(dead_code)]
fn part2() {
    let path = "C:\\Users\\Igascoigne\\advent2018\\dec_01_01\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                vec.push(line.parse::<i32>().unwrap());
            }
            Err(e) => println!("err: {}", e)
        }
    }

    let count = vec.len();
    let mut index : usize = 0;
    let mut map = HashMap::new();
    let mut frequency = 0;
    let mut iters = 0;
    loop {
        frequency += vec[index];
        if map.contains_key(&frequency) {
            println!("freq: {}", frequency);
            break;
        } else {
            map.insert(frequency, true);
            iters += 1;
        }
        index = (index + 1) % count;
    }

    println!("count: {}", count);
    println!("iters: {}", iters);
}

fn main() {
    part2();
}
