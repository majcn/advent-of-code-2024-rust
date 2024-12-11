advent_of_code::solution!(7);

use advent_of_code::majcn::math::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.iter_unsigned().collect())
        .collect()
}

fn solve(result: u64, left: u64, right: &[u64], use_concatenation: bool) -> bool {
    if right.is_empty() {
        return result == left;
    }

    if left > result {
        return false;
    }

    let first_left_value = left + right[0];
    if solve(result, first_left_value, &right[1..], use_concatenation) {
        return true;
    }

    let second_left_value = left * right[0];
    if solve(result, second_left_value, &right[1..], use_concatenation) {
        return true;
    }

    if use_concatenation {
        let third_left_number = left * 10_u64.pow(right[0].count_digits() as u32) + right[0];
        if solve(result, third_left_number, &right[1..], use_concatenation) {
            return true;
        }
    }

    false
}

fn part_x(data: Vec<Vec<u64>>, use_concatenation: bool) -> u64 {
    data.into_iter()
        .filter(|line| solve(line[0], line[1], &line[2..], use_concatenation))
        .map(|line| line[0])
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x(data, false);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
