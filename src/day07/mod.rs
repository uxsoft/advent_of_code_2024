use chumsky::prelude::*;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let u64 =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<u64>().unwrap());

    let components = u64.separated_by(just(' ')).collect();

    let equation = u64.then_ignore(just(": ")).then(components);

    let parser = equation.separated_by(text::newline()).collect();

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(result.len(), 9);
    }
}
