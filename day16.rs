#![feature(test)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

extern crate test;

type Op = fn(u32, u32, &mut u32);

enum Operands {
    RegReg,
    RegImm,
    ImmReg,
}

struct Instruction {
    mnemonic: String,
    opcode: u32,
    op: Op,
    ops: Operands
}

fn add(a: u32, b: u32, c: &mut u32) {
    *c = a + b;
}

fn mul(a: u32, b: u32, c: &mut u32) {
    *c = a * b;
}

fn and(a: u32, b: u32, c: &mut u32) {
    *c = a & b;
}

fn or(a: u32, b: u32, c: &mut u32) {
    *c = a | b;
}

fn set(a: u32, _b: u32, c: &mut u32) {
    *c = a;
}

fn gt(a: u32, b: u32, c: &mut u32) {
    if a > b { *c = 1; } else { *c = 0 }
}

fn eq(a: u32, b: u32, c: &mut u32) {
    if a == b { *c = 1; } else { *c = 0 }
}

struct Device {
    registers: [u32; 4],
    instructions: Vec<Instruction>
}

impl Device {
    pub fn new() -> Device {
        Device {
            registers: [0u32; 4],
            instructions: vec![
                Instruction { mnemonic: String::from("addr"), opcode: 13, op: add, ops: Operands::RegReg },
                Instruction { mnemonic: String::from("addi"), opcode: 10, op: add, ops: Operands::RegImm },
                Instruction { mnemonic: String::from("mulr"), opcode: 14, op: mul, ops: Operands::RegReg },
                Instruction { mnemonic: String::from("muli"), opcode: 5, op: mul, ops: Operands::RegImm },
                Instruction { mnemonic: String::from("banr"), opcode: 0, op: and, ops: Operands::RegReg },
                Instruction { mnemonic: String::from("bani"), opcode: 6, op: and, ops: Operands::RegImm },
                Instruction { mnemonic: String::from("borr"), opcode: 7, op: or, ops: Operands::RegReg },
                Instruction { mnemonic: String::from("bori"), opcode: 4, op: or, ops: Operands::RegImm },
                Instruction { mnemonic: String::from("setr"), opcode: 2, op: set, ops: Operands::RegReg },
                Instruction { mnemonic: String::from("seti"), opcode: 15, op: set, ops: Operands::ImmReg },
                Instruction { mnemonic: String::from("gtir"), opcode: 8, op: gt, ops: Operands::ImmReg },
                Instruction { mnemonic: String::from("gtri"), opcode: 11, op: gt, ops: Operands::RegImm },
                Instruction { mnemonic: String::from("gtrr"), opcode: 9, op: gt, ops: Operands::RegReg },
                Instruction { mnemonic: String::from("eqir"), opcode: 3, op: eq, ops: Operands::ImmReg },
                Instruction { mnemonic: String::from("eqri"), opcode: 12, op: eq, ops: Operands::RegImm },
                Instruction { mnemonic: String::from("eqrr"), opcode: 1, op: eq, ops: Operands::RegReg },
            ]
        }
    }

    pub fn execute_part1(&mut self, regs: &[u32; 4], inst: &[u32; 4], result: &[u32; 4], stats: &mut Vec<Vec<u32>>) -> u32 {
        let mut count = 0u32;
        let mut index = 0;
        for instr in &self.instructions {
            self.registers.copy_from_slice(regs);
            match instr.ops {
                Operands::RegReg => (instr.op)(self.registers[inst[1] as usize], self.registers[inst[2] as usize], &mut self.registers[inst[3] as usize]),
                Operands::RegImm => (instr.op)(self.registers[inst[1] as usize], inst[2], &mut self.registers[inst[3] as usize]),
                Operands::ImmReg => (instr.op)(inst[1], self.registers[inst[2] as usize], &mut self.registers[inst[3] as usize]),
                _ => panic!("unknown")
            }
            //println!("exec: {} regs_b: {:?} ops: {:?} result: {:?} regs_a: {:?}", instr.mnemonic, regs, inst, result, self.registers);
            if self.registers == *result {
                count += 1;
                let line = &mut stats[inst[0] as usize];
                line[index] += 1;
            }
            index += 1;
        }
        count
    }

