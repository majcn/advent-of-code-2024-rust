advent_of_code::solution!(21);

use advent_of_code::maneatingape::heap::*;
use advent_of_code::maneatingape::parse::*;
use advent_of_code_macros::memoize;

fn parse_data(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.unsigned()).collect()
}

const ACTION_DIRECTIONAL: usize = 4;
const ACTION_NUMERIC: usize = 10;

// [0]: LEFT, [1]: RIGHT, [2]: UP, [3]: DOWN
const DIRECTIONS: [usize; 4] = [0, 1, 2, 3];

const NUMERIC_GRID: [[Option<usize>; 4]; 11] = {
    const A: usize = ACTION_NUMERIC;

    [
        [None, Some(A), Some(2), None],
        [None, Some(2), Some(4), None],
        [Some(1), Some(3), Some(5), Some(0)],
        [Some(2), None, Some(6), Some(A)],
        [None, Some(5), Some(7), Some(1)],
        [Some(4), Some(6), Some(8), Some(2)],
        [Some(5), None, Some(9), Some(3)],
        [None, Some(8), None, Some(4)],
        [Some(7), Some(9), None, Some(5)],
        [Some(8), None, None, Some(6)],
        [Some(0), None, Some(3), None],
    ]
};

const DIRECTIONAL_GRID: [[Option<usize>; 4]; 5] = {
    const L: usize = 0;
    const R: usize = 1;
    const U: usize = 2;
    const D: usize = 3;
    const A: usize = ACTION_DIRECTIONAL;

    [
        [None, Some(D), None, None],
        [Some(D), None, Some(A), None],
        [None, Some(A), None, Some(D)],
        [Some(L), Some(R), Some(U), None],
        [Some(U), None, None, Some(R)],
    ]
};

fn keypad_to_keypad<FNextPos, FNextStep>(
    start: usize,
    end: usize,
    next_pos_f: FNextPos,
    nexf_f: FNextStep,
) -> u64
where
    FNextPos: Fn(usize, usize) -> Option<usize>,
    FNextStep: Fn(usize, usize) -> u64,
{
    let mut queue = MinHeap::new();
    queue.push(0, (start, ACTION_DIRECTIONAL, false));

    while let Some((cost, (position, lower_position, pressed))) = queue.pop() {
        if position == end && !pressed {
            let move_cost = nexf_f(lower_position, ACTION_DIRECTIONAL);
            queue.push(cost + move_cost, (position, lower_position, true));
            continue;
        }

        if position == end && pressed {
            return cost;
        }

        for direction in DIRECTIONS {
            if let Some(new_position) = next_pos_f(position, direction) {
                let move_cost = nexf_f(lower_position, direction);
                queue.push(cost + move_cost, (new_position, direction, false));
            }
        }
    }

    unreachable!()
}

#[memoize]
fn directional_to_numeric(start_numeric: usize, end_numeric: usize, n: usize) -> u64 {
    let next_pos_f = |pos: usize, d: usize| NUMERIC_GRID[pos][d];
    let next_f = |lower_pos: usize, d: usize| directional_to_directional(lower_pos, d, n - 1);

    keypad_to_keypad(start_numeric, end_numeric, next_pos_f, next_f)
}

#[memoize]
fn directional_to_directional(start_directional: usize, end_directional: usize, n: usize) -> u64 {
    let next_pos_f = |pos: usize, d: usize| DIRECTIONAL_GRID[pos][d];
    let next_f: Box<dyn Fn(usize, usize) -> u64> = match n {
        0 => Box::new(|_: usize, _: usize| 1),
        _ => Box::new(|lower_pos: usize, d: usize| directional_to_directional(lower_pos, d, n - 1)),
    };

    keypad_to_keypad(start_directional, end_directional, next_pos_f, next_f)
}

fn part_x<const N: usize>(data: Vec<u32>) -> u64 {
    let flow = |code| {
        let d1 = code as usize / 100;
        let d2 = (code as usize / 10) % 10;
        let d3 = code as usize % 10;

        [ACTION_NUMERIC, d1, d2, d3, ACTION_NUMERIC]
    };

    let calculate_cost = |code| {
        flow(code)
            .windows(2)
            .map(|w| directional_to_numeric(w[0], w[1], N))
            .sum::<u64>()
    };

    data.into_iter()
        .map(|code| code as u64 * calculate_cost(code))
        .sum::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<2>(data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x::<25>(data);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(154115708116294));
    }
}
