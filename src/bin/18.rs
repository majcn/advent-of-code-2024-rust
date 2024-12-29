advent_of_code::solution!(18);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;
use advent_of_code::maneatingape::point::*;

#[derive(Clone)]
enum Block {
    Ok,
    Corrupted,
}

fn parse_data(input: &str) -> (Vec<Point>, i32, i32, usize) {
    const DEFAULT_WIDTH: usize = 71;
    const DEFAULT_HEIGHT: usize = 71;
    const DEFAULT_FIRST_TAKE: usize = 1024;

    let default_right = format!("{DEFAULT_WIDTH},{DEFAULT_HEIGHT},{DEFAULT_FIRST_TAKE}",);
    let (left, right) = input.split_once("\n\n").unwrap_or((input, &default_right));

    let data = left
        .iter_signed()
        .chunk::<2>()
        .map(|[x, y]| Point::new(x, y))
        .collect();

    let [width, height, first_take] = right.iter_signed().chunk::<3>().next().unwrap();

    (data, width, height, first_take as usize)
}

fn find_shortest_path_cost(grid: &Grid<Block>) -> Option<u32> {
    let start_position = Point::new(0, 0);
    let end_position = Point::new(grid.width - 1, grid.height - 1);

    let mut queue = std::collections::VecDeque::new();
    let mut seen = Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![false; (grid.width * grid.height) as usize],
    };

    queue.push_front((start_position, 0));
    seen[start_position] = true;

    while let Some((position, cost)) = queue.pop_front() {
        if position == end_position {
            return Some(cost);
        }

        for n_position in ORTHOGONAL.map(|o| position + o) {
            if !grid.contains(n_position) || matches!(grid[n_position], Block::Corrupted) {
                continue;
            }

            if !seen[n_position] {
                queue.push_back((n_position, cost + 1));
                seen[n_position] = true;
            }
        }
    }

    None
}

fn generate_grid(data: &[Point], width: i32, height: i32, n: usize) -> Grid<Block> {
    let mut grid = Grid {
        width,
        height,
        bytes: vec![Block::Ok; (width * height) as usize],
    };

    data.iter().take(n).for_each(|&point| {
        grid[point] = Block::Corrupted;
    });

    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let (data, width, height, first_take) = parse_data(input);

    let result = find_shortest_path_cost(&generate_grid(&data, width, height, first_take))?;

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (data, width, height, first_take) = parse_data(input);

    let mut a = first_take;
    let mut b = data.len();
    while (b - a) > 1 {
        let c = (a + b) / 2;

        if find_shortest_path_cost(&generate_grid(&data, width, height, c)).is_some() {
            a = c;
        } else {
            b = c;
        }
    }

    let result = format!("{},{}", data[a].x, data[a].y);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(String::from("6,1")));
    }
}
