use std::ops::{BitXor, BitXorAssign, Div, DivAssign};

use itertools::Itertools;

use super::Program;

pub fn process(input: &str) -> String {
    let (ins, a, b, c) = Program::parse(input);
    let mut prg = Program::new(a, b, c, &ins);
    prg.instructions = &ins;
    prg.run_to_end();

    prg.output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_example_1() {
        let i = vec![2, 6];
        let mut prg = Program::new(0, 0, 9, &i);
        prg.run_to_end();
        assert_eq!(prg.b, 1);
    }

    #[test]
    fn instructions_example_2() {
        let i = vec![5, 0, 5, 1, 5, 4];
        let mut prg = Program::new(10, 0, 0, &i);
        prg.run_to_end();
        assert_eq!(prg.output(), "0,1,2");
    }

    #[test]
    fn instructions_example_3() {
        let i = vec![0, 1, 5, 4, 3, 0];
        let mut prg = Program::new(2024, 0, 0, &i);
        prg.run_to_end();
        assert_eq!(prg.output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(prg.a, 0);
    }

    #[test]
    fn instructions_example_4() {
        let i = vec![1, 7];
        let mut prg = Program::new(0, 29, 0, &i);
        prg.run_to_end();
        assert_eq!(prg.b, 26);
    }

    #[test]
    fn instructions_example_5() {
        let i = vec![4, 0];
        let mut prg = Program::new(0, 2024, 43690, &i);
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
        assert_eq!(output, "6,0,6,3,0,2,3,1,6");
    }
}
