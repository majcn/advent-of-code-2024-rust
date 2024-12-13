advent_of_code::solution!(9);

use advent_of_code::majcn::math::*;

struct FileBlock {
    id: usize,
    size: usize,
    position: usize,
}

struct FreeBlock {
    size: usize,
    position: usize,
}

fn parse_data(input: &str) -> (Vec<FileBlock>, Vec<FreeBlock>) {
    let mut file_list = vec![];
    let mut free_list = vec![];

    let mut position = 0;

    for (i, x) in input.bytes().enumerate() {
        let size = (x - b'0') as usize;

        if i % 2 == 0 {
            file_list.push(FileBlock {
                id: i / 2,
                size,
                position,
            });
        } else if x != 0 {
            free_list.push(FreeBlock { size, position });
        }

        position += size;
    }

    (file_list, free_list)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (file_list, free_list) = parse_data(input);

    let mut result = 0;

    let mut free_index = 0;
    let mut free_offset = 0;

    for file in file_list.into_iter().rev() {
        let mut file_free_swap = 0;
        for file_offset in 0..file.size {
            while free_list[free_index].size <= free_offset {
                free_index += 1;
                free_offset = 0;
            }

            let free_position = free_list[free_index].position + free_offset;
            let position = if free_position < file.position {
                free_offset += 1;
                file_free_swap += 1;
                free_position
            } else {
                file.position + file_offset - file_free_swap
            };

            result += file.id * position;
        }
    }

    let result = result as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (file_list, mut free_list) = parse_data(input);

    let mut result = 0;

    for mut file in file_list.into_iter().rev() {
        let free_option = free_list
            .iter_mut()
            .take_while(|x| x.position < file.position)
            .find(|x| x.size >= file.size);

        if let Some(free) = free_option {
            file.position = free.position;
            free.position += file.size;
            free.size -= file.size;
        }

        result += file.id * (file.size * file.position + (file.size - 1).sum_of_natural_numbers());
    }

    let result = result as u64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
