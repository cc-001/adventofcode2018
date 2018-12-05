use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// v2 fixed
fn strip(input:&Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let end = input.len();
    let mut it = 0;
    while it < end {
        let i0 = input[it] as i32;
        if it >= end-1 {
            result.push(i0 as u8);
            return result;
        }

        let i1 = input[it+1] as i32;
        if (i0 - i1).abs() != 32 {
            result.push(i0 as u8);
            it += 1;
        } else {
            it += 2;
        }
    }
    return result;
}

// added after looking at work answers, stack version
fn strip2(input:&Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let end = input.len();
    for it in 0..end {
        let len = result.len();
        if len == 0 {
            result.push(input[it]);
        } else {
            let u0 = input[it];
            let i0 = u0 as i32;
            let i1 = result[len-1] as i32;
            if (i0 - i1).abs() == 32 {
                result.pop();
            } else {
                result.push(u0);
            }
        }
    }
    return result;
}

#[allow(dead_code)]
fn part1() {
    let path = "C:\\Users\\Igascoigne\\advent2018\\dec_01_01\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut line_count = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                assert_eq!(line_count, 0);
                assert!(line.is_ascii());
                let original_input = line.into_bytes();
                let mut result = original_input.clone();
                let mut length= result.len();
                let mut passes = 0;
                loop {
                    result = strip(&result);
                    passes += 1;
                    let new_length = result.len();
                    if new_length == length {
                        break;
                    }
                    length = new_length;
                }
                line_count += 1;
                println!("passes: {}", passes);
                println!("result: {}", length);
                println!("result2: {}", strip2(&original_input).len());
            }
            Err(e) => println!("err: {}", e)
        }
    }
}

#[allow(dead_code)]
fn part2() {
    let path = "C:\\Users\\Igascoigne\\advent2018\\dec_01_01\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut line_count = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                assert_eq!(line_count, 0);
                assert!(line.is_ascii());
                let original_input = line.into_bytes();
                let mut best_len = std::usize::MAX;
                for i in 0..26 {
                    let mut result = original_input.clone();
                    let c0 = i as u8 + 'A' as u8;
                    let c1 = i as u8 + 'A' as u8 + 32u8;
                    result.retain(|&x| x != c0 && x != c1);
                    let mut length= result.len();
                    loop {
                        result = strip(&result);
                        let new_length = result.len();
                        if new_length == length {
                            break;
                        }
                        length = new_length;
                    }
                    if length < best_len {
                        best_len = length;
                    }
                    /* faster
                    let length = strip2(&result).len();
                    if length < best_len {
                        best_len = length;
                    }
                    */
                }
                line_count += 1;
                println!("result: {}", best_len);

            }
            Err(e) => println!("err: {}", e)
        }
    }
}

fn main() {
    part2();
    //println!("{}", String::from_utf8(strip(&"aA".to_string().into_bytes())).unwrap());
}
