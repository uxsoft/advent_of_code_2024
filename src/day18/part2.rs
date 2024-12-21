use std::collections::{HashMap, HashSet, VecDeque};

use super::super::shared::coordinate::Coordinate;
use super::super::shared::direction::Direction;

fn neighbors(
    obstacles: &HashSet<Coordinate>,
    from: &Coordinate,
    width: usize,
    height: usize,
) -> Vec<Coordinate> {
    Direction::all()
        .iter()
        .filter_map(|d| d.advance_bounded(from, width, height))
        .filter(|p| !obstacles.contains(p))
        .collect()
}

fn shortest_path_dijsktra(
    obstacles: &HashSet<Coordinate>,
    start: Coordinate,
    width: usize,
    height: usize,
) -> (HashMap<Coordinate, usize>, HashMap<Coordinate, Coordinate>) {
    let mut dist: HashMap<Coordinate, usize> = HashMap::new();
    let mut prev: HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut queue = VecDeque::new();

    dist.insert(start, 0);
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for v in neighbors(obstacles, &u, width, height) {
            let alt = dist.get(&u).unwrap_or(&usize::MAX) + 1;
            if alt < *dist.get(&v).unwrap_or(&usize::MAX) {
                dist.insert(v, alt);
                prev.insert(v, u);
                queue.push_back(v);
            }
        }
    }

    (dist, prev)
}

fn print(obstacles: &HashSet<Coordinate>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let s = match obstacles.contains(&Coordinate::new(x, y)) {
                true => '#',
                false => ' ',
            };

            print!("{}", s);
        }
        println!("");
    }
}

pub fn process(input: &str, end: Coordinate, take: usize) -> Coordinate {
    let obstacle_stream = super::parse(input);
    let mut obstacles: HashSet<_> = obstacle_stream.iter().take(take).copied().collect();
    let start = Coordinate::new(0, 0);

    for i in take..obstacle_stream.len() {
        obstacles.insert(obstacle_stream[i]);
        let (dist, _) = shortest_path_dijsktra(&obstacles, start, end.x + 1, end.y + 1);
        if !dist.contains_key(&end) {
            return obstacle_stream[i];
        }
    }

    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        let result = process(input, Coordinate::new(6, 6), 12);
        assert_eq!(result, Coordinate::new(6, 1));
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input, Coordinate::new(70, 70), 1024);
        assert_eq!(result, Coordinate::new(27, 60));
    }
}
