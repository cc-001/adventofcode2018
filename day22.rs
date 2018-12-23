#![feature(test)]

extern crate pathfinding;
use pathfinding::prelude::{absdiff, astar, Matrix};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tool {
    Neither,
    Climbing,
    Torch
}

const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;
const SWITCH_TIME: u32 = 7;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Region {
    coord: (usize, usize),
    tool: Tool
}

impl Region {
    fn distance(&self, other: &Region) -> u32{
        (absdiff(self.coord.0 as u32, other.coord.0 as u32) + absdiff(self.coord.1 as u32, other.coord.1 as u32))
    }

    fn add_at(&self, coord: (usize, usize), curr_region: usize, regions: &Vec<Vec<usize>>) -> Vec<(Region, u32)> {
        let mut result = Vec::new();
        let next_region = regions[coord.1][coord.0];
        match curr_region {
            ROCKY => {
                assert_ne!(self.tool, Tool::Neither);
                match next_region {
                    ROCKY => {
                        if self.tool == Tool::Torch {
                            result.push((Region {coord: coord, tool: Tool::Torch}, 1));
                            result.push((Region {coord: self.coord, tool: Tool::Climbing}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Climbing);
                            result.push((Region {coord: coord, tool: Tool::Climbing}, 1));
                            result.push((Region {coord: self.coord, tool: Tool::Torch}, SWITCH_TIME));
                        }
                    },
                    WET => {
                        if self.tool == Tool::Torch {
                            result.push((Region {coord: self.coord, tool: Tool::Climbing}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Climbing);
                            result.push((Region {coord: coord, tool: Tool::Climbing}, 1));
                        }
                    },
                    NARROW => {
                        if self.tool == Tool::Climbing {
                            result.push((Region {coord: self.coord, tool: Tool::Torch}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Torch);
                            result.push((Region {coord: coord, tool: Tool::Torch}, 1));
                        }
                    },
                    _ => panic!("")
                }
            },
            WET => {
                assert_ne!(self.tool, Tool::Torch);
                match next_region {
                    ROCKY => {
                        if self.tool == Tool::Neither {
                            result.push((Region {coord: self.coord, tool: Tool::Climbing}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Climbing);
                            result.push((Region {coord: coord, tool: Tool::Climbing}, 1));
                        }
                    },
                    WET => {
                        if self.tool == Tool::Neither {
                            result.push((Region {coord: coord, tool: Tool::Neither}, 1));
                            result.push((Region {coord: self.coord, tool: Tool::Climbing}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Climbing);
                            result.push((Region {coord: coord, tool: Tool::Climbing}, 1));
                            result.push((Region {coord: self.coord, tool: Tool::Neither}, SWITCH_TIME));
                        }
                    },
                    NARROW => {
                        if self.tool == Tool::Climbing {
                            result.push((Region {coord: self.coord, tool: Tool::Neither}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Neither);
                            result.push((Region {coord: coord, tool: Tool::Neither}, 1));
                        }
                    },
                    _ => panic!("")
                }
            },
            NARROW => {
                assert_ne!(self.tool, Tool::Climbing);
                match next_region {
                    ROCKY => {
                        if self.tool == Tool::Torch {
                            result.push((Region {coord: coord, tool: Tool::Torch}, 1));
                        } else {
                            assert_eq!(self.tool, Tool::Neither);
                            result.push((Region {coord: self.coord, tool: Tool::Torch}, SWITCH_TIME));
                        }
                    },
                    WET => {
                        if self.tool == Tool::Torch {
                            result.push((Region {coord: self.coord, tool: Tool::Neither}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Neither);
                            result.push((Region {coord: coord, tool: Tool::Neither}, 1));
                        }
                    },
                    NARROW => {
                        if self.tool == Tool::Torch {
                            result.push((Region {coord: coord, tool: Tool::Torch}, 1));
                            result.push((Region {coord: self.coord, tool: Tool::Neither}, SWITCH_TIME));
                        } else {
                            assert_eq!(self.tool, Tool::Neither);
                            result.push((Region {coord: coord, tool: Tool::Neither}, 1));
                            result.push((Region {coord: self.coord, tool: Tool::Torch}, SWITCH_TIME));
                        }
                    },
                    _ => panic!("")
                }
            },
            _ => panic!("unknown region at {:?} {}", self.coord, curr_region)
        }
        result
    }

    pub fn successors(&self, regions: &Vec<Vec<usize>>) -> Vec<(Region, u32)> {
        let mut result= Vec::new();
        let curr_region = regions[self.coord.1][self.coord.0];
        if self.coord.0 > 0 { result.append(&mut self.add_at((self.coord.0-1, self.coord.1), curr_region, regions)); }
        if self.coord.1 > 0 { result.append(&mut self.add_at((self.coord.0, self.coord.1-1), curr_region, regions)); }
        result.append(&mut self.add_at((self.coord.0+1, self.coord.1), curr_region, regions));
        result.append(&mut self.add_at((self.coord.0, self.coord.1+1), curr_region, regions));
        result
    }
}

fn create(depth: usize, target: (usize, usize), dim: (usize, usize)) -> Vec<Vec<usize>> {
    let mut erosion_levels = Vec::with_capacity(dim.1 as usize);
    let mut regions = Vec::with_capacity(dim.1 as usize);

    let mut er_row = Vec::with_capacity(dim.0 as usize);
    let mut region_row = Vec::with_capacity(dim.0 as usize);

    // y = 0
    for x in 0..=dim.0 {
        let gi = x * 16807;
        let er = (gi + depth) % 20183;
        er_row.push(er);
        region_row.push(er % 3);
    }
    erosion_levels.push(er_row);
    regions.push(region_row);

    // x = 0
    for y in 1..=dim.1 {
        let gi = y * 48271;
        let er = (gi + depth) % 20183;
        erosion_levels.push(vec![er; 1]);
        regions.push(vec![er % 3; 1]);
    }

    // remaining
    for x in 1..=dim.0 {
        for y in 1..=dim.1 {
            let gi = if x == target.0 && y == target.1 { 0 } else { erosion_levels[y][x-1] * erosion_levels[y-1][x] };
            let er = (gi + depth) % 20183;
            erosion_levels[y].push(er);
            regions[y].push(er % 3);
        }
    }

    assert_eq!(ROCKY, regions[target.1][target.0]);
    regions
}

fn part1(depth: usize, target: (usize, usize)) -> usize {
    let regions = create(depth, target, target);
    regions.iter().fold(0usize, |sum, x| sum + x.iter().fold(0usize, |sum, y| sum + y))
}

fn part2(depth: usize, target: (usize, usize), expand: usize) -> u32 {
    let regions = create(depth, target, (target.0 + expand, target.1 + expand));
    let initial = Region { coord: (0, 0), tool: Tool::Torch };
    let target = Region { coord: target, tool: Tool::Torch };
    let result = astar(&initial, |p| p.successors(&regions), |p| p.distance(&target),
        |p| *p == target);
    result.unwrap().1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex() {
        use part1;
        assert_eq!(part1(510, (10, 10)), 114);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(11541, (14, 778)), 11575);
    }
}

fn main() {
    println!("result: {}", part2(11541, (14, 778), 1024));
}
