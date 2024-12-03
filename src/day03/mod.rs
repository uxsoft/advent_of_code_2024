use chumsky::prelude::*;
use miette::*;

mod part1;
mod part2;

#[derive(Debug)]
pub enum Token {
    Mul(i32, i32),
    Noop(char),
}

pub fn parse(input: &str) -> miette::Result<Vec<Token>> {
    let mul = just::<_, _, extra::Err<Simple<char>>>("mul(")
        .ignore_then(text::int(10).map(|s: &str| s.parse().unwrap()))
        .then_ignore(just(","))
        .then(text::int(10).map(|s: &str| s.parse().unwrap()))
        .then_ignore(just(")"))
        .map(|(a, b)| Token::Mul(a, b));

    // let mul_test = mul.parse("mul(2,3)").unwrap();
    // println!("MUL TEST: {:?}", mul_test);

    let mul_or = choice((mul, any().and_is(mul.not()).map(|s| Token::Noop(s))));

    // let mul_or_test = mul_or.parse("mul(2,4)").unwrap();
    // println!("MUL_OR TEST: {:?}", mul_or_test);

    //let b = mul_or.parse("input").unwrap();

    let parser = mul_or.repeated().collect();

    let result = parser
        .parse(input)
        .into_result()
        .map_err(|e| miette!("Failed to parse {:?}", e));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = parse(input);
        println!("{:?}", result);

        assert!(false);
    }
}
