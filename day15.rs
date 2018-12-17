#![feature(test)]

use std::collections::HashMap;
use std::collections::VecDeque;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate test;

#[derive(Copy, Clone, Default)]
struct Npc {
    class: char,
    x: u32,
    y: u32,
    ap: u32,
    hp: u32
}

impl Npc {
    pub fn get_hostile(&self) -> char {
        if self.class == 'E' { 'G' } else { 'E' }
    }
}

#[derive(Default)]
struct Square {
    x: u32,
    y: u32,
    contents: char
}

#[derive(Default)]
struct Map {
    squares: Vec<Vec<Square>>,
    npcs: HashMap<(u32, u32), Npc>
}

impl Map {
    #[allow(dead_code)]
    pub fn print(&self) {
        let mut string = String::new();
        for line in &self.squares {
            let mut units = String::new();
            for cell in line {
                match self.npcs.get(&(cell.x, cell.y)) {
                    Some(npc) => {
                        string.push(npc.class);
                        units.push_str(format!("{}({}), ", npc.class, npc.hp).as_str());
                    },
                    None => string.push(cell.contents)
                }
            }
            println!("{}    {}", string, units);
            string.clear();
        }
    }

    #[allow(dead_code)]
    pub fn try_add_defender(&self, npc: &Npc, coord: &(u32, u32)) -> bool {
        let tmp = self.npcs.get(&coord);
        if tmp.is_some() {
            let x = tmp.unwrap();
            if x.class != npc.class {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn get_defenders(&self, npc: &Npc) -> Vec<Npc> {
        let mut result = Vec::new();
        let mut coord = (npc.x+1, npc.y);
        if self.try_add_defender(npc, &coord) { result.push(self.npcs.get(&coord).unwrap().clone()); }
        coord = (npc.x-1, npc.y);
        if self.try_add_defender(npc, &coord) { result.push(self.npcs.get(&coord).unwrap().clone()); }
        coord = (npc.x, npc.y+1);
        if self.try_add_defender(npc, &coord) { result.push(self.npcs.get(&coord).unwrap().clone()); }
        coord = (npc.x, npc.y-1);
        if self.try_add_defender(npc, &coord) { result.push(self.npcs.get(&coord).unwrap().clone()); }
        result
    }

    #[allow(dead_code)]
    pub fn process_attack(&mut self, npc: &Npc) -> Option<(u32, u32)> {
        let mut defenders = self.get_defenders(npc);
        defenders.sort_by(|a, b| {
           if a.hp != b.hp {
               a.hp.cmp(&b.hp)
           } else {
               if a.y == b.y {
                   a.x.cmp(&b.x)
               } else {
                   a.y.cmp(&b.y)
               }
           }
        });

        if defenders.len() > 0 {
            let defender = &defenders[0];
            if defender.hp > npc.ap {
                self.npcs.get_mut(&(defender.x, defender.y)).unwrap().hp -= npc.ap;
            } else {
                return Some((defender.x, defender.y));
            }
        }
        None
    }

    pub fn in_range_of_enemy(&self, npc: &Npc) -> bool {
        self.get_defenders(npc).len() > 0
    }

    pub fn is_vacant(&self, coord: &(u32, u32)) -> bool {
        let row = &self.squares[coord.1 as usize];
        let square = &row[coord.0 as usize];
        match square.contents {
            '.' => !self.npcs.contains_key(coord),
            _ => false
        }
    }

    pub fn get_vacant(&self, coord: &(u32, u32)) -> Vec<(u32, u32)> {
        let mut result = Vec::new();
        let mut tmp = (coord.0+1, coord.1);
        if self.is_vacant(&tmp) { result.push(tmp); }
        tmp = (coord.0-1, coord.1);
        if self.is_vacant(&tmp) { result.push(tmp); }
        tmp = (coord.0, coord.1+1);
        if self.is_vacant(&tmp) { result.push(tmp); }
        tmp = (coord.0, coord.1-1);
        if self.is_vacant(&tmp) { result.push(tmp); }
        result
    }

    pub fn find_steps(&self, from: &(u32, u32), to: &(u32, u32)) -> Option<Vec<((u32, u32), u32, (u32, u32))>> {
        let vacant = self.get_vacant(from);
        if vacant.len() == 0 {
            return None;
        }

        // distance, no early terminate, no biasing i.e. slow and lame
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back(*to);
        visited.insert(*to, 0);
        while queue.len() > 0 {
            let square = queue.pop_front().unwrap();
            let neighbors = self.get_vacant(&square);
            for neighbor in &neighbors {
                if !visited.contains_key(neighbor) {
                    let dist = *visited.get(&square).unwrap() + 1;
                    visited.insert(*neighbor, dist);
                    queue.push_back(*neighbor);
                }
            }
        }

        let mut result = Vec::new();
        for square in &vacant {
            let visit = visited.get(square);
            if visit.is_some() {
                result.push((*square, *visit.unwrap(), *to));
            }
        }
        Some(result)
    }

    pub fn get_target_squares(&self, class: char) -> Vec<(u32, u32)> {
        let mut result = Vec::new();
        for npc in self.npcs.values() {
            if npc.class == class {
                let coord = (npc.x, npc.y);
                let mut squares = self.get_vacant(&coord);
                if squares.len() > 0 {
                    result.append(&mut squares);
                }
            }
        }
        result
    }

    pub fn process_move(&mut self, npc: &Npc) -> Option<(u32, u32)> {
        let mut targets = self.get_target_squares(npc.get_hostile());
        if targets.len() == 0 {
            return None;
        }

        let mut steps = Vec::new();
        for target in &targets {
            let tmp = self.find_steps(&(npc.x, npc.y), target);
            if tmp.is_some() {
                steps.append(&mut tmp.unwrap());
            }
        }

        if steps.len() > 0 {
            steps.sort_by(|a, b| {
                // the guy who made this problem ruined my xmas
                let da = a.1;
                let db = b.1;
                if da != db {
                    da.cmp(&db)
                } else {
                    if a.2 == b.2 {
                        if (a.0).1 == (b.0).1 {
                            (a.0).0.cmp(&(b.0).0)
                        } else {
                            (a.0).1.cmp(&(b.0).1)
                        }
                    } else {
                        if (a.2).1 == (b.2).1 {
                            (a.2).0.cmp(&(b.2).0)
                        } else {
                            (a.2).1.cmp(&(b.2).1)
                        }
                    }
                }
            });

            // do best step
            let step = steps[0];
            self.npcs.remove(&(npc.x, npc.y));
            let mut moved = npc.clone();
            moved.x = (step.0).0;
            moved.y = (step.0).1;
            self.npcs.insert((moved.x, moved.y), moved);
            return Some((moved.x, moved.y));
        }
        None
    }

    pub fn is_done(&self) -> bool {
        let mut found_goblin = false;
        let mut found_elf = false;
        for x in self.npcs.values() {
            found_goblin |= x.class == 'G';
            found_elf |= x.class == 'E';
            if found_goblin && found_elf {
                return false;
            }
        }
        true
    }

    pub fn process_turn(&mut self, early: bool) -> (bool, bool) {
        let mut npc_coords: Vec<(u32, u32)> = Vec::new();
        for npc in self.npcs.values() {
            npc_coords.push((npc.x, npc.y));
        }

        npc_coords.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut interrupted = false;
        for coord in &npc_coords {
            if self.npcs.get(coord).is_none() {
                continue;
            }

            let mut npc = self.npcs.get(coord).unwrap().clone();
            if !self.in_range_of_enemy(&npc) {
                let new_coord = self.process_move(&npc);
                if new_coord.is_some() {
                    npc = self.npcs.get(&new_coord.unwrap()).unwrap().clone();
                } else {
                    continue;
                }
            }

            let remove = self.process_attack(&npc);
            if remove.is_some() {
                let tmp = remove.unwrap();
                if early && self.npcs.get(&tmp).unwrap().class == 'E' {
                    return (true, true);
                }
                self.npcs.remove(&tmp);
            }

            if self.is_done() && npc_coords.last().unwrap() != coord {
                interrupted = true;
                break;
            }
        }
        (interrupted, false)
    }

    fn hp_remaining(&self) -> u32 {
        self.npcs.values().fold(0,|sum, x| sum + x.hp)
    }

    fn num_class(&self, class: char) -> u32 {
        self.npcs.values().fold( 0, |sum, x| if x.class == class { sum + 1 } else { sum } )
    }
}

fn parse(path: &str, ap: u32) -> Map {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut map = Map::default();
    let mut y = 0u32;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut x = 0u32;
                let mut tmp = Vec::new();
                for ch in line.chars() {
                    match ch {
                        'E' => {
                            map.npcs.insert((x, y), Npc { class: ch, x: x, y: y, ap: ap, hp: 200 });
                            tmp.push(Square {x: x, y: y, contents: '.'})
                        },
                        'G' => {
                            map.npcs.insert((x, y), Npc { class: ch, x: x, y: y, ap: 3, hp: 200 });
                            tmp.push(Square {x: x, y: y, contents: '.'})
                        }
                        _ => { tmp.push(Square {x: x, y: y, contents: ch}) }
                    }
                    x += 1;
                }
                map.squares.push(tmp);
                y += 1;
            }
            Err(e) => println!("err: {}", e)
        }
    }
    map
}

