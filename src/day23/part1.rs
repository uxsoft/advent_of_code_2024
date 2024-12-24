use std::collections::{HashMap, HashSet};

use chumsky::container::Container;
use indicatif::ProgressIterator;
use itertools::Itertools;
use petgraph::{algo::*, graph::NodeIndex};

pub fn process(input: &str) -> usize {
    let edge_list: Vec<_> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let nodes = edge_list
        .iter()
        .flat_map(|(from, to)| [(*from, *to), (*to, *from)])
        .into_group_map();

    let tripples: HashSet<_> = nodes
        .keys()
        .tuple_combinations()
        .progress_count(6430034260)
        .filter(|(a, b, c)| nodes[*a].contains(b) && nodes[*b].contains(c) && nodes[*c].contains(a))
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .collect();

    tripples.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let output = process(input);
        assert_eq!(output, 7);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, 1175);
    }
}
