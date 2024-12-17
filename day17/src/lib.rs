// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

pub type Num = i32;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Registers {
    pub a: Num,
    pub b: Num,
    pub c: Num,
}

impl std::ops::Index<Reg> for Registers {
    type Output = Num;
    fn index(&self, reg: Reg) -> &Self::Output {
        match reg {
            Reg::A => &self.a,
            Reg::B => &self.b,
            Reg::C => &self.c,
        }
    }
}

impl std::ops::IndexMut<Reg> for Registers {
    fn index_mut(&mut self, reg: Reg) -> &mut Num {
        match reg {
            Reg::A => &mut self.a,
            Reg::B => &mut self.b,
            Reg::C => &mut self.c,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    pub fn is_combo(&self) -> bool {
        matches!(
            self,
            Instruction::Adv
                | Instruction::Bst
                | Instruction::Out
                | Instruction::Bdv
                | Instruction::Cdv
        )
    }
}
impl TryFrom<u8> for Instruction {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(format!("invalid instruction {}", v)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reg {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operand {
    Value(Num),
    Reg(Reg),
}

impl Operand {
    pub fn new_instr(instr: &Instruction, v: u8) -> Self {
        if instr.is_combo() {
            Operand::new_combo(v)
        } else {
            Operand::new_literal(v)
        }
    }
    pub fn new_literal(v: u8) -> Self {
        Operand::Value(v as Num)
    }
    pub fn new_combo(v: u8) -> Self {
        match v {
            0..=3 => Operand::Value(v as Num),
            4 => Operand::Reg(Reg::A),
            5 => Operand::Reg(Reg::B),
            6 => Operand::Reg(Reg::C),
            7 => panic!("reserved operand {}", v),
            _ => panic!("invalid operand {}", v),
        }
    }
}

pub type Operation = (Instruction, Operand);
pub type Program = Vec<Operation>;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Computer {
    pub regs: Registers,
    pub prog: Program,
    pub ip: usize,
    pub output: Vec<Num>,
}

impl Computer {
    pub fn new(regs: Registers, prog: Program) -> Computer {
        Computer {
            regs,
            prog,
            ..Default::default()
        }
    }

    pub fn get(&self, op: &Operand) -> Num {
        match op {
            Operand::Value(n) => *n,
            Operand::Reg(r) => self.regs[*r],
        }
    }

    pub fn once(&mut self) {
        assert!(self.ip % 2 == 0);
        let (instr, opcode) = self.prog[self.ip / 2];
        self.ip += 2;
        let op = self.get(&opcode);
        match instr {
            Instruction::Adv => {
                self.regs[Reg::A] /= 1 << op;
            }
            Instruction::Bxl => {
                self.regs[Reg::B] ^= op;
            }
            Instruction::Bst => {
                self.regs[Reg::B] = op % 8;
            }
            Instruction::Jnz => {
                if self.regs[Reg::A] != 0 {
                    self.ip = op as usize;
                }
            }
            Instruction::Bxc => {
                self.regs[Reg::B] ^= self.regs[Reg::C];
            }
            Instruction::Out => {
                self.output.push(op % 8);
            }
            Instruction::Bdv => {
                self.regs[Reg::B] = self.regs[Reg::A] / (1 << op);
            }
            Instruction::Cdv => {
                self.regs[Reg::C] = self.regs[Reg::A] / (1 << op);
            }
        }
    }

    pub fn run(&mut self) {
        while self.ip < 2 * self.prog.len() {
            // eprintln!("regs {:?}", self.regs);
            // let (instr, opcode) = self.prog[self.ip / 2];
            // let op = self.get(&opcode);
            // eprintln!("  instr {:?} op {:?}", instr, op);
            self.once();
        }
    }

    pub fn output_str(&self) -> String {
        let mut s = String::new();
        for (i, o) in self.output.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&format!("{}", o));
        }
        s
    }
}

#[test]
fn test1() {
    let regs = Registers { a: 0, b: 0, c: 9 };
    let prog = parser::parse_program("2,6".as_bytes()).unwrap();
    let mut cpu = Computer::new(regs, prog);
    cpu.run();
    assert_eq!(cpu.regs[Reg::B], 1);
}

#[test]
fn test2() {
    let regs = Registers { a: 10, b: 0, c: 0 };
    let prog = parser::parse_program("5,0,5,1,5,4".as_bytes()).unwrap();
    let mut cpu = Computer::new(regs, prog);
    cpu.run();
    assert_eq!(cpu.output_str(), "0,1,2");
}

#[test]
fn test3() {
    let regs = Registers {
        a: 2024,
        b: 0,
        c: 0,
    };
    let prog = parser::parse_program("0,1,5,4,3,0".as_bytes()).unwrap();
    let mut cpu = Computer::new(regs, prog);
    cpu.run();
    assert_eq!(cpu.output_str(), "4,2,5,6,7,7,7,7,3,1,0");
    assert_eq!(cpu.regs[Reg::A], 0);
}

#[test]
fn test4() {
    let regs = Registers { a: 0, b: 29, c: 0 };
    let prog = parser::parse_program("1,7".as_bytes()).unwrap();
    let mut cpu = Computer::new(regs, prog);
    cpu.run();
    assert_eq!(cpu.regs[Reg::B], 26);
}

#[test]
fn test5() {
    let regs = Registers {
        a: 0,
        b: 2024,
        c: 43690,
    };
    let prog = parser::parse_program("4,0".as_bytes()).unwrap();
    let mut cpu = Computer::new(regs, prog);
    cpu.run();
    assert_eq!(cpu.regs[Reg::B], 44354);
}

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> IResult<&str, Num> {
        character::i32(input)
    }

    fn registers(input: &str) -> IResult<&str, Registers> {
        let (input, _) = tag("Register A: ")(input)?;
        let (input, a) = num(input)?;
        let (input, _) = tag("\nRegister B: ")(input)?;
        let (input, b) = num(input)?;
        let (input, _) = tag("\nRegister C: ")(input)?;
        let (input, c) = num(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, Registers { a, b, c }))
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        let (input, instr) = digit1_one_of("01234567")(input)
            .map(|(input, c)| (input, Instruction::try_from(c).unwrap()))?;
        let (input, _) = tag(",")(input)?;
        let (input, opvalue) = digit1_one_of("01234567")(input)?;
        let operand = Operand::new_instr(&instr, opvalue);
        Ok((input, (instr, operand)))
    }

    fn program_contents(input: &str) -> IResult<&str, Program> {
        let (input, prg) = multi::separated_list1(tag(","), operation)(input)?;
        Ok((input, prg))
    }

    fn program(input: &str) -> IResult<&str, Program> {
        let (input, _) = tag("Program: ")(input)?;
        let (input, prg) = program_contents(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, prg))
    }

    fn all(input: &str) -> IResult<&str, Computer> {
        let (input, regs) = registers(input)?;
        let (input, _) = character::newline(input)?;
        let (input, prg) = program(input)?;
        Ok((input, Computer::new(regs, prg)))
    }

    pub fn parse(mut bufin: impl BufRead) -> Result<Computer> {
        aoc::parse_with!(all, bufin)
    }

    pub fn parse_program(mut bufin: impl BufRead) -> Result<Program> {
        aoc::parse_with!(program_contents, bufin)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE.as_bytes())?;
    assert_eq!(input.prog.len(), 3);
    Ok(())
}
