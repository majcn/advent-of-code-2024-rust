advent_of_code::solution!(24);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

type Gate<'a> = (&'a str, Operator, &'a str, &'a str);

fn parse_data(input: &str) -> (Vec<bool>, Vec<Gate>) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let inputs = left.bytes().skip(5).step_by(7).map(|v| v == b'1').collect();

    let gates = right
        .split_ascii_whitespace()
        .chunk::<5>()
        .map(|[l, op, r, _, res]| {
            let operator = match op {
                "AND" => Operator::And,
                "OR" => Operator::Or,
                "XOR" => Operator::Xor,
                _ => unreachable!(),
            };

            (l, operator, r, res)
        })
        .collect();

    (inputs, gates)
}

fn find_max_z_number(gates: &[Gate]) -> usize {
    let z_count = gates
        .iter()
        .filter(|(_, _, _, res)| res.starts_with("z"))
        .count();

    z_count - 1
}

fn solve(
    input_x: &[bool],
    input_y: &[bool],
    gates_rev: &FastMap<&str, (&str, Operator, &str)>,
    name: &str,
) -> bool {
    if name.starts_with("x") {
        return input_x[name.unsigned::<usize>()];
    }

    if name.starts_with("y") {
        return input_y[name.unsigned::<usize>()];
    }

    let (left_name, operator, right_name) = &gates_rev[name];
    let left = solve(input_x, input_y, gates_rev, left_name);
    let right = solve(input_x, input_y, gates_rev, right_name);

    match operator {
        Operator::And => left & right,
        Operator::Or => left | right,
        Operator::Xor => left ^ right,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (inputs, gates) = parse_data(input);

    let max_z_number = find_max_z_number(&gates);

    let gates_rev = gates
        .into_iter()
        .map(|(l, o, r, res)| (res, (l, o, r)))
        .collect();

    let mut result = 0;

    let x = &inputs[..inputs.len() / 2];
    let y = &inputs[inputs.len() / 2..];
    for i in 0..=max_z_number {
        let z = solve(x, y, &gates_rev, &format!("z{:02}", i));
        result |= (z as u64) << i;
    }

    Some(result)
}

/*
   x_0 XOR y_0 = z_0
   x_0 AND y_0 = c_0
   for i in 1..z_max {
       x_i XOR y_i = s_i_0
       x_i AND y_i = c_i_0
       c_{i-1} XOR s_i_0 = z_i
       c_{i-1} AND s_i_0 = c_i_1
       c_i_0 OR c_i_1 = c_i
   }
*/
pub fn part_two(input: &str) -> Option<String> {
    let (_, gates) = parse_data(input);

    let input_with_operator = gates
        .iter()
        .flat_map(|(l, o, r, _)| [(*l, *o), (*r, *o)])
        .collect::<FastSet<_>>();

    let max_z_name = format!("z{:02}", find_max_z_number(&gates));

    let mut result = gates
        .into_iter()
        .filter(|&(input, operator, _, res)| {
            if input == "x00" || input == "y00" {
                if operator == Operator::Xor && res != "z00" {
                    return true;
                }

                if operator == Operator::And && res.starts_with("z") {
                    return true;
                }

                return false;
            }

            match operator {
                Operator::Xor => {
                    if input.starts_with("x") || input.starts_with("y") {
                        if !input_with_operator.contains(&(res, Operator::Xor)) {
                            return true;
                        }

                        if !input_with_operator.contains(&(res, Operator::And)) {
                            return true;
                        }
                    } else if !res.starts_with("z") {
                        return true;
                    }
                }
                Operator::And => {
                    if !input_with_operator.contains(&(res, Operator::Or)) {
                        return true;
                    }
                }
                Operator::Or => {
                    if res.starts_with("z") && res != max_z_name {
                        return true;
                    }

                    if input_with_operator.contains(&(res, Operator::Or)) {
                        return true;
                    }
                }
            }

            false
        })
        .map(|(_, _, _, res)| res)
        .collect::<Vec<_>>();

    result.sort_unstable();
    let result = result.join(",");

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(&input);
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(&input);
        assert_eq!(result, Some(String::from("e01,q01,q03,w01,z00,z03")));
    }
}
