advent_of_code::solution!(6);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

type Direction = Point;

struct MapSize {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

struct DataType {
    position: Point,
    direction: Direction,
    obstructions: FastSet<Point>,
    map_size: MapSize,
}

impl MapSize {
    fn contains(&self, point: &Point) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }
}

fn parse_data(input: &str) -> DataType {
    let my_direction = UP;
    let mut my_position = Point::new(0, 0);
    let mut obstructions = FastSet::new();

    let height = input.lines().count();
    let width = input.split_once("\n").unwrap().0.len();

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.bytes().enumerate() {
            match v {
                b'#' => {
                    obstructions.insert(Point::new(x as i32, y as i32));
                }
                b'^' => {
                    my_position = Point::new(x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    DataType {
        position: my_position,
        direction: my_direction,
        obstructions,
        map_size: MapSize {
            min_x: 0,
            max_x: width as i32 - 1,
            min_y: 0,
            max_y: height as i32 - 1,
        },
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let data: DataType = parse_data(input);

    let mut my_position = data.position;
    let mut my_direction = data.direction;

    let mut visit = FastSet::new();

    loop {
        visit.insert(my_position);

        let next_position = my_position + my_direction;
        if !data.map_size.contains(&next_position) {
            break;
        }

        if data.obstructions.contains(&next_position) {
            my_direction = my_direction.clockwise();
        } else {
            my_position = next_position;
        }
    }

    let result = visit.len();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: DataType = parse_data(input);

    let mut result = 0;

    for y in data.map_size.min_y..=data.map_size.max_y {
        for x in data.map_size.min_x..=data.map_size.max_x {
            let new_obstruction = Point::new(x, y);

            if data.position == new_obstruction || data.obstructions.contains(&new_obstruction) {
                continue;
            }

            let mut visit: FastSet<(Point, Point)> = FastSet::new();

            let mut my_position = data.position;
            let mut my_direction = data.direction;

            let stuck = loop {
                if !visit.insert((my_position, my_direction)) {
                    break true;
                }

                let next_position = my_position + my_direction;
                if !data.map_size.contains(&next_position) {
                    break false;
                }

                if new_obstruction == next_position || data.obstructions.contains(&next_position) {
                    my_direction = my_direction.clockwise();
                } else {
                    my_position = next_position;
                }
            };

            if stuck {
                result += 1;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
