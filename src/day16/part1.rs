use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    super::shared::{coordinate::Coordinate, direction::Direction},
    neighbors, Node,
};

fn shortest_path_dijsktra(
    spaces: &HashSet<Coordinate>,
    start: Node,
) -> (HashMap<Node, usize>, HashMap<Node, Node>) {
    let mut dist: HashMap<Node, usize> = HashMap::new();
    let mut prev: HashMap<Node, Node> = HashMap::new();
    let mut queue = VecDeque::new();

    dist.insert(start, 0);
    queue.push_back(start);

    //         for each vertex v in Graph.Vertices:
    //             dist[v] ← INFINITY
    //             prev[v] ← UNDEFINED

    while let Some(u) = queue.pop_front() {
        for (cost, v) in neighbors(spaces, u) {
            let alt = dist.get(&u).unwrap_or(&usize::MAX) + cost;
            if alt < *dist.get(&v).unwrap_or(&usize::MAX) {
                dist.insert(v, alt);
                prev.insert(v, u);
                queue.push_back(v);
            }
        }
    }

    //
    //         while Q is not empty:
    //             u ← vertex in Q with minimum dist[u]
    //             remove u from Q
    //
    //             for each neighbor v of u still in Q:
    //                 alt ← dist[u] + Graph.Edges(u, v)
    //                 if alt < dist[v]:
    //                     dist[v] ← alt
    //                     prev[v] ← u
    //
    //         return dist[], prev[]

    (dist, prev)
}

fn reconstruct_path<'a>(prev: &'a HashMap<Node, Node>, end: &'a Node) -> VecDeque<&'a Node> {
    let mut path = VecDeque::new();
    let mut u = end;
    path.push_back(u);

    while let Some(v) = prev.get(u) {
        u = v;
        path.push_back(v);
    }

    path
}

pub fn process(input: &str) -> usize {
    let (start, end, mut spaces) = super::parse(input);

    let start_node = Node {
        pos: start,
        dir: Direction::East,
    };

    spaces.insert(end);

    let (distances, prev) = shortest_path_dijsktra(&spaces, start_node);

    let (end_node, distance) = distances
        .iter()
        .filter(|(n, _)| n.pos.x == end.x && n.pos.y == end.y)
        .min_by_key(|(_, c)| *c)
        .expect("Expected to find a path to the end");

    let path = reconstruct_path(&prev, &end_node);

    // dbg!(&distances);
    for y in 0..15 {
        for x in 0..15 {
            let has_distance = distances.keys().any(|n| n.pos.x == x && n.pos.y == y);
            let is_path = path.iter().any(|n| n.pos.x == x && n.pos.y == y);
            print!(
                "{}",
                match (is_path, has_distance) {
                    (true, _) => 'x',
                    (false, true) => '.',
                    (false, false) => '#',
                }
            )
        }
        println!("")
    }

    *distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let score = process(input);
        assert_eq!(7036, score);
    }

    #[test]
    fn second_example() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let score = process(input);
        assert_eq!(11048, score);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let score = process(input);
        assert_eq!(108504, score);
    }
}
