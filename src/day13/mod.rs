pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    pub fn check(&self, a: i64, b: i64) -> bool {
        let px = a * self.a.0 + b * self.b.0;
        let py = a * self.a.1 + b * self.b.1;

        px == self.prize.0 && py == self.prize.1
    }

    pub fn coefficients(&self) -> (i64, i64) {
        let a = (self.prize.0 * self.b.1 - self.prize.1 * self.b.0)
            / (self.a.0 * self.b.1 - self.b.0 * self.a.1);

        let b = (self.a.0 * self.prize.1 - self.prize.0 * self.a.1)
            / (self.a.0 * self.b.1 - self.b.0 * self.a.1);

        (a, b)
    }
}

pub fn parse(input: &str) -> Vec<ClawMachine> {
    use chumsky::prelude::*;

    let i64 =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<i64>().unwrap());

    let button = just("Button ")
        .ignore_then(just("A").or(just("B")))
        .ignore_then(just(": X+"))
        .ignore_then(i64)
        .then_ignore(just(", Y+"))
        .then(i64)
        .then_ignore(text::newline());

    let prize = just("Prize: X=")
        .ignore_then(i64)
        .then_ignore(just(", Y="))
        .then(i64);

    let claw_machine = button
        .then(button)
        .then(prize)
        .map(|((a, b), prize)| ClawMachine { a, b, prize });

    let parser = claw_machine.separated_by(text::newline().then(text::newline())).collect();

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(4, result.len());
    }
}
