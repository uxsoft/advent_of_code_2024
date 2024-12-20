use std::fmt::Display;

use super::super::shared::{coordinate::Coordinate, direction::Direction, grid::Grid};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Robot,
    Empty,
    Box,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Robot => write!(f, "@"),
            Cell::Empty => write!(f, "."),
            Cell::Box => write!(f, "O"),
        }
    }
}

pub fn parse(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let grid = Grid::parse(grid_str, |c| match c {
        '#' => Cell::Wall,
        '@' => Cell::Robot,
        'O' => Cell::Box,
        _ => Cell::Empty,
    });

    let moves = moves_str
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            _ => None,
        })
        .collect();

    (grid, moves)
}

fn try_cascade_move(
    grid: &mut Grid<Cell>,
    current_pos: &Coordinate,
    direction: &Direction,
    value: Cell,
) -> Option<Coordinate> {
    use Cell::*;
    //[x][O][ ]
    //    c
    if let Some(next_pos) = direction.next_in_bounds(current_pos, grid.width(), grid.height()) {
        let contents = grid.get(&next_pos);

        let can_move: Option<Coordinate> = match contents {
            Wall => None,
            Robot | Empty => Some(next_pos),
            Box => try_cascade_move(grid, &next_pos, direction, *contents),
        };

        if let Some(m) = can_move {
            grid.set(&next_pos, value);
            return Some(next_pos);
        }
    }

    return None;
}

pub fn process(input: &str) -> usize {
    use Cell::*;

    let (mut grid, moves) = parse(input);
    let mut robot_pos = grid.find(|c| *c == Robot).unwrap();

    for direction in moves {
        if let Some(new_loc) = try_cascade_move(&mut grid, &robot_pos, &direction, Robot) {
            grid.set(&robot_pos, Cell::Empty);
            robot_pos = new_loc;
        }

        // println!("");
        // println!("{}", grid);
    }

    let score = grid
        .rows
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| match c {
                    &Box => 100 * y + x,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum();

    score
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn parse() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (g, m) = super::parse(input);
        assert_eq!(g.rows.len(), 10);
        assert_eq!(m.len(), 700);
    }

    #[test]
    fn smaller_example() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let score = process(input);
        assert_eq!(2028, score);
    }

    #[test]
    fn larger_example() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let score = process(input);
        assert_eq!(10092, score);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let score = process(input);
        assert_eq!(1509074, score);
    }
}
