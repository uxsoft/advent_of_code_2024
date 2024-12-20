pub mod part1;
pub mod part2;

use std::ops::{BitXorAssign, Div, DivAssign};

use chumsky::prelude::*;
use itertools::Itertools;

pub struct Program<'a> {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub instructions: &'a Vec<u8>,
    pub instruction_pointer: usize,
    pub output: Vec<u8>,
}

#[derive(Debug)]
#[repr(u8)]
enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<&u8> for OpCode {
    fn from(value: &u8) -> Self {
        use OpCode::*;
        match value {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}

impl<'a> Program<'a> {
    pub fn new(a: usize, b: usize, c: usize, instructions: &Vec<u8>) -> Program {
        Program {
            a,
            b,
            c,
            instructions,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    #[inline]
    pub fn read_opcode(&self) -> Option<&u8> {
        self.instructions.get(self.instruction_pointer)
    }

    #[inline]
    pub fn read_operand(&self) -> Option<&u8> {
        self.instructions.get(self.instruction_pointer + 1)
    }

    #[inline]
    pub fn literal(&self, operand: &u8) -> usize {
        *operand as usize
    }

    #[inline]
    pub fn combo(&self, operand: &u8) -> usize {
        match operand {
            0..=3 => *operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand value!"),
        }
    }

    #[inline]
    pub fn output(&self) -> String {
        self.output.iter().join(",")
    }

    #[inline]
    pub fn adv(&mut self, operand: usize) {
        self.a.div_assign(2_usize.pow(operand as u32));
        self.instruction_pointer += 2;
    }

    #[inline]
    pub fn bxl(&mut self, operand: usize) {
        self.b.bitxor_assign(operand);
        self.instruction_pointer += 2;
    }

    #[inline]
    pub fn bst(&mut self, operand: usize) {
        self.b = operand.rem_euclid(8);
        self.instruction_pointer += 2;
    }

    #[inline]
    pub fn jnz(&mut self, operand: usize) {
        if self.a > 0 {
            self.instruction_pointer = operand;
        } else {
            self.instruction_pointer += 2;
        }
    }

    #[inline]
    pub fn bxc(&mut self, _operand: usize) {
        self.b.bitxor_assign(self.c);
        self.instruction_pointer += 2;
    }

    #[inline]
    pub fn out(&mut self, operand: usize) {
        self.output.push(operand.rem_euclid(8) as u8);
        self.instruction_pointer += 2;
    }

    #[inline]
    pub fn bdv(&mut self, operand: usize) {
        self.b = self.a.div(2_usize.pow(operand as u32));
        self.instruction_pointer += 2;
    }

    #[inline]
    pub fn cdv(&mut self, operand: usize) {
        self.c = self.a.div(2_usize.pow(operand as u32));
        self.instruction_pointer += 2;
    }

    pub fn run_to_end(&mut self) {
        use OpCode::*;

        while let (Some(opcode), Some(operand)) = (self.read_opcode(), self.read_operand()) {
            let opcode: OpCode = opcode.into();
            let operand_value = match opcode {
                Adv | Bst | Out | Bdv | Cdv => self.combo(operand),
                _ => self.literal(operand),
            };

            match opcode {
                Adv => self.adv(operand_value),
                Bxl => self.bxl(operand_value),
                Bst => self.bst(operand_value),
                Jnz => self.jnz(operand_value),
                Bxc => self.bxc(operand_value),
                Out => self.out(operand_value),
                Bdv => self.bdv(operand_value),
                Cdv => self.cdv(operand_value),
                other => panic!("OpCode {:?} not implemented in match", other),
            }
        }
    }

    pub fn parse(input: &str) -> (Vec<u8>, usize, usize, usize) {
        let usize = text::int::<_, _, extra::Err<Simple<char>>>(10)
            .map(|s: &str| s.parse::<usize>().unwrap());

        let u8 =
            text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<u8>().unwrap());

        let register = just("Register ")
            .ignore_then(just("A").or(just("B")).or(just("C")))
            .ignore_then(just(": "))
            .ignore_then(usize)
            .then_ignore(text::newline());

        let instructions = just("Program: ").ignore_then(u8.separated_by(just(",")).collect());

        let parser = register
            .then(register)
            .then(register)
            .then_ignore(text::newline())
            .then(instructions)
            .map(|(((a, b), c), i)| (i, a, b, c));

        let result = parser.parse(input).unwrap();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let result = Program::parse(input);
    }
}
