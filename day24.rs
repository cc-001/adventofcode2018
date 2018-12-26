#![feature(test)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

const IMMUNE_SYSTEM: usize = 0;
const INFECTION: usize = 1;

#[derive(Clone, Debug)]
struct Attack {
    initiative: u32,
    dmg: u32,
    dmg_type: String
}

#[derive(Clone, Debug)]
struct Group {
    id: u32,
    army: usize,
    units: u32,
    hp: u32,
    attack: Attack,
    weak_to: Vec<String>,
    imm_to: Vec<String>
}

impl Group {
    pub fn effective_power(&self) -> u32 {
        self.units * self.attack.dmg
    }

    pub fn calc_damage_to(&self, other: &Group) -> u32 {
        if other.imm_to.contains(&self.attack.dmg_type) {
            0
        } else if other.weak_to.contains(&self.attack.dmg_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let before = self.units;
        let killed = damage / self.hp;
        self.units = self.units.saturating_sub(killed);
        (before - self.units)
    }

    pub fn sort_by_ep(groups: &mut Vec<Group>) {
        groups.sort_by(|a, b| {
            let ea = a.effective_power();
            let eb = b.effective_power();
            if ea == eb {
                a.attack.initiative.cmp(&b.attack.initiative).reverse()
            } else {
                ea.cmp(&eb).reverse()
            }
        });
    }

    pub fn sort_by_initiative(groups: &mut Vec<Group>) {
        groups.sort_by(|a, b| a.attack.initiative.cmp(&b.attack.initiative).reverse());
    }
}

#[derive(Clone, Debug)]
struct Army {
    id: usize,
    name: String,
    groups: Vec<Group>
}

impl Army {
    pub fn print(&self) {
        println!("{}:", self.name);
        if self.groups.len() > 0 {
            for group in &self.groups {
                println!("Group {} contains {} units", group.id, group.units);
            }
        } else {
            println!("No groups remain.");
        }
    }

    pub fn unit_count(&self) -> u32 {
        self.groups.iter().fold(0u32, |sum, x| sum + x.units)
    }

    pub fn target_selection(&self, enemy_army: &Army, attacks: &mut HashMap<(usize, u32), (u32, u32)>, verbose: bool) {
        let mut assigned = HashSet::new();
        for group in &self.groups {
            let mut damages = Vec::with_capacity(enemy_army.groups.len());
            for other in &enemy_army.groups {
                if assigned.contains(&other.id) {
                    continue;
                }
                let dmg = group.calc_damage_to(other);
                if other.units > 0 && dmg > 0 {
                    if verbose {
                        println!("{} group {} would deal defending group {} {} damage", self.name, group.id, other.id, dmg);
                    }
                    damages.push((dmg, other.effective_power(), other.attack.initiative, other.id, other.hp));
                }
            }

            damages.sort_by(|a, b| {
                if a.0 == b.0 {
                    if a.1 == b.1 {
                        a.2.cmp(&b.2).reverse()
                    } else {
                        a.1.cmp(&b.1).reverse()
                    }
                } else {
                    a.0.cmp(&b.0).reverse()
                }
            });
            if verbose {
                println!("damages: {:?}", damages);
            }

            if damages.first().is_some() {
                let attack = damages.first().unwrap();
                attacks.insert((self.id, group.id), (attack.3, attack.0));
                assigned.insert(attack.3);
                if verbose {
                    println!("selected: {}", attack.3);
                }
            }
            if verbose {
                println!("assigned: {:?}", assigned);
            }
        }
    }

    pub fn enemy(id: usize) -> usize {
        if id == IMMUNE_SYSTEM { INFECTION } else { IMMUNE_SYSTEM }
    }

    pub fn damage_from(&mut self, group_id: u32, from: &Group) -> u32 {
        for group in self.groups.iter_mut() {
            if group_id == group.id {
                let damage = from.calc_damage_to(group);
                return group.take_damage(damage);
            }
        }
        0
    }

    pub fn get_group(&self, group_id: u32) -> Option<Group> {
        for group in &self.groups {
            if group_id == group.id {
                return Some(group.clone());
            }
        }
        None
    }

    pub fn get_group_units(&self, group_id: u32) -> Option<u32> {
        for group in &self.groups {
            if group_id == group.id {
                return Some(group.units);
            }
        }
        None
    }

