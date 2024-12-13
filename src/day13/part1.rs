use super::ClawMachine;

pub fn process(input: &str) -> i64 {
    let claw_machines = super::parse(input);

    let tokens: i64 = claw_machines
        .iter()
        .map(|m| (m, m.coefficients()))
        .filter(|(_, (a, b))| *a > 0 && *b > 0 && *a <= 100 && *b <= 100)
        .filter(|(m, (a, b))| m.check(*a, *b))
        .map(|(_, (a, b))| 3 * a + b)
        .sum();

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
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

        let result = process(input);

        assert_eq!(480, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(26810, result);
    }
}
