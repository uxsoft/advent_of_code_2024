fn compute(initial: usize, steps: usize) -> usize {
    let mut iterator = super::PseudoRandomIterator::new(initial);
    iterator.nth(steps).unwrap()
}

pub fn process(input: &str) -> usize {
    let numbers = super::parse(input);
    let total = numbers.into_iter().map(|i| compute(i, 2000)).sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_2000() {
        let cases = vec![
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ];

        for (initial, expected) in cases {
            let output = compute(initial, 2000);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn example() {
        let input = "1
10
100
2024";
        let output = process(input);
        assert_eq!(output, 37327623);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, 14273043166);
    }
}
