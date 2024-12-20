use std::ops::{BitXor, BitXorAssign, Div, DivAssign};

use itertools::Itertools;

use super::Program;

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

impl Program {
    pub fn new(a: usize, b: usize, c: usize, instructions: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            instructions,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    pub fn read_opcode(&self) -> Option<&u8> {
        self.instructions.get(self.instruction_pointer)
    }

    pub fn read_operand(&self) -> Option<&u8> {
        self.instructions.get(self.instruction_pointer + 1)
    }

    pub fn literal(&self, operand: &u8) -> usize {
        *operand as usize
    }

    pub fn combo(&self, operand: &u8) -> usize {
        match operand {
            0..=3 => *operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand value!"),
        }
    }

    pub fn output(&self) -> String {
        self.output.iter().join(",")
    }

    pub fn adv(&mut self, operand: usize) {
        self.a.div_assign(2_usize.pow(operand as u32));
        self.instruction_pointer += 2;
    }

    pub fn bxl(&mut self, operand: usize) {
        self.b.bitxor_assign(operand);
        self.instruction_pointer += 2;
    }

    pub fn bst(&mut self, operand: usize) {
        self.b = operand.rem_euclid(8);
        self.instruction_pointer += 2;
    }

    pub fn jnz(&mut self, operand: usize) {
        if self.a > 0 {
            self.instruction_pointer = operand;
        } else {
            self.instruction_pointer += 2;
        }
    }

    pub fn bxc(&mut self, _operand: usize) {
        self.b.bitxor_assign(self.c);
        self.instruction_pointer += 2;
    }

    pub fn out(&mut self, operand: usize) {
        self.output.push(operand.rem_euclid(8) as u8);
        self.instruction_pointer += 2;
    }

    pub fn bdv(&mut self, operand: usize) {
        self.b = self.a.div(2_usize.pow(operand as u32));
        self.instruction_pointer += 2;
    }

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
}

pub fn process(input: &str) -> String {
    let mut prg = super::parse(input);
    prg.run_to_end();

    prg.output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_example_1() {
        let mut prg = Program::new(0, 0, 9, vec![2, 6]);
        prg.run_to_end();
        assert_eq!(prg.b, 1);
    }

    #[test]
    fn instructions_example_2() {
        let mut prg = Program::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        prg.run_to_end();
        assert_eq!(prg.output(), "0,1,2");
    }

    #[test]
    fn instructions_example_3() {
        let mut prg = Program::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        prg.run_to_end();
        assert_eq!(prg.output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(prg.a, 0);
    }

    #[test]
    fn instructions_example_4() {
        let mut prg = Program::new(0, 29, 0, vec![1, 7]);
        prg.run_to_end();
        assert_eq!(prg.b, 26);
    }

    #[test]
    fn instructions_example_5() {
        let mut prg = Program::new(0, 2024, 43690, vec![4, 0]);
        prg.run_to_end();
        assert_eq!(prg.b, 44354);
    }

    #[test]
    fn example() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let output = process(input);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, "");
    }
}
