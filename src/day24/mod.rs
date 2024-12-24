use chumsky::prelude::*;
use std::collections::HashSet;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    pub fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Op::AND => a & b,
            Op::OR => a | b,
            Op::XOR => a ^ b,
        }
    }
}

type Statement<'a> = (&'a str, Op, &'a str, &'a str);

pub fn parse(input: &str) -> (Vec<(&str, bool)>, Vec<Statement>) {
    let bool = choice((just("0").to(false), just("1").to(true)));

    let op = choice((
        just("AND").to(Op::AND),
        just("OR").to(Op::OR),
        just("XOR").to(Op::XOR),
    ));

    let name = text::ident::<_, _, extra::Err<Simple<char>>>();

    let var = name.then_ignore(just(": ")).then(bool);

    let vars = var.separated_by(text::newline()).collect();

    let statement = name
        .then_ignore(just(" "))
        .then(op)
        .then_ignore(just(" "))
        .then(name)
        .then_ignore(just(" -> "))
        .then(name)
        .map(|(((a, b), c), d)| (a, b, c, d));

    let statements = statement.separated_by(text::newline()).collect();

    let parser = vars
        .then_ignore(text::newline().then(text::newline()))
        .then(statements);

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

        let (vars, statements) = parse(input);
        assert_eq!(vars.len(), 6);
        assert_eq!(statements.len(), 3);
    }
}