    pub fn cleanup(&mut self) {
        self.groups.retain(|x| x.units > 0);
    }
}

fn fight(armies: &mut Vec<Army>, round: u32, verbose: bool) -> bool {
    let mut attacks = HashMap::new();

    Group::sort_by_ep(&mut armies[IMMUNE_SYSTEM].groups);
    armies[IMMUNE_SYSTEM].target_selection(&armies[INFECTION], &mut attacks, false);

    Group::sort_by_ep(&mut armies[INFECTION].groups);
    armies[INFECTION].target_selection(&armies[IMMUNE_SYSTEM], &mut attacks, false);

    let mut all_groups = Vec::new();
    for army in armies.iter() {
        all_groups.append(&mut army.groups.clone());
    }
    Group::sort_by_initiative(&mut all_groups);

    if verbose {
        println!("");
    }

    let mut total_killed = 0;
    for group in &all_groups {
        let tmp = attacks.get(&(group.army, group.id));
        if tmp.is_some() {
            let attack = tmp.unwrap();
            let attacking_group = armies[group.army].get_group(group.id).unwrap();
            if attacking_group.units <= 0 {
                continue;
            }
            let killed = armies[Army::enemy(group.army)].damage_from(attack.0, &attacking_group);
            total_killed += killed;
            if verbose {
                println!("Round: {}", round);
                println!("{} group {} attacks defending {} group {}, killing {} of {}", armies[group.army].name, group.id, armies[Army::enemy(group.army)].name, attack.0,
                         killed, armies[Army::enemy(group.army)].get_group_units(attack.0).unwrap() + killed);
            }
        }
    }

    for army in armies {
        army.cleanup();
    }

    total_killed == 0
}

fn parse(path: &str) -> Vec<Army> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let re0 = Regex::new(r"(\d{1,}) units each with (\d{1,}) hit points").unwrap();
    let re1 = Regex::new(r"with an attack that does (\d{1,}) ([^\s]+) damage at initiative (\d{1,})").unwrap();
    let re2 = Regex::new(r"weak to (\w+)(, \w+)*").unwrap();
    let re3 = Regex::new(r"immune to (\w+)?(, \w+)?(, \w+)*").unwrap();

    let mut result = Vec::new();
    result.push(Army{ id: IMMUNE_SYSTEM, name: String::from("Immune System"), groups: Vec::new() });
    result.push(Army{ id: INFECTION, name: String::from("Infection"), groups: Vec::new() });

    let mut army = IMMUNE_SYSTEM;
    let mut group_id = 1u32;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                match line.as_ref() {
                    "Immune System:" => { army = IMMUNE_SYSTEM; group_id = 1; },
                    "Infection:" => { army = INFECTION; group_id = 1; },
                    _ => {
                        if line.is_empty() { continue; }

                        let mut group = Group {id: group_id, army: army, units: 0, hp: 0, attack: Attack {initiative:0, dmg:0, dmg_type:String::from("")}, weak_to: Vec::new(), imm_to: Vec::new()};
                        for cap in re0.captures_iter(&line) {
                            group.units = cap[1].parse::<u32>().unwrap();
                            group.hp = cap[2].parse::<u32>().unwrap();
                        }
                        for cap in re1.captures_iter(&line) {
                            group.attack.dmg = cap[1].parse::<u32>().unwrap();
                            group.attack.dmg_type = cap[2].to_string();
                            group.attack.initiative = cap[3].parse::<u32>().unwrap();
                        }
                        for cap in re2.captures_iter(&line) {
                            for x in 1..cap.len() {
                                if cap.get(x).is_some() {
                                    if cap[x].chars().next().unwrap() == ',' {
                                        group.weak_to.push(cap[x][2..].to_string());
                                    } else {
                                        group.weak_to.push(cap[x].to_string());
                                    }
                                }
                            }
                        }
                        for cap in re3.captures_iter(&line) {
                            for x in 1..cap.len() {
                                if cap.get(x).is_some() {
                                    if cap[x].chars().next().unwrap() == ',' {
                                        group.imm_to.push(cap[x][2..].to_string());
                                    } else {
                                        group.imm_to.push(cap[x].to_string());
                                    }
                                }
                            }
                        }
                        println!("{} units each with {} hit points weak_to:{:?} imm_to:{:?} with an attack that does {} {} damage at initiative {}",
                                 group.units, group.hp, group.weak_to, group.imm_to, group.attack.dmg, group.attack.dmg_type, group.attack.initiative);
                        result[army].groups.push(group);
                        group_id += 1;
                    }
                }
            },
            Err(e) => println!("err: {}", e)
        }
    }
    result
}

fn part1(path: &str, verbose: bool) -> u32 {
    let mut armies = parse(path);
    let mut round = 1;
    loop {
        if verbose {
            println!("");
            for army in armies.iter_mut() {
                army.groups.sort_by_key(|x| x.id);
                army.print();
            }
            println!("");
        }

        let immune_count = armies[IMMUNE_SYSTEM].unit_count();
        let infection_count = armies[INFECTION].unit_count();
        if immune_count == 0 && infection_count == 0 {
            // tie
            return 0;
        } else if immune_count == 0 {
            return infection_count;
        } else if infection_count == 0 {
            return immune_count;
        }

        if fight(&mut armies, round, false) {
            // stalemate
            return 0;
        }

        round += 1;
    }
}

fn boosted(armies: &mut Vec<Army>, boost: u32) -> Option<u32> {
    for group in armies[IMMUNE_SYSTEM].groups.iter_mut() {
        group.attack.dmg += boost;
    }

    loop {
        let immune_count = armies[IMMUNE_SYSTEM].unit_count();
        let infection_count = armies[INFECTION].unit_count();
        if immune_count == 0 && infection_count == 0 {
            // tie
            return None;
        } else if immune_count == 0 {
            return None;
        } else if infection_count == 0 {
            return Some(immune_count);
        }

        if fight(armies, 0, false) {
            // stalemate
            return None;
        }
    }
}

fn part2(path: &str) -> u32 {
    let mut armies = parse(path);
    let mut boost = 1u32;
    loop {
        println!("boost: {}", boost);
        let result = boosted(&mut armies.clone(), boost);
        if result.is_some() {
            return result.unwrap();
        }
        boost += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt", false), 5216);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt", false), 10538);
    }
}

fn main() {
    println!("result: {}", part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt"));
}
