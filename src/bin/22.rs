advent_of_code::solution!(22);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.unsigned()).collect()
}

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
        .map(|x| (0..2000).fold(x, |x, _| next_price(x)) as u64)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    #[inline]
    fn to_index(a: u32, b: u32, c: u32, d: u32) -> usize {
        const K_A: u32 = 19 * 19 * 19;
        const K_B: u32 = 19 * 19;
        const K_C: u32 = 19;

        (a * K_A + b * K_B + c * K_C + d) as usize
    }

    let mut ids = vec![usize::MAX; 19 * 19 * 19 * 19];
    let mut best_results = vec![0; 19 * 19 * 19 * 19];

    for (i, secret_number) in data.into_iter().enumerate() {
        let mut secret_number = secret_number;
        let mut prev_cost = secret_number % 10;

        let mut a;
        let (mut b, mut c, mut d) = (0, 0, 0);

        for n in 0..2000 {
            secret_number = next_price(secret_number);
            let cost = secret_number % 10;

            (a, b, c, d) = (b, c, d, cost + 9 - prev_cost);

            if n >= 4 {
                let index = to_index(a, b, c, d);

                if ids[index] != i {
                    best_results[index] += cost;
                    ids[index] = i;
                }
            }

            prev_cost = cost;
        }
    }

    let result = best_results.into_iter().max().unwrap();

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
