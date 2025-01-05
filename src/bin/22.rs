advent_of_code::solution!(22);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.unsigned()).collect()
}

const MASK_8_BITS: u32 = (1 << 8) - 1;
const MASK_24_BITS: u32 = (1 << 24) - 1;

#[allow(clippy::let_and_return)]
fn next_price(secret_number: u32) -> u32 {
    let secret_number = secret_number << 6 & MASK_24_BITS ^ secret_number;
    let secret_number = secret_number >> 5 ^ secret_number;
    let secret_number = secret_number << 11 & MASK_24_BITS ^ secret_number;

    secret_number
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|secret_number| {
            (0..2000).fold(secret_number, |secret_number, _| next_price(secret_number)) as u64
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut prices = FastMap::new();

    for (i, secret_number) in data.into_iter().enumerate() {
        let mut secret_number = secret_number;
        let mut prev_cost = secret_number % 10;

        let mut four_changes = 0;

        for n in 0..2000 {
            secret_number = next_price(secret_number);
            let cost = secret_number % 10;

            four_changes <<= 8;
            four_changes |= cost.wrapping_sub(prev_cost) & MASK_8_BITS;

            if n >= 4 {
                prices.entry((i, four_changes)).or_insert(cost);
            }

            prev_cost = cost;
        }
    }

    let mut best_results = FastMap::with_capacity(prices.len());
    for ((_, four_changes), cost) in prices.into_iter() {
        *best_results.entry(four_changes).or_insert(0) += cost;
    }

    let result = best_results.into_values().max().unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(&input);
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(23));
    }
}
