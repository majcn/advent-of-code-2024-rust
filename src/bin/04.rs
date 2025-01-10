advent_of_code::solution!(4);

use advent_of_code::majcn::grid::*;

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    const MAS: [u8; 3] = [b'M', b'A', b'S'];

    let validate = |p, d| -> bool {
        if !grid.contains(p + d * 3) {
            return false;
        }

        MAS == [grid[p + d], grid[p + d * 2], grid[p + d * 3]]
    };

    let result = grid
        .points()
        .filter(|&p| grid[p] == b'X')
        .map(|p| DIAGONAL.into_iter().filter(|&d| validate(p, d)).count() as u32)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    const MAS_LIST: [[u8; 4]; 4] = [
        [b'M', b'M', b'S', b'S'],
        [b'M', b'S', b'M', b'S'],
        [b'S', b'S', b'M', b'M'],
        [b'S', b'M', b'S', b'M'],
    ];

    const MAS_DIRECTIONS: [Point; 4] = [
        Point::new(-1, 1),
        Point::new(1, 1),
        Point::new(-1, -1),
        Point::new(1, -1),
    ];

    let result = grid
        .points()
        .filter(|p| p.x >= 1 && p.x < grid.width - 1 && p.y >= 1 && p.y < grid.height - 1)
        .filter(|&p| grid[p] == b'A')
        .filter(|&p| MAS_LIST.contains(&MAS_DIRECTIONS.map(|d| grid[p + d])))
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(9));
    }
}
