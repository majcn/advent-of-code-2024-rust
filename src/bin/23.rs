advent_of_code::solution!(23);

use advent_of_code::maneatingape::hash::*;

use std::collections::BTreeSet;

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

pub fn part_two(input: &str) -> Option<String> {
    let nodes = parse_data(input);

    let mut cache = FastSet::new();

    let mut keys = nodes.keys().copied().collect::<Vec<_>>();
    keys.sort_by(|a, b| nodes[b].len().cmp(&nodes[a].len()));

    let to_check = keys;
    let group = BTreeSet::new();
    let mut queue = vec![(to_check, group)];
    while let Some((c, g)) = queue.pop() {
        for i in 0..c.len() {
            let mut is_ok = true;
            for el in g.iter() {
                if !nodes[el].contains(&c[i]) {
                    is_ok = false;
                    break;
                }
            }

            if is_ok {
                let mut new_g = g.clone();
                new_g.insert(c[i]);

                if cache.contains(&new_g) {
                    continue;
                }

                cache.insert(new_g.clone());

                let mut new_c = c.clone();
                new_c.remove(i);

                queue.push((new_c, new_g));
            }
        }
    }

    let mut result = cache
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .iter()
        .map(|&x| Computer::decode(x))
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
