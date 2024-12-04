advent_of_code::solution!(4);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    const MAS: [u8; 3] = [b'M', b'A', b'S'];

    let validate = |x, y, dx, dy| -> bool {
        if !data.contains(Point::new(x + 3 * dx, y + 3 * dy)) {
            return false;
        }

        let a = [
            data[Point::new(x + dx, y + dy)],
            data[Point::new(x + 2 * dx, y + 2 * dy)],
            data[Point::new(x + 3 * dx, y + 3 * dy)],
        ];

        a == MAS
    };

    let mut result = 0;
    for x in 0..data.width {
        for y in 0..data.height {
            if data[Point::new(x, y)] == b'X' {
                result += if validate(x, y, 1, 0) { 1 } else { 0 };
                result += if validate(x, y, -1, 0) { 1 } else { 0 };
                result += if validate(x, y, 0, 1) { 1 } else { 0 };
                result += if validate(x, y, 0, -1) { 1 } else { 0 };

                result += if validate(x, y, 1, 1) { 1 } else { 0 };
                result += if validate(x, y, 1, -1) { 1 } else { 0 };
                result += if validate(x, y, -1, 1) { 1 } else { 0 };
                result += if validate(x, y, -1, -1) { 1 } else { 0 };
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    const MAS_LIST: [[u8; 4]; 4] = [
        [b'M', b'M', b'S', b'S'],
        [b'M', b'S', b'M', b'S'],
        [b'S', b'S', b'M', b'M'],
        [b'S', b'M', b'S', b'M'],
    ];

    let mut result = 0;
    for x in 1..data.width - 1 {
        for y in 1..data.height - 1 {
            if data[Point::new(x, y)] == b'A' {
                let a = [
                    data[Point::new(x - 1, y + 1)],
                    data[Point::new(x + 1, y + 1)],
                    data[Point::new(x - 1, y - 1)],
                    data[Point::new(x + 1, y - 1)],
                ];

                if MAS_LIST.contains(&a) {
                    result += 1;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
