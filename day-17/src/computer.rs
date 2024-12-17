use std::env;

use regex::Regex;

use crate::Instruction::{self, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Computer {
    pub reg_a: u64,
    pub reg_b: u64,
    pub reg_c: u64,
}

impl Computer {
    pub fn new(reg_a: u64, reg_b: u64, reg_c: u64) -> Self {
        Self { reg_a, reg_b, reg_c }
    }

    pub fn with_reg_a(&self, reg_a: u64) -> Self {
        let mut ret = self.clone();
        ret.reg_a = reg_a;
        ret
    }
}

impl Computer {
    pub fn parse(s: &str) -> (Self, Vec<u64>) {
        let reg_re = Regex::new(r"^Register ([ABC]): (\d+)$").unwrap();
        let reg_prog = Regex::new(r"^Program: ([0-9,]+)$").unwrap();

        let lines: Vec<_> = s.split("\n").into_iter().collect();

        let reg_a = reg_re.captures(lines[0]).unwrap().extract::<2>().1[1];
        let reg_b = reg_re.captures(lines[1]).unwrap().extract::<2>().1[1];
        let reg_c = reg_re.captures(lines[2]).unwrap().extract::<2>().1[1];
        let instructions = reg_prog.captures(lines[4]).unwrap().extract::<1>().1[0];
        let instructions = instructions.split(",").map(|i| i.parse().unwrap()).collect();

        (
            Computer::new(reg_a.parse().unwrap(), reg_b.parse().unwrap(), reg_c.parse().unwrap()),
            instructions,
        )
    }
    pub fn combo(&self, val: u64) -> u64 {
        match val {
            0 | 1 | 2 | 3 => val,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("Bad instruction"),
        }
    }

    pub fn run(&mut self, instructions: &Vec<Instruction>) -> Vec<u64> {
        let debug = env::var("DEBUG").map_or(false, |var| var == "1");
        let mut ret = Vec::new();
        let mut pc = 0;
        while pc < instructions.len() {
            if debug {
                println!("pc: {}, opcode: {:?}, computer: {:?}", pc, instructions[pc], self);
            }
            match instructions[pc] {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A
                // register. The denominator is found by raising 2 to the power of the instruction's combo
                // operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by
                // 2^B.) The result of the division operation is truncated to an integer and then written to
                // the A register.
                Adv(val) => {
                    self.reg_a = self.reg_a >> self.combo(val);
                    pc += 1;
                },
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result
                // is stored in the B register. (The numerator is still read from the A register.)
                Bdv(val) => {
                    self.reg_b = self.reg_a >> self.combo(val);
                    pc += 1;
                },
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result
                // is stored in the C register. (The numerator is still read from the A register.)
                Cdv(val) => {
                    self.reg_c = self.reg_a >> self.combo(val);
                    pc += 1;
                },
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
                // instruction's literal operand, then stores the result in register B.
                Bxl(val) => {
                    self.reg_b = self.reg_b ^ val;
                    pc += 1;
                },
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby
                // keeping only its lowest 3 bits), then writes that value to the B register.
                Bst(val) => {
                    self.reg_b = self.combo(val) % 8;
                    pc += 1;
                },
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A
                // register is not zero, it jumps by setting the instruction pointer to the value of its
                // literal operand; if this instruction jumps, the instruction pointer is not increased by 2
                // after this instruction.
                Jnz(val) => {
                    if self.reg_a != 0 {
                        if val % 2 == 1 {
                            panic!("val is odd !");
                        }
                        pc = (val / 2) as usize;
                    } else {
                        pc += 1;
                    }
                },
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then
                // stores the result in register B. (For legacy reasons, this instruction reads an operand but
                // ignores it.)
                Bxc(_) => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                    pc += 1;
                },
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then
                // outputs that value. (If a program outputs multiple values, they are separated by commas.)
                Out(val) => {
                    ret.push(self.combo(val) % 8);
                    pc += 1;
                },
            }
        }
        ret
    }
}
