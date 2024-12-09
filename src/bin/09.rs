use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let first_line = input.lines().next().unwrap();
    let mut file_system: Vec<Option<u64>> = Vec::new();
    let mut free_space_indexes: VecDeque<usize> = VecDeque::new();
    first_line
        .chars()
        .map(|char| char.to_digit(10).expect("All characters should be digits (0-9)"))
        .enumerate()
        .for_each(|(index, disk_map_number)| {
            if index % 2 == 0 {
                let file_number = (index / 2) as u64;
                (0..disk_map_number).for_each(|_| file_system.push(Some(file_number)));
            } else {
                let first_free_index = file_system.len();
                (0..disk_map_number).for_each(|index_offset| {
                    free_space_indexes.push_back(first_free_index + index_offset as usize);
                    file_system.push(None);
                });
            }
        });
    while free_space_indexes.len() > 0 {
        let last_block = file_system.pop().unwrap();

        let block_to_allocate = match last_block {
            None => {
                free_space_indexes.pop_back().unwrap();
                continue;
            },
            Some(block) => block
        };
        let free_block_index = free_space_indexes.pop_front().unwrap();
        file_system[free_block_index] = Some(block_to_allocate);
    }
    let checksum = file_system.iter().enumerate().fold(0, |acc, (index, file_number)| {
        acc + (index as u64 * file_number.unwrap())
    });
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let first_line = input.lines().next().unwrap();
    let mut file_system: Vec<Option<u64>> = Vec::new();
    let mut free_space_indexes: Vec<(usize, usize)> = Vec::new();
    let mut file_indexes: Vec<(usize, usize, u64)> = Vec::new();
    first_line
        .chars()
        .map(|char| char.to_digit(10).expect("All characters should be digits (0-9)"))
        .enumerate()
        .for_each(|(index, disk_map_number)| {
            if index % 2 == 0 {
                let file_number = (index / 2) as u64;
                let first_file_block_index = file_system.len();
                file_indexes.push((first_file_block_index, disk_map_number as usize, file_number));
                (0..disk_map_number).for_each(|_| file_system.push(Some(file_number)));
            } else {
                let first_free_index = file_system.len();
                free_space_indexes.push((first_free_index, disk_map_number as usize));
                (0..disk_map_number).for_each(|_| file_system.push(None));
            }
        });
    while file_indexes.len() > 0 {
        let (file_system_index, file_size, file_number) = file_indexes.pop().unwrap();
        let free_space = free_space_indexes.iter()
            .enumerate()
            .filter(|&(_, (free_space_file_system_index, _))| *free_space_file_system_index < file_system_index)
            .find(|(_, (_, free_space_size))| *free_space_size >= file_size);

        let (free_space_index, (free_space_file_system_index, free_space_size)) = match free_space {
            None => continue,
            Some(free_space) => free_space
        };

        (0..file_size).for_each(|file_block_index| {
            file_system[free_space_file_system_index + file_block_index] = Some(file_number);
            file_system[file_system_index + file_block_index] = None;
        });

        if *free_space_size == file_size {
            free_space_indexes.remove(free_space_index);
        } else {
            free_space_indexes[free_space_index] = (free_space_file_system_index + file_size, free_space_size - file_size);
        }
    }
    let checksum = file_system.iter()
        .enumerate()
        .fold(0, |acc, (index, file_number)| {
        acc + (index as u64 * file_number.unwrap_or(0))
    });
    Some(checksum)
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
