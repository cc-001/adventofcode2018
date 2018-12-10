#![feature(test)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use gif::{Frame, Encoder, Repeat, SetParameter};
use std::borrow::Cow;

#[macro_use]
extern crate scan_fmt;

extern crate gif;
extern crate test;

#[derive(Debug)]
struct Point {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32
}

fn parse_points(path:&str) -> Vec<(Point)> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut result = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (a, b, c, d) = scan_fmt!(&line, "position=<{},{}> velocity=<{},{}>", i32, i32, i32, i32);
                result.push(Point {pos_x: a.unwrap(), pos_y: b.unwrap(), vel_x: c.unwrap(), vel_y: d.unwrap()});
            }
            Err(e) => println!("err: {}", e)
        }
    }
    return result;
}

fn get_extents(points: &Vec<Point>) -> (usize, usize, i32, i32) {
    let mut xmin = std::i32::MAX;
    let mut ymin = std::i32::MAX;
    let mut xmax = std::i32::MIN;
    let mut ymax = std::i32::MIN;
    for point in points {
        if point.pos_x < xmin {
            xmin = point.pos_x;
        }
        if point.pos_y < ymin {
            ymin = point.pos_y;
        }
        if point.pos_x > xmax {
            xmax = point.pos_x;
        }
        if point.pos_y > ymax {
            ymax = point.pos_y;
        }
    }
    let width =(xmax - xmin) as usize;
    let height = (ymax - ymin) as usize;
    (width, height, xmin.abs(), ymin.abs())
}

fn points_at_time(points: &Vec<Point>, time: u32) -> Vec<Point> {
    let mut result = Vec::with_capacity(points.len());
    let t = time as i32;
    for x in points {
        let pos_x = x.pos_x + x.vel_x * t;
        let pos_y = x.pos_y + x.vel_y * t;
        result.push(Point {pos_x: pos_x, pos_y: pos_y, vel_x:0, vel_y:0});
    }
    result
}

fn extents_at_time(points: &Vec<Point>, time: u32) -> (usize, usize, i32, i32) {
    let new_points = points_at_time(points, time);
    return get_extents(&new_points);
}

fn find_start_time_dims(points: &Vec<Point>) -> Option<(usize, u32)> {
    let mut start = 0;
    let mut step = 512;

    loop {
        let result = find_start_time_dims_step(points, start, step);
        if result.is_some() {
            let std = result.unwrap();
            start = u32::max(0, std.1 - step);
            step /= 2;
            if step == 0 {
                return result;
            }
        } else {
            break;
        }
    }
    None
}

fn find_start_time_dims_step(points: &Vec<Point>, start: u32, step: u32) -> Option<(usize, u32)> {
    let mut time = start;
    let mut last_dim = std::usize::MAX;
    let mut last_time = 0u32;

    for _x in 0..128 {
        let dims = extents_at_time(points, time);
        let max = usize::max(dims.0, dims.1);
        if max < last_dim {
            last_dim = max;
            last_time = time;
        } else {
            return Some((last_dim, last_time));
        }
        time += step;
    }
    None
}

fn generate_pixels(points: &Vec<Point>, max_dims: usize, time: u32) -> Option<Vec<u8>> {
    let points_t = points_at_time(points, time);
    let dims = get_extents(&points_t);
    println!("dims: {:?}", dims);
    if dims.0 > max_dims || dims.1 > max_dims {
        None
    } else {
        let num_pixels = max_dims * max_dims;
        let mut pixels = vec![0u8; num_pixels];
        //let shift = max_dims as i32 / 2;
        for x in points_t {
            println!("pos_x = {} pos_y = {}", x.pos_x, x.pos_y);
            let pos_x = x.pos_x;
            let pos_y = x.pos_y;
            if pos_y >= 0 && pos_x >= 0 {
                let pixel_index = pos_y as usize * max_dims + pos_x as usize;
                if pixel_index < num_pixels {
                    pixels[pixel_index] = 1u8;
                }
            }
        }
        Some(pixels)
    }
}

fn part1(path:&str, output:&str, frame_count: u32, frame_duration: u32) {
    let points = parse_points(path);

    let tr = find_start_time_dims(&points);
    if tr.is_none() {
        panic!("failed to find start time");
    }

    let std = tr.unwrap();
    let mut start_time = std.1;
    println!("start_time: {}", start_time);
    if frame_count * frame_duration > start_time {
        start_time = 0;
    } else {
        start_time -= frame_count * frame_duration;
    }

    let dims = extents_at_time(&points, start_time);
    let max_dims = usize::max(dims.0, dims.1) * 4;
    println!("max_dims: {}", max_dims);

    let width = max_dims as u16;
    let height = max_dims as u16;
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut image = File::create(output).unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    let mut pixels: Vec<u8>;
    for x in 0..=frame_count {
        let result = generate_pixels(&points, max_dims as usize, x * frame_duration + start_time);
        if result.is_some() {
            println!("fr: {}", x * frame_duration + start_time);
            let mut frame = Frame::default();
            frame.delay = (x * 10) as u16;//(x * 100) as u16;
            frame.width = width;
            frame.height = height;
            pixels = result.unwrap();
            frame.buffer = Cow::Borrowed(&*pixels);
            encoder.write_frame(&frame).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex_0() {
    }
}

fn main() {
    part1(r"C:\Users\lgascoigne\IdeaProjects\advent\input.txt", r"C:\Users\lgascoigne\IdeaProjects\advent\part1_input.gif", 10, 1);
}
