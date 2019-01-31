use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use regex::Regex;

pub fn solve(filename: String) {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    /* Data:
     *
     *
     * #ip 1
     * addi 1 16 1
     * seti 1 4 2
     * seti 1 0 3
     * mulr 2 3 4
     * eqrr 4 5 4
     * addr 4 1 1
     * addi 1 1 1
     * addr 2 0 0
     * addi 3 1 3
     * gtrr 3 5 4
     * addr 1 4 1
     * seti 2 4 1
     * addi 2 1 2
     * gtrr 2 5 4
     * addr 4 1 1
     * seti 1 1 1
     * mulr 1 1 1
     * addi 5 2 5
     * mulr 5 5 5
     * mulr 1 5 5
     * muli 5 11 5
     * addi 4 2 4
     * mulr 4 1 4
     * addi 4 16 4
     * addr 5 4 5
     * addr 1 0 1
     * seti 0 7 1
     * setr 1 5 4
     * mulr 4 1 4
     * addr 1 4 4
     * mulr 1 4 4
     * muli 4 14 4
     * mulr 4 1 4
     * addr 5 4 5
     * seti 0 9 0
     * seti 0 4 1
     *
     */

    let instruction_re = Regex::new(r"(?m)^(\w+) (\d+) (\d+) (\d+)$").unwrap();

    let mut instructions = Vec::new();
    for cap in instruction_re.captures_iter(&input) {
        instructions.push(Instruction {
            op: cap[1].parse().unwrap(),
            a: cap[2].parse().unwrap(),
            b: cap[3].parse().unwrap(),
            c: cap[4].parse().unwrap(),
        });
    }
    println!("{} instructions parsed", instructions.len());

    let mut registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
    };

    loop {
        //println!("{}: {:?}", registers.b, instructions[registers.b as usize]);

        registers = registers
            .evaluate(&instructions[registers.b as usize])
            .unwrap();
        registers.b += 1;

        if registers.b as usize >= instructions.len() {
            break;
        }
    }
    println!("{:?}", registers);

    let mut registers = Registers {
        a: 1,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
    };

    let mut old_a = 1;

    loop {
        //println!("{}: {:?}", registers.b, instructions[registers.b as usize]);

        registers = registers
            .evaluate(&instructions[registers.b as usize])
            .unwrap();
        registers.b += 1;

        if registers.b as usize >= instructions.len() {
            break;
        }
        if registers.a != old_a {
            println!("{:?}", registers);
            old_a = registers.a;
        }
    }
    println!("{:?}", registers);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    a: u64,
    b: u64,
    c: u64,
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

impl FromStr for Operation {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "addr" => Ok(Operation::Addr),
            "addi" => Ok(Operation::Addi),
            "mulr" => Ok(Operation::Mulr),
            "muli" => Ok(Operation::Muli),
            "banr" => Ok(Operation::Banr),
            "bani" => Ok(Operation::Bani),
            "borr" => Ok(Operation::Borr),
            "bori" => Ok(Operation::Bori),
            "setr" => Ok(Operation::Setr),
            "seti" => Ok(Operation::Seti),
            "gtir" => Ok(Operation::Gtir),
            "gtri" => Ok(Operation::Gtri),
            "gtrr" => Ok(Operation::Gtrr),
            "eqir" => Ok(Operation::Eqir),
            "eqri" => Ok(Operation::Eqri),
            "eqrr" => Ok(Operation::Eqrr),
            _ => Err(From::from("Invalid opcode")),
        }
    }
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = instruction.b;
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src A register"),
                };
                let src_b = match instruction.b {
                    0 => temp.a,
                    1 => temp.b,
                    2 => temp.c,
                    3 => temp.d,
                    4 => temp.e,
                    5 => temp.f,
                    _ => return Err("Illegal src B register"),
                };
                let dst = match instruction.c {
                    0 => &mut temp.a,
                    1 => &mut temp.b,
                    2 => &mut temp.c,
                    3 => &mut temp.d,
                    4 => &mut temp.e,
                    5 => &mut temp.f,
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
