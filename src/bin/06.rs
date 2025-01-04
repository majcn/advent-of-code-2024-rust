advent_of_code::solution!(6);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

struct Block {}

impl Block {
    const WALL: u8 = b'#';
    const GUARD: u8 = b'^';
}

fn parse_data(input: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(input);
    let start_position = grid.find(Block::GUARD).unwrap();

    (grid, start_position)
}

fn visited_positions(grid: &Grid<u8>, start_position: Point) -> Vec<Point> {
    let mut result = vec![];

    let mut my_position = start_position;
    let mut my_direction = UP;

    let mut visit = grid.same_size_with(false);

    loop {
        if !visit[my_position] {
            result.push(my_position);
        };

        visit[my_position] = true;

        let next_position = my_position + my_direction;
        if !grid.contains(next_position) {
            return result;
        }

        if grid[next_position] == Block::WALL {
            my_direction = my_direction.clockwise();
        } else {
            my_position = next_position;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start_position) = parse_data(input);

    let result = visited_positions(&grid, start_position).len() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start_position) = parse_data(input);

    let result = visited_positions(&grid, start_position)
        .into_iter()
        .filter_map(|new_obstruction| {
            let mut visit = grid.same_size_with([false, false, false, false]);

            let mut my_position = start_position;
            let mut my_direction = UP;

            loop {
                let visit_index = match my_direction {
                    UP => 0,
                    RIGHT => 1,
                    DOWN => 2,
                    LEFT => 3,
                    _ => unreachable!(),
                };

                if visit[my_position][visit_index] {
                    break Some(true);
                }

                visit[my_position][visit_index] = true;

                let next_position = my_position + my_direction;
                if !grid.contains(next_position) {
                    break None;
                }

                if new_obstruction == next_position || grid[next_position] == Block::WALL {
                    my_direction = my_direction.clockwise();
                } else {
                    my_position = next_position;
                }
            }
        })
        .count() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(6));
    }
}
