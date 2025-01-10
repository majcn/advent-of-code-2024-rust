advent_of_code::solution!(8);

use advent_of_code::majcn::grid::*;

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;

struct Block {}

impl Block {
    const EMPTY: u8 = b'.';
}

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn part_x(grid: Grid<u8>, unlimited: bool) -> u32 {
    let mut data = FastMap::new();
    grid.points()
        .filter(|&p| grid[p] != Block::EMPTY)
        .for_each(|p| data.entry(grid[p]).or_insert(vec![]).push(p));

    let mut antinodes = grid.same_size_with(false);

    for v in data.into_values() {
        for &p1 in &v {
            for &p2 in &v {
                if p1 == p2 {
                    continue;
                }

                let diff = p2 - p1;
                if unlimited {
                    let mut new_point = p2;
                    while grid.contains(new_point) {
                        antinodes[new_point] = true;
                        new_point += diff;
                    }
                } else {
                    let new_point = p2 + diff;
                    if grid.contains(new_point) {
                        antinodes[new_point] = true;
                    }
                }
            }
        }
    }

    antinodes.bytes.into_iter().filter(|&x| x).count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(data, false);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(data, true);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(34));
    }
}
