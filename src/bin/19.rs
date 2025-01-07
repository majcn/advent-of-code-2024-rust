advent_of_code::solution!(19);

use advent_of_code_macros::memoize;

fn parse_data(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let towels = left.split(", ").collect();
    let messages = right.lines().collect();

    (towels, messages)
}

fn validate_message(message: &str, towels: &[&str]) -> bool {
    if message.is_empty() {
        return true;
    }

    towels
        .iter()
        .filter(|towel| message.len() >= towel.len())
        .filter(|towel| &&&message[..towel.len()] == towel)
        .any(|towel| validate_message(&message[towel.len()..], towels))
}

fn count_valid_message_cache_key(message: &str, _: &[&str]) -> String {
    String::from(message)
}

#[memoize(key_function = "count_valid_message_cache_key -> String")]
fn count_valid_message(message: &str, towels: &[&str]) -> u64 {
    if message.is_empty() {
        return 1;
    }

    towels
        .iter()
        .filter(|towel| message.len() >= towel.len())
        .filter(|towel| &&&message[..towel.len()] == towel)
        .map(|towel| count_valid_message(&message[towel.len()..], towels))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, messages) = parse_data(input);

    let result = messages
        .into_iter()
        .filter(|message| validate_message(message, &towels))
        .count() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, messages) = parse_data(input);

    let result = messages
        .into_iter()
        .map(|message| count_valid_message(message, &towels))
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let intput = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&intput);
        assert_eq!(result, Some(16));
    }
}
