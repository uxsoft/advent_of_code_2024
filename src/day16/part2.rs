use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    super::shared::{coordinate::Coordinate, direction::Direction},
    neighbors, Node,
};

fn shortest_path_dijsktra(
    spaces: &HashSet<Coordinate>,
    start: Node,
) -> (HashMap<Node, usize>, HashMap<Node, Vec<Node>>) {
    let mut dist: HashMap<Node, usize> = HashMap::new();
    let mut prev: HashMap<Node, Vec<Node>> = HashMap::new();
    let mut queue = VecDeque::new();

    dist.insert(start, 0);
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for (cost, v) in neighbors(spaces, u) {
            let alt = dist.get(&u).unwrap_or(&usize::MAX) + cost;
            if alt < *dist.get(&v).unwrap_or(&usize::MAX) {
                dist.insert(v, alt);
                queue.push_back(v);
                prev.insert(v, vec![u]);
            } else if alt == *dist.get(&v).unwrap_or(&usize::MAX) {
                queue.push_back(v);
                prev.entry(v).and_modify(|l| l.push(u)).or_insert(vec![u]);
            }
        }
    }

    (dist, prev)
}

fn reconstruct_paths<'a>(prev: &'a HashMap<Node, Vec<Node>>, end: &'a Node) -> HashSet<&'a Node> {
    let mut path = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(end);
    path.insert(end);

    while let Some(u) = queue.pop_front() {
        if let Some(vs) = prev.get(u) {
            for v in vs {
                if path.insert(v) {
                    queue.push_back(v);
                }
            }
        }
    }

    path
}

fn print(distances: &HashMap<Node, usize>, path: HashSet<Node>) {
    for y in 0..17 {
        for x in 0..17 {
            let has_distance = distances.keys().any(|n| n.pos.x == x && n.pos.y == y);
            let is_path = path.iter().any(|n| n.pos.x == x && n.pos.y == y);
            print!(
                "{}",
                match (is_path, has_distance) {
                    (true, _) => '.',
                    (false, true) => ' ',
                    (false, false) => '#',
                }
            )
        }
        println!("")
    }
}

pub fn process(input: &str) -> usize {
    let (start, end, mut spaces) = super::parse(input);

    let start_node = Node {
        pos: start,
        dir: Direction::East,
    };

    spaces.insert(start);
    spaces.insert(end);

    let (distances, prev) = shortest_path_dijsktra(&spaces, start_node);

    // TODO what if there are more directions from which to reach the end
    let (end_node, _) = distances
        .iter()
        .filter(|(n, _)| n.pos.x == end.x && n.pos.y == end.y)
        .min_by_key(|(_, c)| *c)
        .expect("Expected to find a path to the end");

    let path = reconstruct_paths(&prev, &end_node);

    let coords_on_path = path.iter().map(|n| n.pos).collect::<HashSet<_>>();

    coords_on_path.len()
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
        assert_eq!(45, score);
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
        assert_eq!(64, score);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let score = process(input);
        assert_eq!(538, score);
    }
}
