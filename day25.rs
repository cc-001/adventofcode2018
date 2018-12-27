#![feature(test)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate num_traits;
use num_traits::Float;

extern crate kdtree;
use kdtree::KdTree;

const DIMS: usize = 4;

#[macro_use]
extern crate scan_fmt;

fn manhattan<T: Float>(a: &[T], b: &[T]) -> T {
    debug_assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| (*x - *y).abs().round()).fold(T::zero(), ::std::ops::Add::add)
}

fn parse(path: &str) -> Vec<([f64; DIMS], usize)> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut result = Vec::new();
    let reader = BufReader::new(file);
    let mut index = 0usize;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (x, y, z, a) = scan_fmt!(&line, "{},{},{},{}", i32, i32, i32, i32);
                let point: [f64; DIMS] = [x.unwrap() as f64, y.unwrap() as f64, z.unwrap() as f64, a.unwrap() as f64];
                result.push((point, index));
                index += 1;
            },
            Err(e) => println!("err: {}", e)
        }
    }
    result
}

fn add_recursive(cluster: &mut HashSet<usize>, assigned: &mut HashSet<usize>, points: &Vec<([f64; DIMS], usize)>, tree: &KdTree<f64, usize, &[f64; DIMS]>, point: &([f64; DIMS], usize)) {
    let result = tree.within(&point.0, 3.0f64, &manhattan).unwrap();
    for x in &result {
        if !assigned.contains(x.1) && !cluster.contains(x.1) {
            cluster.insert(*x.1);
            assigned.insert(*x.1);
            add_recursive(cluster, assigned, points, tree, &points[*x.1]);
        }
    }
}

fn part1(path: &str) -> usize {
    let points = parse(path);
    let mut tree = KdTree::new(DIMS);
    for point in &points {
        tree.add(&point.0, point.1).unwrap();
    }

    let mut constellations = Vec::new();
    let mut assigned: HashSet<usize> = HashSet::new();
    for point in &points {
        if assigned.contains(&point.1) {
            continue;
        }

        let mut cluster: HashSet<usize> = HashSet::new();
        add_recursive(&mut cluster, &mut assigned, &points, &tree, point);
        constellations.push(cluster);
    }

    constellations.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex() {
    }
}

fn main() {
    println!("result: {}", part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt"));
}
