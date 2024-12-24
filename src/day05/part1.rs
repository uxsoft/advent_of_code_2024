use std::collections::HashMap;

fn is_valid_order(pages: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    let page_to_rank: HashMap<u32, usize> = pages
        .into_iter()
        .enumerate()
        .map(|(a, b)| (*b, a + 1))
        .collect();

    let is_valid = rules.iter().all(|(first_page, second_page)| {
        page_to_rank.get(first_page).unwrap_or(&usize::MIN)
            < page_to_rank.get(second_page).unwrap_or(&usize::MAX)
    });

    return is_valid;
}

fn middle_number(pages: Vec<u32>) -> u32 {
    let index = pages.len() / 2;
    let number = pages.get(index).expect(&format!(
        "Expected pages.len() > 0 but length {}",
        pages.len()
    ));

    return *number;
}

pub fn process(input: &str) -> u32 {
    let (rules, manuals) = super::parse(input);

    let total: u32 = manuals
        .into_iter()
        .filter(|pages| is_valid_order(pages, &rules))
        .map(middle_number)
        .sum();

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
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

        let result = process(input);

        println!("{:?}", result);

        assert_eq!(143, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        println!("{:?}", result);

        assert_eq!(7307, result);
    }
}
