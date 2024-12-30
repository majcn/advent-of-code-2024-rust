advent_of_code::solution!(12);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn find_data_with_flood(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut visited = grid.same_size_with(false);

    let mut result = vec![];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let start_location = Point::new(x, y);
            if visited[start_location] {
                continue;
            }

            let plot = grid[start_location];

            let mut area = 0;
            let mut perimeter = 0;

            let mut queue = vec![start_location];
            while let Some(position) = queue.pop() {
                if visited[position] {
                    continue;
                }
                visited[position] = true;

                area += 1;
                perimeter += 4;

                for new_position in ORTHOGONAL.map(|o| position + o) {
                    if grid.contains(new_position) && grid[new_position] == plot {
                        queue.push(new_position);
                        perimeter -= 1;
                    }
                }
            }

            result.push((area, perimeter));
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_data_with_flood(&grid)
        .into_iter()
        .map(|(area, perimeter)| (area * perimeter) as u32)
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
