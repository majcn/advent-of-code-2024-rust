advent_of_code::solution!(16);

use advent_of_code::majcn::direction::*;

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::heap::*;
use advent_of_code::maneatingape::point::*;

struct Block {}

impl Block {
    const WALL: u8 = b'#';
    const START: u8 = b'S';
    const FINISH: u8 = b'E';
}

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn dijkstra_g_score(grid: Grid<u8>, start_position: Point, end_position: Point) -> Grid<[u32; 4]> {
    let start_direction = RIGHT;

    let mut min_heap = MinHeap::new();
    let mut g_score = grid.same_size_with([u32::MAX; 4]);

    min_heap.push(0, (start_position, start_direction));
    g_score[start_position][direction_to_index(start_direction)] = 0;

    while let Some((cost, (position, direction))) = min_heap.pop() {
        if position == end_position {
            return g_score;
        }

        let neighbors = [
            (position + direction, direction, cost + 1),
            (position, direction.clockwise(), cost + 1000),
            (position, direction.counter_clockwise(), cost + 1000),
        ];

        for (n_position, n_direction, n_cost) in neighbors {
            if grid[n_position] == Block::WALL {
                continue;
            }

            let n_direction_index = direction_to_index(n_direction);
            if n_cost < g_score[n_position][n_direction_index] {
                g_score[n_position][n_direction_index] = n_cost;
                min_heap.push(n_cost, (n_position, n_direction));
            }
        }
    }

    g_score
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let start_position = grid.find(Block::START).unwrap();
    let end_position = grid.find(Block::FINISH).unwrap();

    let result = dijkstra_g_score(grid, start_position, end_position)[end_position]
        .into_iter()
        .min()
        .unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let start_position = grid.find(Block::START).unwrap();
    let end_position = grid.find(Block::FINISH).unwrap();

    let mut result = grid.same_size_with(false);

    let mut g_score = dijkstra_g_score(grid, start_position, end_position);
    let best = *g_score[end_position].iter().min().unwrap();

    let mut queue = vec![];
    queue.extend(
        g_score[end_position]
            .iter()
            .enumerate()
            .filter(|(_, &p)| p == best)
            .map(|(i, _)| (best, (end_position, index_to_direction(i)))),
    );

    while let Some((cost, (position, direction))) = queue.pop() {
        result[position] = true;

        if position == start_position {
            continue;
        }

        let neighbors = match cost {
            0 => vec![],
            1..1000 => vec![(position - direction, direction, cost - 1)],
            _ => vec![
                (position - direction, direction, cost - 1),
                (position, direction.clockwise(), cost - 1000),
                (position, direction.counter_clockwise(), cost - 1000),
            ],
        };

        for (n_position, n_direction, n_cost) in neighbors {
            let n_direction_index = direction_to_index(n_direction);
            if n_cost == g_score[n_position][n_direction_index] {
                queue.push((n_cost, (n_position, n_direction)));
                g_score[n_position][n_direction_index] = u32::MAX;
            }
        }
    }

    let result = result.bytes.into_iter().filter(|&x| x).count() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(64));
    }
}
