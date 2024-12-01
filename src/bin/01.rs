advent_of_code::solution!(1);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut iter = line.iter_unsigned::<u32>();
            (iter.next().unwrap(), iter.next().unwrap())
        })
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

    let mut right_counter = FastMap::new();
    for item in right {
        *right_counter.entry(item).or_default() += 1;
    }

    let result = left
        .into_iter()
        .map(|x| match right_counter.get(&x) {
            Some(v) => x * v,
            None => 0,
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
