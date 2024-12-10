use std::collections::{HashSet};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let topographic_map = parse_topographic_map(input);

    Some(topographic_map
        .iter()
        .enumerate()
        .map(|(y, row)|
            row
                .iter()
                .enumerate()
                .filter(|&(_, &value)| value == 0)
                .map(|(x, _)| find_reachable_peaks(&topographic_map, y, x, Vec::new()))
                .map(|reachable_peaks| reachable_peaks.iter().collect::<HashSet<_>>().len() as u32)
                .sum::<u32>()
        )
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let topographic_map = parse_topographic_map(input);

    Some(topographic_map
        .iter()
        .enumerate()
        .map(|(y, row)|
            row
                .iter()
                .enumerate()
                .filter(|&(_, &value)| value == 0)
                .map(|(x, _)| find_reachable_peaks(&topographic_map, y, x, Vec::new()))
                .map(|reachable_peaks| reachable_peaks.len() as u32)
                .sum::<u32>()
        )
        .sum())
}

fn parse_topographic_map(input: &str) -> Vec<Vec<u8>> {
    let topographic_map: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line|
            line
                .chars()
                .filter_map(|char| char.to_digit(10).map(|d| d as u8))
                .collect()
        )
        .collect();
    topographic_map
}

fn find_reachable_peaks(topographic_map: &Vec<Vec<u8>>, y: usize, x: usize, mut already_found_reachable_peaks: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let topographic_map_size = topographic_map.len();
    let current_height = topographic_map[y][x];
    if current_height == 9 {
        already_found_reachable_peaks.push((y, x));
        return already_found_reachable_peaks;
    }
    let next_positions_to_check: Vec<(usize, usize)> = [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .map(|&(dy, dx)| (y as i32 + dy, x as i32 + dx))
        .filter(|&(y, x)| {
            y >= 0 && y < topographic_map_size as i32 && x >= 0 && x < topographic_map_size as i32
        })
        .map(|(y, x)| (y as usize, x as usize))
        .filter(|&(y, x)| topographic_map[y][x] == current_height + 1)
        .collect();

    if next_positions_to_check.len() == 0 {
        return already_found_reachable_peaks;
    }

    for next_position in next_positions_to_check {
        already_found_reachable_peaks = find_reachable_peaks(&topographic_map, next_position.0, next_position.1, already_found_reachable_peaks);
    }

    already_found_reachable_peaks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
