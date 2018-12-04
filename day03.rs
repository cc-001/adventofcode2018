#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn part1() {
    let path = "C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut cols = HashMap::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (id, x, y, w, h) = scan_fmt!(&line, "#{} @ {},{}: {}x{}", i32, i32, i32, i32, i32);
                let xx: i32 = x.unwrap();
                let yy: i32 = y.unwrap();
                let ww: i32 = w.unwrap();
                let hh: i32 = h.unwrap();
                for i in xx..(xx + ww) {
                    if !cols.contains_key(&i) {
                        cols.insert(i, HashMap::new());
                    }
                    let mut col = cols.get_mut(&i).unwrap();
                    for j in yy..(yy + hh) {
                        if col.contains_key(&j) {
                            *col.get_mut(&j).unwrap() += 1;
                        } else {
                            col.insert(j, 1);
                        }
                    }
                }
            }
            Err(e) => println!("err: {}", e)
        }
    }

    let mut overlaps = 0;
    for (k, v) in cols {
        for (kk, vv) in v {
            if vv >= 2 {
                overlaps += 1;
            }
        }
    }
    println!("result: {0}", overlaps);
}

fn part2() {
    let path = "C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut cols = HashMap::new();
    let mut rects = HashMap::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (id, x, y, w, h) = scan_fmt!(&line, "#{} @ {},{}: {}x{}", i32, i32, i32, i32, i32);
                let iid: i32 = id.unwrap();
                let xx: i32 = x.unwrap();
                let yy: i32 = y.unwrap();
                let ww: i32 = w.unwrap();
                let hh: i32 = h.unwrap();
                rects.insert(iid, (xx, yy, ww, hh));
                for i in xx..(xx + ww) {
                    if !cols.contains_key(&i) {
                        cols.insert(i, HashMap::new());
                    }
                    let mut col = cols.get_mut(&i).unwrap();
                    for j in yy..(yy + hh) {
                        if col.contains_key(&j) {
                            *col.get_mut(&j).unwrap() += 1;
                        } else {
                            col.insert(j, 1);
                        }
                    }
                }
            }
            Err(e) => println!("err: {}", e)
        }
    }

    let mut found = Vec::new();
    for (k, v) in rects {
        let xx = v.0;
        let yy = v.1;
        let ww = v.2;
        let hh = v.3;

        let mut multi = false;
        for i in xx..(xx + ww) {
            let mut col = cols.get_mut(&i).unwrap();
            for j in yy..(yy + hh) {
                if col.contains_key(&j) {
                    if *col.get(&j).unwrap() > 1 {
                        multi = true;
                        break;
                    }
                }
            }
        }

        if !multi {
            found.push(k);
        }
    }
    assert_eq!(found.len(), 1);
    println!("result: {}", found[0]);
}

fn main() {
    part2();
}
