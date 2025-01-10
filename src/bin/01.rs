advent_of_code::solution!(1);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .iter_unsigned::<u32>()
        .chunk::<2>()
        .map(|[l, r]| (l, r))
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_data(input);

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .into_iter()
        .zip(right)
        .map(|(x, y)| u32::abs_diff(x, y))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_data(input);

    let mut right_counter = FastMap::with_capacity(right.len());
    for item in right {
        *right_counter.entry(item).or_default() += 1;
    }

    let result = left
        .iter()
        .filter_map(|x| right_counter.get(x).map(|v| x * v))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(31));
    }
}
