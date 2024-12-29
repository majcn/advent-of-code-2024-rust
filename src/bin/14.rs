advent_of_code::solution!(14);

use advent_of_code::majcn::math::*;
use advent_of_code::maneatingape::hash::*;
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

    let mut robots = robots;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.x = (robot.position.x + robot.velocity.x).modulo(width);
            robot.position.y = (robot.position.y + robot.velocity.y).modulo(height);
        }
    }

    let width_center_start = width / 2 + 1;
    let width_center_end = width / 2 - 1;
    let height_center_start = height / 2 + 1;
    let height_center_end = height / 2 - 1;

    let mut quadrants = [0; 4];
    for robot in robots {
        if (0..=width_center_end).contains(&robot.position.x)
            && (0..=height_center_end).contains(&robot.position.y)
        {
            quadrants[0] += 1;
        }

        if (0..=width_center_end).contains(&robot.position.x)
            && (height_center_start..height).contains(&robot.position.y)
        {
            quadrants[1] += 1;
        }

        if (width_center_start..width).contains(&robot.position.x)
            && (0..=height_center_end).contains(&robot.position.y)
        {
            quadrants[2] += 1;
        }

        if (width_center_start..width).contains(&robot.position.x)
            && (height_center_start..height).contains(&robot.position.y)
        {
            quadrants[3] += 1;
        }
    }

    let result = quadrants.into_iter().product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (robots, width, height) = parse_data(input);

    let mut robots = robots;

    let mut time = 0;
    'outer: loop {
        for robot in robots.iter_mut() {
            robot.position.x = (robot.position.x + robot.velocity.x).modulo(width);
            robot.position.y = (robot.position.y + robot.velocity.y).modulo(height);
        }

        time += 1;

        let mut robots_set = FastSet::with_capacity(robots.len());
        for robot in robots.iter() {
            if !robots_set.insert(robot.position) {
                continue 'outer;
            }
        }

        return Some(time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
