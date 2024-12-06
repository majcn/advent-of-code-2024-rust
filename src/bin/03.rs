advent_of_code::solution!(3);

use advent_of_code::maneatingape::parse::*;

use regex::Regex;

enum Command {
    Enable,
    Disable,
    Multiply(u32, u32),
}

fn parse_data(input: &str) -> Vec<Command> {
    let mut commands = vec![];

    let mul_re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for i in 0..input.len() {
        if input[i..].starts_with("mul") {
            if let Some(m) = mul_re.captures(&input[i..]) {
                let [x, y] = m.extract().1.map(|x| x.unsigned());
                commands.push(Command::Multiply(x, y));
            }
        } else if input[i..].starts_with("do()") {
            commands.push(Command::Enable);
        } else if input[i..].starts_with("don't()") {
            commands.push(Command::Disable);
        }
    }

    commands
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

    let mut enabled = true;

    let mut result = 0;
    for c in data {
        match c {
            Command::Multiply(x, y) => result += if enabled { x * y } else { 0 },
            Command::Enable => enabled = true,
            Command::Disable => enabled = false,
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
