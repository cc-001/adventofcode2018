use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// https://adventofcode.com/2018/day/2#part1
#[allow(dead_code)]
fn part1() {
    let path = "C:\\Users\\Igascoigne\\advent2018\\dec_02_01\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut two_count = 0;
    let mut three_count = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut array: [i32; 256] = [0; 256];
                for c in line.chars() {
                    let index = c as usize;
                    array[index] += 1;
                }
                let mut a = 0;
                let mut b = 0;
                for x in array.iter() {
                    let cmp = *x;
                    if cmp == 2 {
                        a = 1;
                    } else if cmp == 3 {
                        b = 1;
                    }
                }
                two_count += a;
                three_count += b;
            }
            Err(e) => println!("err: {}", e)
        }
    }
    println!("two_count: {}", two_count);
    println!("three_count: {}", three_count);
    println!("id: {}", two_count * three_count);
}

// https://adventofcode.com/2018/day/2#part2
#[allow(dead_code)]
fn compare_ids(a:&String, b:&String) -> String {
    let mut found = false;
    let mut diff_index = 0;
    let len = a.len();
    assert_eq!(len, b.len());
    for i in 0..len {
        let c = a.chars().nth(i).unwrap();
        let c2 = b.chars().nth(i).unwrap();
        if c != c2 {
            if !found {
                diff_index = i;
                found = true;
            } else {
                return String::from("");
            }
        }
    }
    if found {
        let mut ret = a.clone();
        ret.remove(diff_index);
        return ret;
    }
    return String::from("");
}

#[allow(dead_code)]
fn part2() {
    let path = "C:\\Users\\Igascoigne\\advent2018\\dec_02_01\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut id_vec = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                id_vec.push(line);
            }
            Err(e) => println!("err: {}", e)
        }
    }

    let mut results = 0;
    while !id_vec.is_empty() {
        let a = id_vec.pop().unwrap();
        for b in &id_vec {
            let result = compare_ids(&a, &b);
            if !result.is_empty() {
                println!("result: {}", result);
                results += 1;
            }
        }
    }
    assert_eq!(results, 1);
}

fn main() {
    part2();
}
