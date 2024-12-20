use std::fmt::Display;

use super::super::shared::{coordinate::Coordinate, direction::Direction, grid::Grid};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Robot,
    Empty,
    LBox,
    RBox,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Robot => write!(f, "@"),
            Cell::Empty => write!(f, "."),
            Cell::LBox => write!(f, "["),
            Cell::RBox => write!(f, "]"),
        }
    }
}

pub fn parse(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let grid_str = grid_str
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");

    let grid = Grid::parse(&grid_str, |c| match c {
        '#' => Cell::Wall,
        '@' => Cell::Robot,
        '[' => Cell::LBox,
        ']' => Cell::RBox,
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

fn can_cascade_move_to(grid: &Grid<Cell>, current_pos: &Coordinate, direction: &Direction) -> bool {
    use Cell::*;
    use Direction::*;
    // [@][O][.]
    //  c  n
    if let Some(next_pos) = direction.advance_bounded(current_pos, grid.width(), grid.height()) {
        let next_contents = grid.get(&next_pos);

        let can_move = match next_contents {
            Wall => false,
            Robot | Empty => true,
            LBox if direction == &North || direction == &South => {
                let left_pos = next_pos;
                let right_pos = East.advance(&next_pos).unwrap();

                let can_left = can_cascade_move_to(grid, &left_pos, direction);
                let can_right = can_cascade_move_to(grid, &right_pos, direction);

                can_left && can_right
            }
            RBox if direction == &North || direction == &South => {
                let left_pos = West.advance(&next_pos).unwrap();
                let right_pos = next_pos;

                let can_left = can_cascade_move_to(grid, &left_pos, direction);
                let can_right = can_cascade_move_to(grid, &right_pos, direction);

                can_left && can_right
            }
            _box_from_side => can_cascade_move_to(grid, &next_pos, direction),
        };

        return can_move;
    }

    return false;
}

/// Moves value at current_pos to next_pos in direction
fn cascade_move(grid: &mut Grid<Cell>, current_pos: &Coordinate, direction: &Direction) {
    use Cell::*;
    use Direction::*;

    let current_contents = *grid.get(current_pos);

    if let Some(next_pos) = direction.advance(current_pos) {
        let next_contents = *grid.get(&next_pos);

        // Optionally push next_contents further before writing
        match next_contents {
            LBox if direction == &North || direction == &South => {
                cascade_move(grid, &next_pos, direction);
                cascade_move(grid, &East.advance(&next_pos).unwrap(), direction);
            }
            RBox if direction == &North || direction == &South => {
                cascade_move(grid, &West.advance(&next_pos).unwrap(), direction);
                cascade_move(grid, &next_pos, direction);
            }
            LBox | RBox => {
                cascade_move(grid, &next_pos, direction);
            }
            _ => (),
        }

        // Do the move
        grid.set(current_pos, Empty);
        grid.set(&next_pos, current_contents);
    } else {
        println!(
            "Tried to cascade_move out of bounds! {:?} {:?}",
            current_pos, direction
        );
        // println!("{}", grid);
    }
}

pub fn process(input: &str) -> usize {
    use Cell::*;

    let (mut grid, moves) = parse(input);
    let mut robot_pos = grid.find(|c| *c == Robot).unwrap();

    for direction in moves.iter() {
        if can_cascade_move_to(&grid, &robot_pos, &direction) {
            cascade_move(&mut grid, &robot_pos, &direction);
            robot_pos = direction
                .advance_bounded(&robot_pos, grid.width(), grid.height())
                .unwrap()
        } else {
            // println!("Can't move {:?}", direction);
        }

        // println!("{}", grid);
        // println!("");
        // println!("");
    }

    let score = grid
        .rows
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| match c {
                    &LBox => 100 * y + x,
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
        assert_eq!(9021, score);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let score = process(input);
        assert_eq!(1521453, score);
    }
}
