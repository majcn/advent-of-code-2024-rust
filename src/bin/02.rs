advent_of_code::solution!(2);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.iter_signed().collect())
        .collect()
}

fn is_valid_pair(x: i32, y: i32, inc: bool) -> bool {
    let diff = if inc { y - x } else { x - y };
    (1..=3).contains(&diff)
}

fn find_broken_position<'a, I>(mut iter: I) -> Option<usize>
where
    I: Iterator<Item = &'a i32>,
{
    let first = *iter.next()?;
    let second = *iter.next()?;
    let inc = second > first;

    if !is_valid_pair(first, second, inc) {
        return Some(0);
    }

    let mut x = second;
    for (i, &next_x) in iter.enumerate() {
        if !is_valid_pair(x, next_x, inc) {
            return Some(i + 1);
        }

        x = next_x;
    }

    None
}

fn find_broken_position_with_dampener(line: &[i32]) -> Option<usize> {
    let broken_position = find_broken_position(line.iter())?;

    let left = &line[..broken_position];
    let right = &line[broken_position + 1..];
    find_broken_position(left.iter().chain(right.iter()))?;

    if broken_position > 0 {
        let left = &line[..broken_position - 1];
        let right = &line[broken_position..];
        find_broken_position(left.iter().chain(right.iter()))?;
    }

    if broken_position < line.len() {
        let left = &line[..broken_position + 1];
        let right = &line[broken_position + 2..];
        find_broken_position(left.iter().chain(right.iter()))?;
    }

    Some(broken_position)
}

fn part_x(data: Vec<Vec<i32>>, use_dampener: bool) -> u32 {
    let result = data
        .iter()
        .filter_map(|line| {
            if use_dampener {
                find_broken_position_with_dampener(line)
            } else {
                find_broken_position(line.iter())
            }
        })
        .count();

    (data.len() - result) as u32
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
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(4));
    }
}
