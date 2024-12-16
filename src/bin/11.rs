use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let initial_stones = parse_stones(input);
    const NUMBER_OF_BLINKS: u64 = 25;
    Some(initial_stones.iter()
        .map(|stone_number| num_stones_after_blinks(*stone_number, NUMBER_OF_BLINKS))
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones = parse_stones(input);
    const NUMBER_OF_BLINKS: u64 = 75;
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    Some(initial_stones.iter()
        .map(|stone_number| num_stones_after_blinks_optimized(*stone_number, NUMBER_OF_BLINKS, &mut cache))
        .sum())
}

fn parse_stones(input: &str) -> Vec<u64> {
    let initial_stones = input
        .split_whitespace()
        .filter(|s| *s != "")
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    initial_stones
}

fn num_stones_after_blinks(stone_number: u64, number_of_blinks: u64) -> u64 {
    if number_of_blinks == 0 {
        return 1
    }

    if stone_number == 0 {
        return num_stones_after_blinks(1, number_of_blinks - 1);
    }

    let log10 = stone_number.ilog10();
    let has_even_number_of_digits = log10 % 2 == 1;
    if has_even_number_of_digits {
        let divisor: u64 = 10_u64.pow(log10 / 2 + 1);
        let first_half = stone_number / divisor;
        let second_half = stone_number % divisor;
        let num_stones_first_half_after_blink = num_stones_after_blinks(first_half, number_of_blinks - 1);
        let num_stones_second_half_after_blink = num_stones_after_blinks(second_half, number_of_blinks - 1);
        let num_stones_after_blink = num_stones_first_half_after_blink + num_stones_second_half_after_blink;
        return num_stones_after_blink;
    }

    num_stones_after_blinks(stone_number * 2024, number_of_blinks - 1)
}

fn num_stones_after_blinks_optimized(stone_number: u64, num_blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if num_blinks == 0 {
        return 1
    }

    if let Some(num_stones) = cache.get(&(stone_number, num_blinks)) {
        return *num_stones;
    }

    if stone_number == 0 {
        let num_stones = num_stones_after_blinks_optimized(1, num_blinks - 1, cache);
        cache.insert((stone_number, num_blinks), num_stones);
        return num_stones;
    }

    let log10 = stone_number.ilog10();
    let has_even_number_of_digits = log10 % 2 == 1;
    if has_even_number_of_digits {
        let divisor: u64 = 10_u64.pow(log10 / 2 + 1);
        let first_half = stone_number / divisor;
        let second_half = stone_number % divisor;
        let second_half_num_stones = num_stones_after_blinks_optimized(first_half, num_blinks - 1, cache);
        let first_half_num_stones = num_stones_after_blinks_optimized(second_half, num_blinks - 1, cache);
        let num_stones = second_half_num_stones + first_half_num_stones;
        cache.insert((stone_number, num_blinks), num_stones);
        return num_stones;
    }

    let num_stones = num_stones_after_blinks_optimized(stone_number * 2024, num_blinks - 1, cache);
    cache.insert((stone_number, num_blinks), num_stones);
    num_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_stones() {
        let result = num_stones_after_blinks(125, 6);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_num_stones_using_cache() {
        let result = num_stones_after_blinks_optimized(125, 6, &mut HashMap::new());
        assert_eq!(result, 7);
    }

    #[test]
    fn test_num_stones_with_more() {
        let first_result = num_stones_after_blinks(125, 25);
        let second_result = num_stones_after_blinks(17, 25);
        assert_eq!(first_result + second_result, 55312);
    }

    #[test]
    fn test_num_stones_using_cache_with_more() {
        let mut cache = HashMap::new();
        let first_result = num_stones_after_blinks_optimized(125, 25, &mut cache);
        let second_result = num_stones_after_blinks_optimized(17, 25, &mut cache);
        assert_eq!(first_result + second_result, 55312);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
