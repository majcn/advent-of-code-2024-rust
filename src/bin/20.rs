advent_of_code::solution!(20);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::parse::ParseOps;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> (Grid<u8>, usize) {
    const DEFAULT_LIMIT: usize = 100;

    let default_right = format!("{DEFAULT_LIMIT}",);
    let (left, right) = input.split_once("\n\n").unwrap_or((input, &default_right));

    let grid = Grid::parse(left);
    let limit = right.unsigned();

    (grid, limit)
}

fn find_path(grid: Grid<u8>, start_position: Point) -> Vec<Point> {
    let mut result = vec![];
    let mut next_position = Some(start_position);

    while let Some(position) = next_position {
        result.push(position);

        next_position = ORTHOGONAL.into_iter().map(|d| position + d).find(|&p| {
            (result.len() < 2 || result[result.len() - 2] != p)
                && grid.contains(p)
                && grid[p] != b'#'
        });
    }

    result
}

fn part_x<const N: usize>(grid: Grid<u8>, limit: usize) -> u32 {
    let start_location = grid.find(b'S').unwrap();

    let path = find_path(grid, start_location);
    let normal_cost = path.len();

    let mut result = 0;

    for (i1, &p1) in path.iter().enumerate() {
        for (i2, &p2) in path.iter().skip(i1 + limit).rev().enumerate() {
            let point_distance = p1.manhattan(p2) as usize;
            if point_distance <= N {
                let cheated_cost = i1 + point_distance + i2;
                if normal_cost - cheated_cost >= limit {
                    result += 1;
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, limit) = parse_data(input);

    let result = part_x::<2>(grid, limit);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, limit) = parse_data(input);

    let result = part_x::<20>(grid, limit);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(285));
    }
}
