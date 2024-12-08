advent_of_code::solution!(8);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

struct MapSize {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

struct DataType {
    data: FastMap<u8, Vec<Point>>,
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
    let mut data: FastMap<u8, Vec<Point>> = FastMap::new();

    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c != b'.' {
                let p = Point::new(x as i32, y as i32);
                data.entry(c).or_default().push(p);
            }
        }
    }

    DataType {
        data,
        map_size: MapSize {
            min_x: 0,
            max_x: width - 1,
            min_y: 0,
            max_y: height - 1,
        },
    }
}

fn part_x(data: DataType, unlimited: bool) -> u32 {
    let DataType { data, map_size } = data;

    let mut antinodes = FastSet::new();
    for (_, v) in data {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }

                let p1 = v[i];
                let p2 = v[j];

                let diff = p2 - p1;
                if unlimited {
                    let mut new_point = p2;
                    while map_size.contains(&new_point) {
                        antinodes.insert(new_point);
                        new_point += diff;
                    }
                } else {
                    let new_point = p2 + diff;
                    if map_size.contains(&new_point) {
                        antinodes.insert(new_point);
                    }
                }
            }
        }
    }

    antinodes.len() as u32
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
