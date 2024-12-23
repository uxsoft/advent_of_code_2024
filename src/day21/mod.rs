pub mod keypad;
pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "029A
980A
179A
456A
379A";
        let result = parse(input);
        assert_eq!(result.len(), 5)
    }
}
