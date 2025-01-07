advent_of_code::solution!(11);

use advent_of_code_macros::memoize;

use advent_of_code::majcn::math::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

fn calculate_step(v: u64) -> Vec<u64> {
    if v == 0 {
        return vec![1];
    }

    let n = v.count_digits() as u32;
    if n % 2 == 0 {
        let split_value = 10_u64.pow(n / 2);
        return vec![v / split_value, v % split_value];
    }

    vec![v * 2024]
}

#[memoize]
fn calculate(v: u64, n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    calculate_step(v)
        .into_iter()
        .map(|x| calculate(x, n - 1))
        .sum()
}

fn part_x<const N: usize>(data: Vec<u32>) -> u64 {
    data.into_iter().map(|x| calculate(x as u64, N)).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<25>(data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<75>(data);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
