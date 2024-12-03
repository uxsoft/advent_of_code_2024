pub fn process(input: &str) -> miette::Result<i32> {
    let tokens = super::parse(input)?;

    let sum = tokens
        .iter()
        .map(|token| match token {
            super::Token::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<i32>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = process(input).unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input).unwrap();
        assert_eq!(result, 160672468);
    }
}
