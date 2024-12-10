advent_of_code::solution!(10);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn part_x(grid: &Grid<u8>) -> FastMap<(Point, Point), u32> {
    let mut start_positions = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[Point::new(x, y)] == b'0' {
                start_positions.push(Point::new(x, y));
            }
        }
    }

    let mut result = FastMap::new();
    for start_position in start_positions {
        let mut paths = vec![(b'0', start_position)];

        while let Some((height, location)) = paths.pop() {
            if height == b'9' {
                *result.entry((start_position, location)).or_insert(0) += 1;
                continue;
            }

            for step in [LEFT, RIGHT, UP, DOWN] {
                let next_location = location + step;
                if grid.contains(next_location) && grid[next_location] == height + 1 {
                    paths.push((height + 1, next_location));
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = part_x(&grid).len() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = part_x(&grid).values().sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
