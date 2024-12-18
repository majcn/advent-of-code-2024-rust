advent_of_code::solution!(18);

use std::collections::VecDeque;

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;
use advent_of_code::maneatingape::point::*;

#[derive(Clone)]
enum Block {
    Ok,
    Corrupted,
}

fn parse_data(input: &str) -> (Vec<Point>, i32, i32, usize) {
    let default_right = format!("{},{},{}", 71, 71, 1024);
    let (left, right) = input.split_once("\n\n").unwrap_or((input, &default_right));

    let data = left
        .iter_signed()
        .chunk::<2>()
        .map(|[x, y]| Point::new(x, y))
        .collect();

    let [width, height, first_take] = right.iter_signed().chunk::<3>().next().unwrap();

    (data, width, height, first_take as usize)
}

fn neighbors(grid: &Grid<Block>, position: Point, cost: u32) -> Vec<(Point, u32)> {
    let mut result = Vec::with_capacity(4);

    for direction in [LEFT, RIGHT, UP, DOWN] {
        let n_position = position + direction;

        if grid.contains(n_position) && matches!(grid[n_position], Block::Ok) {
            result.push((n_position, cost + 1));
        }
    }

    result
}

fn find_shortest_path_cost(grid: &Grid<Block>) -> u32 {
    let start_position = Point::new(0, 0);
    let end_position = Point::new(grid.width - 1, grid.height - 1);

    let mut queue = VecDeque::new();
    let mut g_score = FastMap::new();

    queue.push_front((0, start_position));
    g_score.insert(start_position, 0);

    while let Some((cost, position)) = queue.pop_front() {
        if position == end_position {
            return cost;
        }

        for (n_position, n_cost) in neighbors(grid, position, cost) {
            if n_cost < *g_score.get(&n_position).unwrap_or(&u32::MAX) {
                g_score.insert(n_position, n_cost);
                queue.push_back((n_cost, n_position));
            }
        }
    }

    u32::MAX
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

    let result = find_shortest_path_cost(&generate_grid(&data, width, height, first_take));

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (data, width, height, first_take) = parse_data(input);

    let mut a = first_take;
    let mut b = input.lines().count();
    while (b - a) > 1 {
        let c = (a + b) / 2;

        let result = find_shortest_path_cost(&generate_grid(&data, width, height, c));
        if result == u32::MAX {
            b = c;
        } else {
            a = c;
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
