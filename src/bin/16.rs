advent_of_code::solution!(16);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::heap::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn neighbors(
    grid: &Grid<u8>,
    position: Point,
    direction: Point,
    cost: u32,
) -> Vec<(Point, Point, u32)> {
    let mut result = Vec::with_capacity(4);

    for n_direction in [LEFT, RIGHT, UP, DOWN] {
        let n_position = position + n_direction;

        if grid[n_position] != b'#' {
            let n_cost_diff = match (direction, n_direction) {
                (LEFT, LEFT) => 1,
                (LEFT, UP) => 1001,
                (LEFT, DOWN) => 1001,
                (LEFT, RIGHT) => 2001,

                (RIGHT, RIGHT) => 1,
                (RIGHT, UP) => 1001,
                (RIGHT, DOWN) => 1001,
                (RIGHT, LEFT) => 2001,

                (UP, UP) => 1,
                (UP, LEFT) => 1001,
                (UP, RIGHT) => 1001,
                (UP, DOWN) => 2001,

                (DOWN, DOWN) => 1,
                (DOWN, LEFT) => 1001,
                (DOWN, RIGHT) => 1001,
                (DOWN, UP) => 2001,

                _ => panic!("Invalid state"),
            };

            result.push((n_position, n_direction, cost + n_cost_diff));
        }
    }

    result
}

fn find_shortest_path_cost(grid: Grid<u8>) -> u32 {
    let start_position = grid.find(b'S').unwrap();
    let start_direction = RIGHT;

    let mut min_heap = MinHeap::new();
    let mut g_score = FastMap::new();

    min_heap.push(0, (start_position, start_direction));
    g_score.insert((start_position, start_direction), 0);

    while let Some((cost, (position, direction))) = min_heap.pop() {
        if grid[position] == b'E' {
            return cost;
        }

        for (n_position, n_direction, n_cost) in neighbors(&grid, position, direction, cost) {
            if n_cost < *g_score.get(&(n_position, n_direction)).unwrap_or(&u32::MAX) {
                g_score.insert((n_position, n_direction), n_cost);
                min_heap.push(n_cost, (n_position, n_direction));
            }
        }
    }

    u32::MAX
}

fn find_all_shortest_paths_points(grid: Grid<u8>) -> FastSet<Point> {
    let start_position = grid.find(b'S').unwrap();
    let start_direction = RIGHT;
    let start_path = vec![start_position];

    let mut min_heap = MinHeap::new();
    let mut g_score = FastMap::new();

    min_heap.push(0, (start_position, start_direction, start_path));
    g_score.insert((start_position, start_direction), 0);

    let mut first_winner_cost = u32::MAX;
    let mut all_shortest_paths_points = FastSet::new();

    while let Some((cost, (position, direction, path))) = min_heap.pop() {
        if grid[position] == b'E' {
            if cost > first_winner_cost {
                return all_shortest_paths_points;
            } else {
                first_winner_cost = cost;
                all_shortest_paths_points.extend(path.iter());
            }
        }

        for (n_position, n_direction, n_cost) in neighbors(&grid, position, direction, cost) {
            if n_cost <= *g_score.get(&(n_position, n_direction)).unwrap_or(&u32::MAX) {
                g_score.insert((n_position, n_direction), n_cost);

                let mut n_path = Vec::with_capacity(path.len() + 1);
                n_path.extend(path.iter());
                n_path.push(n_position);

                min_heap.push(n_cost, (n_position, n_direction, n_path));
            }
        }
    }

    all_shortest_paths_points
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_shortest_path_cost(grid);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_all_shortest_paths_points(grid).len() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
