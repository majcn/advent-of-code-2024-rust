advent_of_code::solution!(3);

enum Command {
    Enable,
    Disable,
    Multiply(u32, u32),
}

fn parse_data(input: &str) -> Vec<Command> {
    (0..input.len())
        .filter_map(|i| {
            let value = &input[i..];

            // Enable
            if value.starts_with("do()") {
                return Some(Command::Enable);
            }

            // Disable
            if value.starts_with("don't()") {
                return Some(Command::Disable);
            }

            // Multiply
            if value.starts_with("mul(") {
                let mut first_number = None;
                let mut number = 0;

                for c in value.bytes().skip(4) {
                    match c {
                        b'0'..=b'9' => {
                            if number > 1000 {
                                return None;
                            }

                            number = number * 10 + (c - b'0') as u32;
                        }
                        b',' => {
                            if number == 0 || first_number.is_some() {
                                return None;
                            }

                            first_number = Some(number);
                            number = 0;
                        }
                        b')' => {
                            if number == 0 {
                                return None;
                            }

                            return Some(Command::Multiply(first_number?, number));
                        }
                        _ => {
                            return None;
                        }
                    }
                }
            }

            None
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|c| match c {
            Command::Multiply(x, y) => x * y,
            _ => 0,
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .fold((0, true), |acc, c| match (c, acc.1) {
            (Command::Enable, false) => (acc.0, true),
            (Command::Disable, true) => (acc.0, false),
            (Command::Multiply(x, y), true) => (acc.0 + x * y, true),
            _ => acc,
        })
        .0;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(48));
    }
}