#[allow(dead_code)]
fn part1(path: &str, print: bool) -> u32 {
    let mut map = parse(path, 3);
    let mut round = 0;

    if print {
        println!("Initially:");
        map.print();
        println!("");
    }

    loop {
        round += 1;
        println!("simulating {}", round);
        let done = map.is_done();
        if done || map.process_turn(false).0 {
            if print {
                println!("Round {} incomplete:", round);
                map.print();
                println!("");
            }
            println!("round: {} hp: {} done: {}", round - 1, map.hp_remaining(), done);
            return (round - 1) * map.hp_remaining();
        }

        if print {
            println!("After {} rounds:", round);
            map.print();
            println!("");
        }
    }
}

#[allow(dead_code)]
fn part2(path: &str) -> u32 {
    // lame but tired of this problem
    // use to figure out ap then use ap in part 1
    for x in 4..50u32 {
        println!("ap: {}", x);
        let mut map = parse(path, x);
        let mut iter = 1;
        loop {
            let done = map.is_done();
            let result = map.process_turn(true);
            if done || result.0 {
                if result.1 {
                    break;
                } else {
                    return x;
                }
            }
            if iter % 10 == 0 {
                println!("iter: {} hp: {} goblins: {} elves: {}", iter, map.hp_remaining(), map.num_class('G'), map.num_class('E'));
            }
            iter += 1;
        }
    }
    panic!("not found");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt", false), 27730);
    }

    #[test]
    fn test_part1_ex1() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test2.txt", false), 36334);
    }

    #[test]
    fn test_part1_ex2() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test3.txt", false), 39514);
    }

    #[test]
    fn test_part1_ex3() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test4.txt", false), 27755);
    }

    #[test]
    fn test_part1_ex4() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test5.txt", false), 28944);
    }

    #[test]
    fn test_part1_ex5() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test6.txt", false), 18740);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt", false), 250594);
    }
}

fn main() {
}
