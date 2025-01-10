advent_of_code::solution!(5);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> ([[bool; 100]; 100], Vec<Vec<usize>>) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let mut rules = [[false; 100]; 100];
    left.iter_unsigned::<usize>()
        .chunk::<2>()
        .for_each(|[key, value]| {
            rules[key][value] = true;
        });

    let updates = right
        .lines()
        .map(|line| line.iter_unsigned().collect())
        .collect();

    (rules, updates)
}

fn validate(rules: &[[bool; 100]; 100], update: &[usize]) -> bool {
    !update
        .iter()
        .enumerate()
        .any(|(i, &x)| update[..i].iter().any(|&b| rules[x][b]))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_data(input);

    let result = updates
        .iter()
        .filter(|update| validate(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum::<usize>() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut updates) = parse_data(input);

    let update_cmp_f = |&a: &usize, &b: &usize| {
        if rules[a][b] {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    };

    let result = updates
        .iter_mut()
        .filter(|update| !validate(&rules, update))
        .map(|update| (update.len(), update))
        .map(|(l, update)| *update.select_nth_unstable_by(l / 2, update_cmp_f).1)
        .sum::<usize>() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(123));
    }
}
