pub mod part1;
pub mod part2;

use chumsky::prelude::*;

pub struct Program {
    a: usize,
    b: usize,
    c: usize,
    instructions: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u8>
}

pub fn parse(input: &str) -> Program {
    let usize =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<usize>().unwrap());

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
        .map(|(((a, b), c), i)| Program {
            a,
            b,
            c,
            instructions: i,
            instruction_pointer: 0,
            output: Vec::new()
        });

    let result = parser.parse(input).unwrap();

    result
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

        let result = parse(input);
    }
}
