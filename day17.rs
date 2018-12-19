#![feature(test)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate gif;
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::borrow::Cow;

extern crate test;

#[macro_use]
extern crate scan_fmt;

#[derive(Default)]
struct Scan {
    rows: Vec<Vec<char>>,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    active_water: Vec<(usize, usize)>,
    reached: HashSet<(usize, usize)>,
    sand: HashSet<(usize, usize)>
}

impl Scan {
    pub fn init(&mut self, xmin: usize, xmax: usize, ymin: usize, ymax: usize) {
        self.xmin = xmin;
        self.xmax = xmax;
        self.ymin = ymin;
        self.ymax = ymax;

        for _y in 0..=ymax {
            self.rows.push(vec!['.'; xmax - xmin + 1]);
        }

        self.active_water = Vec::new();
        self.active_water.push((500 - xmin, 0));
        self.reached = HashSet::new();
        self.sand = HashSet::new();
    }

    pub fn print(&self) {
        let mut line = String::new();
        let mut y = 0;
        for row in &self.rows {
            line.clear();
            let mut x = 0;
            for ch in row {
                match *ch {
                    '~' | '#' | '|' => { line.push(*ch); },
                    _ => {
                       if self.reached.contains(&(x, y)) {
                           line.push('x');
                       } else {
                           line.push(*ch);
                       }
                    }
                }
                x += 1;
            }
            y += 1;
            println!("{}", line);
        }
        println!("rows: {}", y);
    }

    pub fn write_gif(&self, path: &str) {
        let color_map = &[0xCB, 0xCB, 0xCB, 0, 0, 0, 0x45, 0xE7, 0xFF, 0xF1, 0xE3, 0x46];
        let width = self.rows[0].len();
        let height = self.rows.len();
        let mut pixels = vec![0; width * height];
        let mut iter = 0usize;
        let mut y = 0;
        let mut blue = 0;
        for row in &self.rows {
            let mut x = 0;
            for cell in row {
                match cell {
                    '~' | '|' => {
                        if self.sand.contains(&(x, y)) {
                            pixels[iter] = 3;
                        } else {
                            pixels[iter] = 2;
                            blue += 1;
                        }
                    },
                    '#' => { pixels[iter] = 1; },
                    _ => {
                        if self.sand.contains(&(x, y)) {
                            pixels[iter] = 3;
                        } else {
                            pixels[iter] = 0;
                        }
                    }
                }
                iter += 1;
                x += 1;
            }
            y += 1;
        }

        println!("blue: {}", blue);

        let mut image = File::create(path).unwrap();
        let mut encoder = Encoder::new(&mut image, width as u16, height as u16, color_map).unwrap();
        encoder.set(Repeat::Infinite).unwrap();

        let mut frame = Frame::default();
        frame.width = width as u16;
        frame.height = height as u16;
        frame.buffer = Cow::Borrowed(&pixels);
        encoder.write_frame(&frame).unwrap();
    }

    pub fn fill_row_at(&mut self, coord: (usize, usize), forw: bool) -> (Vec<(usize, usize)>, bool, usize) {
        let mut result = Vec::new();
        let rows = &mut self.rows;
        let mut iter = coord.0;
        let mut revisit = false;
        loop {
            match rows[coord.1][iter] {
                '.' => {
                    let next = rows[coord.1+1][iter];
                    self.reached.insert((iter, coord.1));
                    if next == '.' {
                        rows[coord.1][iter] = '|';
                        result.push((iter, coord.1 + 1));
                        revisit = true;
                        break;
                    } else {
                        rows[coord.1][iter] = '~';
                    }
                },
                '|' => { revisit = true; break; }
                '~' => {},
                '#' => { break; },
                _ => { panic!("unkown contents at {:?} {}", (iter, coord.1), rows[coord.1][iter]) }
            }
            if forw { iter += 1 } else { iter -= 1; }
        }
        (result, revisit, iter)
    }

