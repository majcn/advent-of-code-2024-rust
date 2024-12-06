advent_of_code::solution!(5);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;
use std::cmp::Ordering;

fn parse_data(input: &str) -> (FastMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let mut rules: FastMap<u32, Vec<u32>> = FastMap::new();
    left.iter_unsigned().chunk::<2>().for_each(|[key, value]| {
        rules.entry(key).or_default().push(value);
    });

    let updates = right
        .lines()
        .map(|line| line.iter_unsigned().collect())
        .collect();

    (rules, updates)
}

fn validate(rules: &FastMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    update
        .iter()
        .enumerate()
        .filter_map(|(i, x)| rules.get(x)?.iter().find(|b| update[..i].contains(b)))
        .next()
        .is_none()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_data(input);

    let result = updates
        .iter()
        .filter(|update| validate(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut updates) = parse_data(input);

    let update_cmp_f = |a: &u32, b: &u32| match rules.get(a) {
        Some(r) => {
            if r.contains(b) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
        None => Ordering::Equal,
    };

    let result = updates
        .iter_mut()
        .filter(|update| !validate(&rules, update))
        .map(|update| {
            update.sort_unstable_by(update_cmp_f);
            update[update.len() / 2]
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