    pub fn execute_part2(&mut self, input: &[u32; 4]) {
        let inst = &self.instructions[input[0] as usize];
        match inst.ops {
            Operands::RegReg => {
                println!("{} {}, {}, {}", inst.mnemonic, self.registers[input[1] as usize], self.registers[input[2] as usize], self.registers[input[3] as usize]);
                (inst.op)(self.registers[input[1] as usize], self.registers[input[2] as usize], &mut self.registers[input[3] as usize])
            },
            Operands::RegImm => {
                println!("{} {}, {}, {}", inst.mnemonic, self.registers[input[1] as usize], input[2], self.registers[input[3] as usize]);
                (inst.op)(self.registers[input[1] as usize], input[2], &mut self.registers[input[3] as usize])
            },
            Operands::ImmReg => {
                println!("{} {}, {}, {}", inst.mnemonic, input[1], self.registers[input[2] as usize], self.registers[input[3] as usize]);
                (inst.op)(input[1], self.registers[input[2] as usize], &mut self.registers[input[3] as usize])
            },
            _ => panic!("unknown")
        }
    }
}

#[allow(dead_code)]
fn part1(path: &str) -> u32 {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut device = Device::new();
    let mut regs: [u32; 4] = [0u32; 4];
    let mut inst: [u32; 4] = [0u32; 4];
    let mut result: [u32; 4] = [0u32; 4];

    let reader = BufReader::new(file);
    let mut lc = 0;
    let mut count = 0u32;

    let mut stats: Vec<Vec<u32>> = vec![vec![0u32; 16]; 16];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                match lc % 4 {
                    0 => {
                        let (a, b, c, d) = scan_fmt!(&line, "Before: [{}, {}, {}, {}]", u32, u32, u32, u32);
                        regs[0] = a.unwrap();
                        regs[1] = b.unwrap();
                        regs[2] = c.unwrap();
                        regs[3] = d.unwrap();
                    },
                    1 => {
                        let (a, b, c, d) = scan_fmt!(&line, "{} {} {} {}", u32, u32, u32, u32);
                        inst[0] = a.unwrap();
                        inst[1] = b.unwrap();
                        inst[2] = c.unwrap();
                        inst[3] = d.unwrap();
                    },
                    2 => {
                        let (a, b, c, d) = scan_fmt!(&line, "After: [{}, {}, {}, {}]", u32, u32, u32, u32);
                        result[0] = a.unwrap();
                        result[1] = b.unwrap();
                        result[2] = c.unwrap();
                        result[3] = d.unwrap();

                        if device.execute_part1(&regs, &inst, &result, &mut stats) >= 3 {
                            count += 1;
                        }
                    },
                    _ => {}
                }
                lc += 1;
            }
            Err(e) => println!("err: {}", e)
        }
    }

    // compute and print opcodes
    let mut assigned: [bool; 16] = [false; 16];
    let mut assigned_ops: [bool; 16] = [false; 16];
    let mut assign_count = 0;
    while assign_count < 16 {
        let mut opcode = 0u32;
        let orig_count = assign_count;
        for arr in &stats {
            if assigned_ops[opcode as usize] {
                opcode += 1;
                continue;
            }

            // find unique
            let mut num = 0u32;
            let mut idx = 0;
            for x in 0..16 {
                if arr[x] > 0 && !assigned[x] {
                    num += 1;
                    idx = x;
                }
            }
            if num == 1 {
                //println!("assign {} to {}", opcode, device.instructions[idx].mnemonic);
                assign_count += 1;
                assigned[idx] = true;
                assigned_ops[opcode as usize] = true;
                device.instructions[idx].opcode = opcode;
                break;
            } else {
                //println!("{} num {}", opcode, num);
                opcode += 1;
            }
        }
        assert_eq!(assign_count - 1, orig_count);
    }

    for inst in &device.instructions {
        println!("{}, {}", inst.mnemonic, inst.opcode);
    }

    count
}

#[allow(dead_code)]
fn part2(path: &str) -> u32 {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut device = Device::new();
    device.instructions.sort_by(|a, b| a.opcode.cmp(&b.opcode));
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (a, b, c, d) = scan_fmt!(&line, "{} {} {} {}", u32, u32, u32, u32);
                let mut inst: [u32; 4] = [a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap()];
                println!("exec: {:?}", inst);
                device.execute_part2(&inst);
            }
            Err(e) => println!("err: {}", e)
        }
    }

    device.registers[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_ex0() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\test.txt"), 1);
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt"), 677);
    }
}

fn main() {
    println!("result: {}", part2(r"C:\Users\Igascoigne\advent2018\dec_01_01\input2.txt"));
    //println!("result: {}", part1(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt"));
}
