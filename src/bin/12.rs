advent_of_code::solution!(12);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

struct Garden {
    area: u32,
    perimeter: u32,
    edges: FastSet<(Point, Point)>,
}

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn find_gardens(grid: &Grid<u8>) -> Vec<Garden> {
    let mut visited = grid.same_size_with(false);

    let mut gardens = vec![];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let start_location = Point::new(x, y);
            if visited[start_location] {
                continue;
            }

            let plot = grid[start_location];

            let mut area = 0;
            let mut perimeter = 0;
            let mut edges = FastSet::new();

            let mut queue = vec![start_location];
            while let Some(position) = queue.pop() {
                if visited[position] {
                    continue;
                }
                visited[position] = true;

                area += 1;
                perimeter += 4;

                for direction in ORTHOGONAL {
                    let new_position = position + direction;
                    if grid.contains(new_position) && grid[new_position] == plot {
                        queue.push(new_position);
                        perimeter -= 1;
                    } else {
                        edges.insert((position, direction));
                    }
                }
            }

            gardens.push(Garden {
                area,
                perimeter,
                edges,
            });
        }
    }

    gardens
}

fn find_sides(mut edges: FastSet<(Point, Point)>) -> u32 {
    let next_corner_edge = |edges: &FastSet<(Point, Point)>| {
        let mut edge = edges.iter().next().copied()?;
        loop {
            let new_edge = (edge.0 + edge.1.clockwise(), edge.1);
            if !edges.contains(&new_edge) {
                return Some(edge);
            }

            edge = new_edge;
        }
    };

    let mut sides = 0;
    let mut next_edge = next_corner_edge(&edges);
    while let Some(edge @ (p, d)) = next_edge {
        edges.remove(&edge);

        let left_edge = (p + d.counter_clockwise(), d);
        if edges.contains(&left_edge) {
            next_edge = Some(left_edge);
            continue;
        }

        let right_edge = (p + d.clockwise(), d);
        if edges.contains(&right_edge) {
            next_edge = Some(right_edge);
            continue;
        }

        next_edge = next_corner_edge(&edges);
        sides += 1;
    }

    sides
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_gardens(&grid)
        .into_iter()
        .map(|garden| garden.area * garden.perimeter)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_gardens(&grid)
        .into_iter()
        .map(|garden| garden.area * find_sides(garden.edges))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1206));
    }
}
