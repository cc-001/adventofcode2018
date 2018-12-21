#![feature(test)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

extern crate test;

const NUM_REGS: usize = 6;

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
    // diff vs. day19.rs must wrap
    *c = a.wrapping_add(b);
}

fn mul(a: u32, b: u32, c: &mut u32) {
    // diff vs. day19.rs must wrap
    *c = a.wrapping_mul(b);
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

// data breakpoint w/write
// substitutes for actual debugger twiddling since i don't have one with Intellij rust plugin
struct DataBreakpoint {
    reg: u32,
    ip: u32,
    data: u32,
    write_reg: u32,
    write_val: u32,
    run_count: u32
}

struct Device {
    registers: [u32; NUM_REGS],
    instructions: Vec<Instruction>,
    ip: u32,
    bound: usize,
    program: Vec<[u32; 4]>,
    breakpoints: Vec<DataBreakpoint>
}

impl Device {
    pub fn new() -> Device {
        Device {
            registers: [0u32; NUM_REGS],
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
            ],
            ip: 0,
            bound: 0,
            program: Vec::new(),
            breakpoints: Vec::new()
        }
    }

    pub fn get_opcode(&self, mnemonic: &str) -> Option<u32> {
        for inst in &self.instructions {
            if inst.mnemonic == mnemonic {
                return Some(inst.opcode);
            }
        }
        None
    }

    pub fn execute_ip(&mut self) -> bool {
        let instr = self.program[self.ip as usize];
        self.execute(&instr);
        (self.ip as usize) < self.program.len()
    }

    // note comment out printing if speed important as for part 2 on day 21
    pub fn execute(&mut self, input: &[u32; 4]) {
        let mut msg = String::new();
        let inst = &self.instructions[input[0] as usize];
        
        // data breakpoints before execute
        for breakpoint in &mut self.breakpoints {
            if (breakpoint.run_count > 0 || breakpoint.run_count < 0) && breakpoint.ip == self.ip && breakpoint.data == self.registers[breakpoint.reg as usize] {
                println!("hit breakpoint on ip: {} reg: {} data: {} write_reg: {}", breakpoint.ip, breakpoint.reg, breakpoint.data, breakpoint.write_reg);
                self.registers[breakpoint.write_reg as usize] = breakpoint.write_val;
                if breakpoint.run_count > 0 {
                    breakpoint.run_count -= 1;
                }
            }
        }

        self.registers[self.bound] = self.ip;
        msg.push_str(&format!("ip={} {:?} ", self.ip, self.registers));
        match inst.ops {
            Operands::RegReg => {
                let mut in2 = 0;
                if (input[2] as usize) < NUM_REGS { in2 = input[2] as usize; }
                (inst.op)(self.registers[input[1] as usize], self.registers[in2], &mut self.registers[input[3] as usize]);
            },
            Operands::RegImm => {
                (inst.op)(self.registers[input[1] as usize], input[2], &mut self.registers[input[3] as usize])
            },
            Operands::ImmReg => {
                let mut in2 = 0;
                if (input[2] as usize) < NUM_REGS { in2 = input[2] as usize; }
                (inst.op)(input[1], self.registers[in2], &mut self.registers[input[3] as usize])
            }
        }
        msg.push_str(&format!("{} {} {} {} {:?}", inst.mnemonic, input[1], input[2], input[3], self.registers));
        println!("{}", msg);
        self.ip = self.registers[self.bound];
        self.ip += 1;
    }
}

#[allow(dead_code)]
fn solution(path: &str, reg0: u32) -> u32 {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut device = Device::new();
    device.registers[0] = reg0;
    device.instructions.sort_by(|a, b| a.opcode.cmp(&b.opcode));

    let reader = BufReader::new(file);
    let mut lc = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if lc == 0 {
                    let a = scan_fmt!(&line, "#ip {}", u32);
                    device.bound = a.unwrap() as usize;
                    lc += 1;
                } else {
                    let (a, b, c, d) = scan_fmt!(&line, "{} {} {} {}", String, u32, u32, u32);
                    let mut inst: [u32; 4] = [device.get_opcode(&a.unwrap()).unwrap(), b.unwrap(), c.unwrap(), d.unwrap()];
                    device.program.push(inst);
                }
            }
            Err(e) => println!("err: {}", e)
        }
    }

    //device.registers[0] = 2985446; answer to part 1
    let mut hs = HashSet::new();
    while device.execute_ip() {
        if device.ip == 28 {
            // part 1
            // ip=28 this is when register 0 is tested
            // eqrr 4 0 2
            println!("device.registers[4]: {}", device.registers[4]);
            if (hs.contains(&device.registers[4])) {
                // this wraps with my input on 11235
                // part 2
                println!("hs.len(): {}", hs.len());
                return 0;
            }
            hs.insert(device.registers[4]);
            // part 1
            // return device.registers[4];
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_input() {
    }
}

fn main() {
    println!("result: {}", solution(r"C:\Users\Igascoigne\advent2018\dec_01_01\input.txt", 1));
}
