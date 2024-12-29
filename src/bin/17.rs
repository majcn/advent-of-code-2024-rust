advent_of_code::solution!(17);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> (u64, Vec<u8>) {
    let (left, right) = input.split_once("\n\n").unwrap();

    (left.unsigned(), right.iter_unsigned().collect())
}

fn run_program(a: u64) -> Vec<u8> {
    let mut result = vec![];

    let mut a = a;
    while a > 0 {
        let b = (a ^ 1) & 7;
        result.push((b ^ 4 ^ (a >> b) & 7) as u8);

        a >>= 3;
    }

    result
}

pub fn part_one(input: &str) -> Option<String> {
    let (a, _) = parse_data(input);

    let program_result = run_program(a);

    let mut result = String::with_capacity(program_result.len() * 2);
    for x in program_result {
        result.push((b'0' + x) as char);
        result.push(',');
    }
    result.pop();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, program) = parse_data(input);

    let mut result = u64::MAX;

    let mut queue = vec![];
    queue.push((0, program.len() - 1));

    while let Some((a, i)) = queue.pop() {
        let p_i = program[i] as u64;

        for part_a in 0..=7 {
            let n_a = (a << 3) | (p_i ^ part_a ^ 5) << (part_a ^ 1) | part_a;

            if i > 0 && run_program(n_a) == program[i..] {
                queue.push((n_a, i - 1));
                continue;
            }

            if i == 0 && run_program(n_a) == program {
                result = result.min(n_a);
            }
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
        assert_eq!(result, Some(String::from("5,0,4,5")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(188468));
    }
}
