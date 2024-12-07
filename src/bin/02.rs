use std::iter::Map;
use std::str::Lines;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_to_iter(input)
            .filter(|levels| is_strictly_increasing(levels) || is_strictly_decreasing(levels))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_to_iter(input)
            .filter(|levels| {
                levels
                    .iter()
                    .enumerate()
                    .map(|(index, _level)| {
                        let mut partial_levels = levels.clone();
                        partial_levels.remove(index);
                        partial_levels
                    })
                    .any(|levels| {
                        is_strictly_increasing(&levels) || is_strictly_decreasing(&levels)
                    })
            })
            .count() as u32,
    )
}

fn is_strictly_decreasing(levels: &[u32]) -> bool {
    levels.windows(2).all(|w| {
        if w[1] >= w[0] {
            return false;
        }
        let diff = w[0] - w[1];
        (1..=3).contains(&diff)
    })
}

fn is_strictly_increasing(levels: &[u32]) -> bool {
    levels.windows(2).all(|w| {
        if w[0] >= w[1] {
            return false;
        }
        let diff = w[1] - w[0];
        (1..=3).contains(&diff)
    })
}

fn parse_to_iter(input: &str) -> Map<Lines, fn(&str) -> Vec<u32>> {
    input.lines().map(|l| {
        l.split_whitespace()
            .filter_map(|w| w.parse::<u32>().ok())
            .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
