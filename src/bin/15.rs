advent_of_code::solution!(15);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

struct Block {}

impl Block {
    const WALL: u8 = b'#';
    const EMPTY: u8 = b'.';
    const CRATE: u8 = b'O';
    const ROBOT: u8 = b'@';
}

fn parse_data(input: &str) -> (Grid<u8>, Vec<Point>) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let grid = Grid::parse(left);
    let instructions = right
        .bytes()
        .filter(|x| !x.is_ascii_whitespace())
        .map(Point::from)
        .collect();

    (grid, instructions)
}

fn part_x<F>(mut grid: Grid<u8>, instructions: Vec<Point>, can_move_f: F) -> u32
where
    F: Fn(&Grid<u8>, Point, Point, &mut Vec<Point>) -> bool,
{
    let mut robot_position = grid.find(Block::ROBOT).unwrap();

    let mut affected_crates = vec![];
    for direction in instructions {
        if can_move_f(&grid, robot_position, direction, &mut affected_crates) {
            robot_position += direction;

            affected_crates.drain(..).rev().for_each(|c| {
                grid[c] = Block::EMPTY;
                grid[c + direction] = Block::CRATE;
            });
        }
    }

    let mut result = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            if grid[p] == Block::CRATE {
                result += y as u32 * 100 + x as u32;
            }
        }
    }

    result
}

fn can_move_part_one(
    grid: &Grid<u8>,
    position: Point,
    direction: Point,
    affected_crates: &mut Vec<Point>,
) -> bool {
    let next_position = position + direction;

    if grid[next_position] == Block::WALL {
        affected_crates.clear();
        return false;
    }

    if grid[next_position] == Block::CRATE {
        affected_crates.push(next_position);
        return can_move_part_one(grid, next_position, direction, affected_crates);
    }

    true
}

fn can_move_part_two(
    grid: &Grid<u8>,
    position: Point,
    direction: Point,
    affected_crates: &mut Vec<Point>,
) -> bool {
    let next_position = position + direction;

    if grid[next_position] == Block::WALL {
        affected_crates.clear();
        return false;
    }

    if direction == UP || direction == DOWN {
        for next_crate_position in [next_position, next_position + LEFT] {
            if grid[next_crate_position] == Block::CRATE {
                affected_crates.push(next_crate_position);
                if !can_move_part_two_vertical(
                    grid,
                    next_crate_position,
                    direction,
                    affected_crates,
                ) {
                    return false;
                }
            }
        }
    }

    if direction == LEFT || direction == RIGHT {
        let next_crate_position = match direction {
            LEFT => next_position + LEFT,
            RIGHT => next_position,
            _ => unreachable!(),
        };

        if grid[next_crate_position] == Block::CRATE {
            affected_crates.push(next_crate_position);
            return can_move_part_two_horizontal(
                grid,
                next_crate_position,
                direction,
                affected_crates,
            );
        }
    }

    true
}

fn can_move_part_two_vertical(
    grid: &Grid<u8>,
    position: Point,
    direction: Point,
    affected_crates: &mut Vec<Point>,
) -> bool {
    let next_position = position + direction;
    let next_position_l = next_position + LEFT;
    let next_position_r = next_position + RIGHT;

    if grid[next_position] == Block::WALL || grid[next_position_r] == Block::WALL {
        affected_crates.clear();
        return false;
    }

    for next_crate_position in [next_position_l, next_position, next_position_r] {
        if grid[next_crate_position] == Block::CRATE {
            affected_crates.push(next_crate_position);
            if !can_move_part_two_vertical(grid, next_crate_position, direction, affected_crates) {
                return false;
            }
        }
    }

    true
}

fn can_move_part_two_horizontal(
    grid: &Grid<u8>,
    position: Point,
    direction: Point,
    affected_crates: &mut Vec<Point>,
) -> bool {
    let next_position = position + direction;
    let next_next_position = next_position + direction;

    if direction == LEFT && grid[next_position] == Block::WALL {
        affected_crates.clear();
        return false;
    }

    if direction == RIGHT && grid[next_next_position] == Block::WALL {
        affected_crates.clear();
        return false;
    }

    if grid[next_next_position] == Block::CRATE {
        affected_crates.push(next_next_position);
        return can_move_part_two_horizontal(grid, next_next_position, direction, affected_crates);
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, instructions) = parse_data(input);

    let result = part_x(grid, instructions, can_move_part_one);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, instructions) = parse_data(input);

    let mut big_grid_data = Vec::with_capacity(grid.height as usize * grid.width as usize * 2);
    for el in grid.bytes {
        match el {
            Block::WALL => big_grid_data.extend([Block::WALL, Block::WALL]),
            Block::EMPTY => big_grid_data.extend([Block::EMPTY, Block::EMPTY]),
            Block::CRATE => big_grid_data.extend([Block::CRATE, Block::EMPTY]),
            Block::ROBOT => big_grid_data.extend([Block::ROBOT, Block::EMPTY]),
            _ => unreachable!(),
        }
    }

    let grid = Grid {
        width: grid.width * 2,
        height: grid.height,
        bytes: big_grid_data,
    };

    let result = part_x(grid, instructions, can_move_part_two);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(9021));
    }
}
