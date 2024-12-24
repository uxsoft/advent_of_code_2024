use std::collections::{HashMap, HashSet, VecDeque};

use chumsky::container::Container;
use indicatif::ProgressIterator;
use itertools::Itertools;
use petgraph::{algo::*, graph::NodeIndex};

pub fn process(input: &str) -> String {
    let edge_list: Vec<_> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let nodes = edge_list
        .iter()
        .flat_map(|(from, to)| [(*from, *to), (*to, *from)])
        .into_group_map();

    let mut queue: Vec<&str> = nodes.keys().copied().collect();
    let mut components: Vec<Vec<&str>> = Vec::new();

    while let Some(v) = queue.pop() {
        let neighbors = &nodes[v];

        if let Some((index, _)) = components
            .iter()
            .find_position(|e| e.iter().all(|a| neighbors.contains(a)))
        {
            components.get_mut(index).unwrap().push(v);
        } else {
            components.push(vec![v]);
        }
    }

    let largest_component = components.iter().max_by_key(|c| c.len()).unwrap();
    let lc_str = largest_component.into_iter().sorted().join(",");
    lc_str
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
        assert_eq!(output, "co,de,ka,ta");
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, "bw,dr,du,ha,mm,ov,pj,qh,tz,uv,vq,wq,xw");
    }
}
