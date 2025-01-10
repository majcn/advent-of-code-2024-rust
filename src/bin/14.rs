advent_of_code::solution!(14);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;
use advent_of_code::maneatingape::point::*;

struct Robot {
    position: Point,
    velocity: Point,
}

fn parse_data(input: &str) -> (Vec<Robot>, i32, i32) {
    const DEFAULT_WIDTH: i32 = 101;
    const DEFAULT_HEIGHT: i32 = 103;

    let default_right = format!("{DEFAULT_WIDTH},{DEFAULT_HEIGHT}",);
    let (left, right) = input.split_once("\n\n").unwrap_or((input, &default_right));

    let robots = left
        .iter_signed()
        .chunk::<4>()
        .map(|[x, y, vx, vy]| Robot {
            position: Point::new(x, y),
            velocity: Point::new(vx, vy),
        })
        .collect();

    let [width, height] = right.iter_signed().chunk::<2>().next().unwrap();

    (robots, width, height)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (robots, width, height) = parse_data(input);

    const TIME: i32 = 100;

    let mut quadrants = [0; 4];
    for robot in robots {
        let x = (robot.position.x + TIME * robot.velocity.x).rem_euclid(width);
        let y = (robot.position.y + TIME * robot.velocity.y).rem_euclid(height);

        if x == width / 2 || y == height / 2 {
            continue;
        }

        let i = match (x < width / 2, y < height / 2) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };

        quadrants[i] += 1;
    }

    let result = quadrants.into_iter().product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (robots, width, height) = parse_data(input);

    let result = (0..width * height)
        .find(|&time| {
            let mut seen = vec![false; width as usize * height as usize];
            for robot in robots.iter() {
                let x = (robot.position.x + time * robot.velocity.x).rem_euclid(width);
                let y = (robot.position.y + time * robot.velocity.y).rem_euclid(height);
                let i = x as usize + y as usize * width as usize;

                if seen[i] {
                    return false;
                }

                seen[i] = true;
            }

            true
        })
        .unwrap() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(1));
    }
}
