advent_of_code::solution!(13);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

struct Machine {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    prize_x: u64,
    prize_y: u64,
}

fn parse_data(input: &str) -> Vec<Machine> {
    input
        .iter_unsigned()
        .chunk::<6>()
        .map(|[a_x, a_y, b_x, b_y, prize_x, prize_y]| Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        })
        .collect()
}

fn calculate_press(machine: Machine) -> Option<(u64, u64)> {
    let b = calculate_press_b(&machine)?;
    let a = calculate_press_a(&machine, b)?;

    Some((a, b))
}

fn calculate_press_b(machine: &Machine) -> Option<u64> {
    let top = (machine.prize_y * machine.a_x) as i64 - (machine.prize_x * machine.a_y) as i64;
    let bottom = (machine.b_y * machine.a_x) as i64 - (machine.a_y * machine.b_x) as i64;

    if top % bottom == 0 {
        Some((top / bottom) as u64)
    } else {
        None
    }
}

fn calculate_press_a(machine: &Machine, press_b: u64) -> Option<u64> {
    let top = machine.prize_y - machine.b_y * press_b;
    let bottom = machine.a_y;

    if top % bottom == 0 {
        Some(top / bottom)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .filter_map(calculate_press)
        .filter(|&(a, b)| a <= 100 && b <= 100)
        .fold(0, |acc, (a, b)| acc + a * 3 + b);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|mut machine| {
            machine.prize_x += 10000000000000;
            machine.prize_y += 10000000000000;
            machine
        })
        .filter_map(calculate_press)
        .fold(0, |acc, (a, b)| acc + a * 3 + b);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(875318608908));
    }
}
