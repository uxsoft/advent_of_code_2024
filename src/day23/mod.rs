use std::collections::HashMap;

use chumsky::input;
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph};

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> (Graph<&str, ()>, HashMap<&str, NodeIndex>) {
    let mut graph = Graph::new();

    let edge_list: Vec<_> = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let node_index: HashMap<_, _> = edge_list
        .iter()
        .flat_map(|(from, to)| [from, to])
        .unique()
        .map(|n| (*n, graph.add_node(*n)))
        .collect();

    for (from, to) in &edge_list {
        graph.add_edge(node_index[from], node_index[to], ());
    }

    (graph, node_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
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

        let (g, _n) = parse(input);
        assert_eq!(g.edge_count(), input.lines().count());
    }
}
