pub fn process(input: &str) -> miette::Result<i32> {
    let tokens = super::parse(input)?;

    let (sum, _) = tokens
        .iter()
        .fold((0, true), |(sum, enabled), token| match token {
            super::Token::Mul(a, b) => {
                if enabled {
                    (sum + (a * b), enabled)
                } else {
                    (sum, enabled)
                }
            }
            super::Token::Noop(_) => (sum, enabled),
            super::Token::Do => (sum, true),
            super::Token::Dont => (sum, false),
        });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = process(input).unwrap();
        assert_eq!(result, 48);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input).unwrap();
        assert_eq!(result, 0);
    }
}
