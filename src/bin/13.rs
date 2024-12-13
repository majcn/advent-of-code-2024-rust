advent_of_code::solution!(13);

use advent_of_code::maneatingape::parse::*;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_data(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|x| {
            let mut x_iter = x.iter_signed();

            Machine {
                a: Point::new(x_iter.next().unwrap(), x_iter.next().unwrap()),
                b: Point::new(x_iter.next().unwrap(), x_iter.next().unwrap()),
                prize: Point::new(x_iter.next().unwrap(), x_iter.next().unwrap()),
            }
        })
        .collect()
}

fn calculate_press(machine: Machine) -> Option<(u64, u64)> {
    if let Some(b) = calculate_press_b(&machine) {
        if let Some(a) = calculate_press_a(&machine, b) {
            return Some((a, b));
        }
    }

    None
}

fn calculate_press_b(machine: &Machine) -> Option<u64> {
    let top = machine.prize.y * machine.a.x - machine.prize.x * machine.a.y;
    let bottom = machine.b.y * machine.a.x - machine.a.y * machine.b.x;

    if top % bottom == 0 {
        Some((top / bottom) as u64)
    } else {
        None
    }
}

fn calculate_press_a(machine: &Machine, press_b: u64) -> Option<u64> {
    let top = machine.prize.y - machine.b.y * press_b as i64;
    let bottom = machine.a.y;

    if top % bottom == 0 {
        Some((top / bottom) as u64)
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
            machine.prize.x += 10000000000000;
            machine.prize.y += 10000000000000;
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
