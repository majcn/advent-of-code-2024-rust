advent_of_code::solution!(7);

use advent_of_code_macros::memoize;

use advent_of_code::majcn::math::*;

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.iter_unsigned().collect())
        .collect()
}

#[derive(Hash, Eq, PartialEq)]
struct SolveCacheKey {
    result: u64,
    left: u64,
    right: String,
}

fn solve_cache_key(result: u64, left: u64, right: &[u64]) -> SolveCacheKey {
    let right_str = right
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("_");

    SolveCacheKey {
        result,
        left,
        right: right_str,
    }
}

#[memoize(key_function = "solve_cache_key -> SolveCacheKey")]
fn solver(result: u64, left: u64, right: &[u64]) -> bool {
    solve(result, left, right, false)
}

#[memoize(key_function = "solve_cache_key -> SolveCacheKey")]
fn solver_with_concatenation(result: u64, left: u64, right: &[u64]) -> bool {
    solve(result, left, right, true)
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
        .filter(|line| {
            if use_concatenation {
                solver_with_concatenation(line[0], line[1], &line[2..])
            } else {
                solver(line[0], line[1], &line[2..])
            }
        })
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
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(11387));
    }
}
