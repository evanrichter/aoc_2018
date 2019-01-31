use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    /* Sample-data:
     *
     * Before: [3, 1, 2, 0]
     * 5 1 2 0
     * After:  [0, 1, 2, 0]
     *
     * Before: [3, 3, 0, 2]
     * 10 2 0 1
     * After:  [3, 0, 0, 2]
     *
     */

    let re = Regex::new(r"(?m)^Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]\n$")
        .unwrap();

    let mut samples = Vec::new();
    for cap in re.captures_iter(&input) {
        samples.push(Sample {
            before: Registers {
                a: cap[1].parse::<u16>().unwrap(),
                b: cap[2].parse::<u16>().unwrap(),
                c: cap[3].parse::<u16>().unwrap(),
                d: cap[4].parse::<u16>().unwrap(),
            },
            opcode: Registers {
                a: cap[5].parse::<u16>().unwrap(),
                b: cap[6].parse::<u16>().unwrap(),
                c: cap[7].parse::<u16>().unwrap(),
                d: cap[8].parse::<u16>().unwrap(),
            },
            after: Registers {
                a: cap[9].parse::<u16>().unwrap(),
                b: cap[10].parse::<u16>().unwrap(),
                c: cap[11].parse::<u16>().unwrap(),
                d: cap[12].parse::<u16>().unwrap(),
            },
        });
    }
    println!("Found {} samples", samples.iter().count());

    let mut possible_opcodes: HashMap<Operation, BTreeSet<u16>> = HashMap::with_capacity(16);

    {
        let mut allopcodes = BTreeSet::new();
        for o in samples.iter().map(|s| s.opcode.a) {
            allopcodes.insert(o);
        }

        possible_opcodes.insert(Operation::Addr, allopcodes.clone());
        possible_opcodes.insert(Operation::Addi, allopcodes.clone());
        possible_opcodes.insert(Operation::Mulr, allopcodes.clone());
        possible_opcodes.insert(Operation::Muli, allopcodes.clone());
        possible_opcodes.insert(Operation::Banr, allopcodes.clone());
        possible_opcodes.insert(Operation::Bani, allopcodes.clone());
        possible_opcodes.insert(Operation::Borr, allopcodes.clone());
        possible_opcodes.insert(Operation::Bori, allopcodes.clone());
        possible_opcodes.insert(Operation::Setr, allopcodes.clone());
        possible_opcodes.insert(Operation::Seti, allopcodes.clone());
        possible_opcodes.insert(Operation::Gtir, allopcodes.clone());
        possible_opcodes.insert(Operation::Gtri, allopcodes.clone());
        possible_opcodes.insert(Operation::Gtrr, allopcodes.clone());
        possible_opcodes.insert(Operation::Eqir, allopcodes.clone());
        possible_opcodes.insert(Operation::Eqri, allopcodes.clone());
        possible_opcodes.insert(Operation::Eqrr, allopcodes.clone());
    }

    let mut counts = Vec::new();

    for sample in samples.iter() {
        let mut count = 0;
        for (mut op, mut opcode_set) in possible_opcodes.iter_mut() {
            let guess = Instruction {
                op: op.clone(),
                a: sample.opcode.b,
                b: sample.opcode.c,
                c: sample.opcode.d,
            };
            match sample.before.evaluate(&guess) {
                Ok(reg) => {
                    if sample.after != reg {
                        opcode_set.remove(&sample.opcode.a);
                    } else {
                        count += 1;
                    }
                }
                Err(_) => {
                    opcode_set.remove(&sample.opcode.a);
                }
            };
        }
        counts.push(count);
    }

    println!(
        "PART1: samples with three or more possible: {}",
        counts.iter().filter(|&&c| c >= 3).count()
    );

    //part2
    let mut real_opcodes: HashMap<Operation, u16> = HashMap::new();
    loop {
        for single in possible_opcodes.iter().filter(|(_, v)| v.len() == 1) {
            let (key, val) = single;
            real_opcodes.insert(key.clone(), val.iter().next().unwrap().clone());
        }

        for (_, mut opcode_set) in possible_opcodes.iter_mut() {
            for opcode in real_opcodes.values() {
                opcode_set.remove(opcode);
            }
        }

        if real_opcodes.len() == 16 {
            break;
        }
    }
    for opcode in real_opcodes {
        println!("{:?}", opcode);
    }

    let mut program = String::new();
    File::open("input/day16b")
        .unwrap()
        .read_to_string(&mut program)
        .unwrap();

    let re = Regex::new(r"(?m)^(\d+) (\d+) (\d+) (\d+)$").unwrap();

    let mut registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    for cap in re.captures_iter(&program) {
        match Instruction::from(Registers {
            a: cap[1].parse::<u16>().unwrap(),
            b: cap[2].parse::<u16>().unwrap(),
            c: cap[3].parse::<u16>().unwrap(),
            d: cap[4].parse::<u16>().unwrap(),
        }) {
            Ok(inst) => {
                let result = match registers.evaluate(&inst) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                };
                registers = result;
                println!("{:?}", registers);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Final state: {:?}", registers);
}

#[derive(Debug)]
struct Sample {
    before: Registers,
    opcode: Registers,
    after: Registers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Registers {
    a: u16,
    b: u16,
    c: u16,
    d: u16,
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    a: u16,
    b: u16,
    c: u16,
}

impl Instruction {
    fn from(registers: Registers) -> Result<Self, &'static str> {
        return Ok(Instruction {
            op: match registers.a {
                0 => Operation::Bani,
                1 => Operation::Addr,
                2 => Operation::Mulr,
                3 => Operation::Addi,
                4 => Operation::Gtri,
                5 => Operation::Banr,
                6 => Operation::Borr,
                7 => Operation::Eqri,
                8 => Operation::Seti,
                9 => Operation::Eqrr,
                10 => Operation::Bori,
                11 => Operation::Setr,
                12 => Operation::Eqir,
                13 => Operation::Muli,
                14 => Operation::Gtrr,
                15 => Operation::Gtir,
                _ => return Err("Unknown opcode"),
            },
            a: registers.b,
            b: registers.c,
            c: registers.d,
        });
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Registers {
    /// Evaluate instruction and mutate register state
    /*
    fn evaluate_in_place(&mut self, instruction: &Instruction) -> Result<(), &str> {
        //let imm_self = &*self;
        let res = match Registers::evaluate(&*self, instruction) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        self.a = res.a;
        return Ok(());
    }
    */

    /// Evaluate instruction and return next register state
    fn evaluate(&self, instruction: &Instruction) -> Result<Registers, &str> {
        let mut temp = self.clone();
        match instruction.op {
            // (add register) stores into register C the result of adding register A and register B.
            Operation::Addr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a + src_b;
            }
            // addi (add immediate) stores into register C the result of adding register A and value B.
            Operation::Addi => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a + src_b;
            }
            // mulr (multiply register) stores into register C the result of multiplying register A and register B.
            Operation::Mulr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a * src_b;
            }
            // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
            Operation::Muli => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a * src_b;
            }
            // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
            Operation::Banr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a & src_b;
            }
            // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
            Operation::Bani => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a & src_b;
            }
            // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
            Operation::Borr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a | src_b;
            }
            // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
            Operation::Bori => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a | src_b;
            }
            // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
            Operation::Setr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a;
            }
            // seti (set immediate) stores value A into register C. (Input B is ignored.)
            Operation::Seti => {
                let src_a = instruction.a;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = src_a;
            }
            // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
            Operation::Gtir => {
                let src_a = instruction.a;
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = if src_a > src_b { 1 } else { 0 };
            }
            // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
            Operation::Gtri => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = if src_a > src_b { 1 } else { 0 };
            }
            // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
            Operation::Gtrr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = if src_a > src_b { 1 } else { 0 };
            }
            // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
            Operation::Eqir => {
                let src_a = instruction.a;
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = if src_a == src_b { 1 } else { 0 };
            }
            // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
            Operation::Eqri => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = if src_a == src_b { 1 } else { 0 };
            }
            // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
            Operation::Eqrr => {
                let src_a = match instruction.a {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    _ => return Err("Illegal dst register"),
                };

                *dst = if src_a == src_b { 1 } else { 0 };
            }
        }

        Ok(temp)
    }
}

#[test]
fn eval() {
    let a = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let ans = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 7,
    };
    assert_eq!(
        a.evaluate(&Instruction {
            op: Operation::Addi,
            a: 0,
            b: 7,
            c: 3
        })
        .unwrap(),
        ans
    );
}