    pub fn bounded(&self, coord: &(usize, usize)) -> bool {
        let rows = &self.rows;
        let mut iter = coord.0;
        loop {
            match rows[coord.1][iter] {
                '#' => { break; },
                '|' => { return false; },
                _ => { iter += 1; }
            }
        }
        iter = coord.0;
        loop {
            match rows[coord.1][iter] {
                '#' => { break; },
                '|' => { return false; },
                _ => { iter -= 1; }
            }
        }
        true
    }

    pub fn step_fill(&mut self) -> Vec<(usize, usize)> {
        let mut next_active = Vec::new();
        let active = self.active_water.clone();
        for coord in &active {
            if coord.1 > self.ymax {
                continue;
            }

            let ch = self.rows[coord.1][coord.0];
            match ch {
                '.' => {
                    next_active.push((coord.0, coord.1 + 1));
                    if coord.1 >= self.ymin {
                        self.reached.insert(*coord);
                        self.sand.insert(*coord);
                    }
                },
                '#' | '~' | '|' => {
                    if ch == '~' {
                        if !self.bounded(&(coord.0, coord.1)) {
                            continue;
                        }
                    }
                    let mut y = coord.1;
                    loop {
                        y -= 1;
                        self.rows[y][coord.0] = '~';
                        self.reached.insert((coord.0, y));
                        let prev = next_active.len();
                        let result_left = &mut self.fill_row_at((coord.0-1, y), false);
                        next_active.append(&mut (result_left.0));
                        let result_right = &mut self.fill_row_at((coord.0+1, y), true);
                        next_active.append(&mut (result_right.0));
                        if prev != next_active.len() || result_left.1 || result_right.1 {
                            // adjust sand
                            for tmp in result_left.2..=result_right.2 {
                                self.sand.insert((tmp, y));
                                let mut iter = y + 1;
                                while iter < self.rows.len() {
                                    match self.rows[iter][tmp] {
                                        '#' | '.' => { break; },
                                        _ => { self.sand.remove(&(tmp, iter)); }
                                    }
                                    iter += 1;
                                }
                            }
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
        next_active
    }
}

fn solution(path: &str, print: bool) -> usize {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut lines = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (x, y0, y1) = scan_fmt!(&line, "x={}, y={}..{}", usize, usize, usize);
                if x.is_some() && y0.is_some() && y1.is_some() {
                    lines.push((x.unwrap(), x.unwrap(), y0.unwrap(), y1.unwrap()));
                } else {
                    let (y, x0, x1) = scan_fmt!(&line, "y={}, x={}..{}", usize, usize, usize);
                    assert!(y.is_some() && x0.is_some() && x1.is_some());
                    lines.push( (x0.unwrap(), x1.unwrap(), y.unwrap(), y.unwrap()));
                }
            }
            Err(e) => println!("err: {}", e)
        }
    }

    let xmin = lines.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0 - 1;
    let xmax = lines.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1 + 1;
    let ymin = lines.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2;
    let ymax = lines.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap().3;
    println!("xmin: {}, xmax: {}, ymin: {}, ymax: {}", xmin, xmax, ymin, ymax);

    let mut scan = Scan::default();
    scan.init(xmin, xmax, ymin, ymax);

    let trans: Vec<(usize, usize, usize, usize)> = lines.iter().map(|a| (a.0 - xmin, a.1 - xmin, a.2, a.3)).collect();
    for entry in &trans {
        if entry.0 == entry.1 {
            for row_idx in entry.2..=entry.3 {
                let row = scan.rows.get_mut(row_idx).unwrap();
                row[entry.0] = '#';
            }
        } else if entry.2 == entry.3 {
            let row = scan.rows.get_mut(entry.2).unwrap();
            for col_idx in entry.0..=entry.1 {
                row[col_idx] = '#';
            }
        } else {
            panic!("bogus entry {:?}", entry);
        }
    }

    if print { scan.print(); }

    let mut iter = 0;
    let print_iter = 2765;
    while scan.active_water.len() > 0 {
        if print { println!(""); }

        iter += 1;
        println!("iter: {}", iter);
        if iter == print_iter {
            scan.print();
            scan.write_gif(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.gif");
            break;
        }

        scan.active_water = scan.step_fill();
        if print { scan.print(); }
    }

    scan.reached.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
    }
}

fn main() {
    println!("result: {}", solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt", false));
}
