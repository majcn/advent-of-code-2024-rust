advent_of_code::solution!(11);

use advent_of_code::majcn::math::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

fn calculate_step(v: u64) -> [Option<u64>; 2] {
    if v == 0 {
        return [Some(1), None];
    }

    let n = v.count_digits() as u32;
    if n % 2 == 0 {
        let split_value = 10_u64.pow(n / 2);
        return [Some(v / split_value), Some(v % split_value)];
    }

    [Some(v * 2024), None]
}

fn calculate<const N: usize>(v: u64, i: usize, cache: &mut FastMap<(usize, u64), u64>) -> u64 {
    if i == N {
        return 1;
    }

    if let Some(cached_result) = cache.get(&(i, v)) {
        return *cached_result;
    }

    let result = calculate_step(v)
        .into_iter()
        .filter_map(|x| Some(calculate::<N>(x?, i + 1, cache)))
        .sum();

    cache.insert((i, v), result);

    result
}

fn part_x<const N: usize>(data: Vec<u32>) -> u64 {
    let mut cache = FastMap::new();

    data.into_iter()
        .map(|x| calculate::<N>(x as u64, 0, &mut cache))
        .sum()
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
