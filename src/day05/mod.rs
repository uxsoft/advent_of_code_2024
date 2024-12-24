use chumsky::prelude::*;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let u32 =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<u32>().unwrap());

    let ordering_rule = u32.then_ignore(just("|")).then(u32).map(|(a, b)| (a, b));
    let ordering_rules = ordering_rule.separated_by(text::newline()).collect();

    let page_update = u32.separated_by(just(",")).collect();
    let page_updates = page_update.separated_by(text::newline()).collect();

    let parser = ordering_rules
        .then_ignore(text::newline())
        .then_ignore(text::whitespace())
        .then(page_updates);

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = parse(input);

        println!("{:?}", result);

        assert!(true);
    }
}
