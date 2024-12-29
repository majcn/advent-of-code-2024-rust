advent_of_code::solution!(19);

use advent_of_code::maneatingape::hash::*;

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

fn count_valid_message(cache: &mut FastMap<String, u64>, message: &str, towels: &[&str]) -> u64 {
    if message.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(message) {
        return result;
    }

    towels
        .iter()
        .filter(|towel| message.len() >= towel.len())
        .filter(|towel| &&&message[..towel.len()] == towel)
        .map(|towel| {
            let result = count_valid_message(cache, &message[towel.len()..], towels);
            cache.insert(String::from(&message[towel.len()..]), result);
            result
        })
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

    let mut cache = FastMap::new();

    let result = messages
        .into_iter()
        .map(|message| count_valid_message(&mut cache, message, &towels))
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
