advent_of_code::solution!(23);

use std::iter::once;

use advent_of_code::maneatingape::hash::*;

struct Computer {}

impl Computer {
    fn encode(name: &str) -> usize {
        let mut chars = name.chars();
        let first = chars.next().unwrap() as u8 - b'a';
        let second = chars.next().unwrap() as u8 - b'a';

        first as usize * 26 + second as usize
    }

    fn decode(id: usize) -> String {
        let first = (id / 26) as u8 + b'a';
        let second = (id % 26) as u8 + b'a';

        format!("{}{}", first as char, second as char)
    }
}

fn parse_data(input: &str) -> FastMap<usize, FastSet<usize>> {
    let mut nodes = FastMap::new();

    for line in input.lines() {
        let left = Computer::encode(&line[0..2]);
        let right = Computer::encode(&line[3..5]);

        nodes.entry(left).or_insert(FastSet::new()).insert(right);
        nodes.entry(right).or_insert(FastSet::new()).insert(left);
    }

    nodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let nodes = parse_data(input);

    let keys = nodes.keys().copied().collect::<Vec<_>>();

    let mut result = 0;

    let mut visited_t = FastSet::new();

    // TODO: poglej samo sosede
    // TODO: spremeni hashmap-e v arraye (hitrejsi dostop)
    for computer_i in Computer::encode("ta")..=Computer::encode("tz") {
        if !nodes.contains_key(&computer_i) {
            continue;
        }

        visited_t.insert(computer_i);

        for (j, computer_j) in keys.iter().enumerate() {
            if visited_t.contains(computer_j) {
                continue;
            }

            for computer_k in keys.iter().skip(j + 1) {
                if visited_t.contains(computer_k) {
                    continue;
                }

                if nodes[&computer_i].contains(computer_j)
                    && nodes[&computer_i].contains(computer_k)
                    && nodes[computer_j].contains(computer_k)
                {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

fn bors_kerbosch(
    r: Vec<usize>,
    p: FastSet<usize>,
    x: FastSet<usize>,
    g: &FastMap<usize, FastSet<usize>>,
) -> Vec<usize> {
    if p.is_empty() && x.is_empty() {
        return r;
    }

    let mut max_result = vec![];
    let mut p = p;
    let mut x = x;

    let pivot = p.union(&x).max_by_key(|v| g[v].len()).unwrap();

    for v in p.difference(&g[pivot]).copied().collect::<Vec<_>>() {
        let next_r = r.iter().chain(once(&v)).copied().collect::<Vec<_>>();
        let next_p = p.intersection(&g[&v]).copied().collect::<FastSet<_>>();
        let next_x = x.intersection(&g[&v]).copied().collect::<FastSet<_>>();

        let result = bors_kerbosch(next_r, next_p, next_x, g);
        if result.len() > max_result.len() {
            max_result = result;
        }

        p.remove(&v);
        x.insert(v);
    }

    max_result
}

pub fn part_two(input: &str) -> Option<String> {
    let nodes = parse_data(input);

    let keys = nodes.keys().copied().collect::<FastSet<_>>();
    let result = bors_kerbosch(vec![], keys, FastSet::new(), &nodes);

    let mut result = result.into_iter().map(Computer::decode).collect::<Vec<_>>();
    result.sort_unstable();

    let result = result.join(",");

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
