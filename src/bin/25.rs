advent_of_code::solution!(25);

fn parse_data(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n\n")
        .map(|s| s.bytes().filter(|c| !c.is_ascii_whitespace()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematics = parse_data(input);

    let mut locks_and_keys = [
        Vec::with_capacity(schematics.len()),
        Vec::with_capacity(schematics.len()),
    ];

    for schematic in schematics {
        let locks_and_keys_index = if schematic[0] == b'#' { 0 } else { 1 };

        let mut pins = [0; 5];
        schematic
            .into_iter()
            .enumerate()
            .for_each(|(i, c)| pins[i % 5] += if c == b'#' { 1 } else { 0 });

        locks_and_keys[locks_and_keys_index].push(pins);
    }

    let [locks, keys] = locks_and_keys;

    let result = locks
        .into_iter()
        .flat_map(|lock| std::iter::repeat(lock).zip(keys.iter()))
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 7))
        .count() as u32;

    Some(result)
}

pub fn part_two(_input: &str) -> Option<String> {
    // "Thank you Eric for another wonderful year of AoC!"
    Some(String::from("⭐️⭐️"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(String::from("⭐️⭐️")));
    }
}
