use chumsky::container::Container;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, LinkedList};

fn is_valid_order(pages: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let page_to_rank: HashMap<u32, usize> = pages
        .into_iter()
        .enumerate()
        .map(|(index, p)| (*p, index + 1))
        .collect();

    let is_valid = pages.into_iter().all(|page| {
        let before_index = page_to_rank[page];

        rules
            .get(page)
            .unwrap_or(&vec![])
            .iter()
            .all(|after| page_to_rank.get(after).unwrap_or(&usize::MAX) > &before_index)
    });

    return is_valid;
}

fn middle_number(pages: LinkedList<u32>) -> u32 {
    let index = pages.len() / 2;
    let number = pages.iter().nth(index).expect(&format!(
        "Expected pages.len() > 0 but length {}",
        pages.len()
    ));

    return *number;
}

fn topological_visit(
    node: u32,
    nodes: &Vec<u32>,
    rules: &HashMap<u32, Vec<u32>>,
    permanent_mark: &mut HashSet<u32>,
    sorted_nodes: &mut LinkedList<u32>,
) {
    if permanent_mark.contains(&node) {
        return;
    }

    for v in rules
        .get(&node)
        .unwrap_or(&vec![])
        .iter()
        .filter(|v| nodes.contains(v))
    {
        topological_visit(*v, &nodes, &rules, permanent_mark, sorted_nodes);
    }

    permanent_mark.push(node);

    sorted_nodes.push_front(node);
}

fn topological_sort(nodes: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> LinkedList<u32> {
    // Topological Sort
    // u comes before v means an edge from u to v

    let mut sorted_nodes = LinkedList::new();
    let mut permanent_mark = HashSet::new();

    for node in nodes {
        topological_visit(*node, &nodes, rules, &mut permanent_mark, &mut sorted_nodes);
    }

    sorted_nodes
}

pub fn process(input: &str) -> miette::Result<u32> {
    let (rules, manuals) = super::parse(input)?;

    let rule_map = rules.into_iter().fold(HashMap::new(), |mut map, (a, b)| {
        map.entry(a)
            .and_modify(|v: &mut Vec<u32>| v.push(b))
            .or_insert(vec![b]);
        map
    });

    let total: u32 = manuals
        .par_iter()
        .filter(|pages| !is_valid_order(pages, &rule_map))
        .map(|pages| topological_sort(pages, &rule_map)) // TODO Try built-in sort_by
        .map(middle_number)
        .sum();

    return Ok(total);
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

        let result = process(input).unwrap();

        println!("{:?}", result);

        assert_eq!(123, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input).unwrap();

        println!("{:?}", result);

        assert_eq!(4713, result);
    }
}
