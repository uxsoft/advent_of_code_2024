use chumsky::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub enum Token {
    Mul(i32, i32),
    Noop(char),
    Do,
    Dont,
}

pub fn parse(input: &str) -> Vec<Token> {
    let u32 = text::int(10).map(|s: &str| s.parse().unwrap());

    let mul = just::<_, _, extra::Err<Simple<char>>>("mul(")
        .ignore_then(u32)
        .then_ignore(just(","))
        .then(u32)
        .then_ignore(just(")"))
        .map(|(a, b)| Token::Mul(a, b));

    let mul_or = choice((
        mul,
        just("do()").to(Token::Do),
        just("don't()").to(Token::Dont),
        any().and_is(mul.not()).map(|s| Token::Noop(s)),
    ));

    let parser = mul_or.repeated().collect();

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(result.len(), 0);
    }
}
