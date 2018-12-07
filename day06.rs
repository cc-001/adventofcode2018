use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

const INFINITE:i32 = -1;
const UNVISITED:i32 = -1;
const SHARED:i32 = -2;

struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn manhattan(a:&Coord, b:&Coord) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }

    pub fn normalize_extents(coords:&mut Vec<Coord>) -> (i32, i32, i32, i32) {
        let mut xmin = std::i32::MAX;
        let mut xmax = std::i32::MIN;
        let mut ymin = std::i32::MAX;
        let mut ymax = std::i32::MIN;

        for coord in coords.iter() {
            let cx = coord.x;
            let cy = coord.y;
            xmin = cmp::min(cx, xmin);
            xmax = cmp::max(cx, xmax);
            ymin = cmp::min(cy, ymin);
            ymax = cmp::max(cy, ymax);
        }

        if xmin > 0 {
            xmin -= 1;
        }
        if ymin > 0 {
            ymin -= 1;
        }
        xmax += 1;
        ymax += 1;

        for coord in coords.iter_mut() {
            coord.x -= xmin;
            coord.y -= ymin;
        }

        (xmin, xmax, ymin, ymax)
    }
}

struct Grid {
    cell_flag: Vec<i32>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32
}

impl Grid {
    pub fn len(xmin:i32, xmax:i32, ymin:i32, ymax:i32) -> i32 {
        (xmax - xmin) * (ymax - ymin)
    }

    pub fn dimx(&self) -> i32 {
        self.xmax - self.xmin
    }

    pub fn dimy(&self) -> i32 {
        self.ymax - self.ymin
    }
}

fn parse_coords(path:&str) -> Vec<Coord> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut coords = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (x, y) = scan_fmt!(&line, "{}, {}", i32, i32);
                coords.push(Coord {x: x.unwrap(), y: y.unwrap()});
            }
            Err(e) => println!("err: {}", e)
        }
    }
    return coords;
}

#[allow(dead_code)]
fn part1(path:&str) -> i32 {
    let mut coords = parse_coords(path);

    let extents = Coord::normalize_extents(&mut coords);
    let xmin = extents.0;
    let xmax = extents.1;
    let ymin = extents.2;
    let ymax = extents.3;

    // this is a dumb algorithm, order is higher than needed should do simultaneous flood from the origins
    let dim = Grid::len(xmin, xmax, ymin, ymax);
    let mut grid = Grid {cell_flag: vec![UNVISITED; dim as usize], xmin:xmin, xmax:xmax, ymin:ymin, ymax:ymax};
    let dimx = grid.dimx();
    let dimy = grid.dimy();
    let mut test_coord : Coord = Coord {x:0, y:0};
    let coords_len = coords.len();

    for i in 0..dim {
        test_coord.y = i / dimx;
        test_coord.x = i - (test_coord.y * dimx);

        let mut min_dist = std::i32::MAX;
        let mut shared = false;
        let mut coord_index = 0;

        for j in 0..coords_len {
            let dist = Coord::manhattan(&test_coord, &coords[j]);
            if dist < min_dist {
                min_dist = dist;
                shared = false;
                coord_index = j;
            } else if dist == min_dist {
                min_dist = dist;
                shared = true;
                coord_index = 0;
            }
        }

        if shared {
            grid.cell_flag[i as usize] = SHARED;
        } else {
            grid.cell_flag[i as usize] = coord_index as i32;
        }
    }

    let mut areas = vec![0; coords_len];
    for i in 0..dim {
        let tcy = i / dimx;
        let tcx = i - (tcy * dimx);

        assert_ne!(grid.cell_flag[i as usize], UNVISITED);
        let idx = grid.cell_flag[i as usize];
        if idx >= 0 {
            if tcx == 0 || tcx == dimx - 1 || tcy == 0 || tcy == dimy - 1 {
                areas[idx as usize] = INFINITE;
            } else {
                areas[idx as usize] += 1;
            }
        }
    }

    areas.retain(|&x| x >= 1);
    *areas.iter().max().unwrap()
}

fn part2(path:&str, total_dist:i32) -> i32 {
    let mut coords = parse_coords(path);

    let extents = Coord::normalize_extents(&mut coords);
    let xmin = extents.0;
    let xmax = extents.1;
    let ymin = extents.2;
    let ymax = extents.3;

    let dim = Grid::len(xmin, xmax, ymin, ymax);
    let mut grid = Grid {cell_flag: vec![UNVISITED; dim as usize], xmin:xmin, xmax:xmax, ymin:ymin, ymax:ymax};
    let dimx = grid.dimx();
    let mut test_coord : Coord = Coord {x:0, y:0};
    let coords_len = coords.len();

    for i in 0..dim {
        test_coord.y = i / dimx;
        test_coord.x = i - (test_coord.y * dimx);

        let mut sum = 0;
        for j in 0..coords_len {
            sum += Coord::manhattan(&test_coord, &coords[j]);
        }

        grid.cell_flag[i as usize] = sum;
    }

    grid.cell_flag.retain(|&x| x < total_dist);
    grid.cell_flag.len() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_example() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt"), 17);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt"), 5975);
    }

    #[test]
    fn test_part2_example() {
        use part2;
        assert_eq!(part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt", 32), 16);
    }

    #[test]
    fn test_part2_input() {
        use part2;
        assert_eq!(part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt", 10000), 38670);
    }
}

fn main() {
    //println!("result: {}", part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt"));
    //println!("result: {}", part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt"));
    println!("result: {}", part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt", 10000));
}
