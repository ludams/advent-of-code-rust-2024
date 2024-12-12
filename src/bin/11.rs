use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let initial_stones = parse_stones(input);
    const NUMBER_OF_BLINKS: u64 = 25;
    Some(initial_stones.iter()
        .map(|stone_number| calculate_amount_of_stones(*stone_number, NUMBER_OF_BLINKS))
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones = parse_stones(input);
    const NUMBER_OF_BLINKS: u64 = 75;
    let mut cache = HashMap::new();
    Some(initial_stones.iter()
        .map(|stone_number| calculate_amount_of_stones_using_cache(*stone_number, NUMBER_OF_BLINKS, &mut cache))
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

fn calculate_amount_of_stones(stone_number: u64, number_of_blinks: u64) -> u64 {
    if number_of_blinks == 0 {
        return 1
    }

    if stone_number == 0 {
        return calculate_amount_of_stones(1, number_of_blinks - 1);
    }

    let log10 = stone_number.ilog10();
    let has_even_number_of_digits = log10 % 2 == 1;
    if has_even_number_of_digits {
        let divisor: u64 = 10_u64.pow(log10 / 2 + 1);
        let first_half = stone_number / divisor;
        let second_half = stone_number % divisor;
        let amount_of_stones_first_half_after_blink = calculate_amount_of_stones(first_half, number_of_blinks - 1);
        let amount_of_stones_second_half_after_blink = calculate_amount_of_stones(second_half, number_of_blinks - 1);
        let amount_of_stones_after_blink = amount_of_stones_first_half_after_blink + amount_of_stones_second_half_after_blink;
        return amount_of_stones_after_blink;
    }

    calculate_amount_of_stones(stone_number * 2024, number_of_blinks - 1)
}

fn calculate_amount_of_stones_using_cache(stone_number: u64, number_of_blinks: u64, cache: &mut HashMap<u64, HashMap<u64, u64>>) -> u64 {
    if number_of_blinks == 0 {
        return 1
    }

    let amount_of_stones = cache
        .entry(stone_number)
        .or_insert_with(HashMap::new)
        .get(&number_of_blinks);

    if let Some(amount_of_stones) = amount_of_stones {
        return *amount_of_stones;
    }

    if stone_number == 0 {
        let amount_of_stones_after_blink = calculate_amount_of_stones_using_cache(1, number_of_blinks - 1, cache);
        cache.get_mut(&stone_number).unwrap().insert(number_of_blinks, amount_of_stones_after_blink);
        return amount_of_stones_after_blink;
    }

    let log10 = stone_number.ilog10();
    let has_even_number_of_digits = log10 % 2 == 1;
    if has_even_number_of_digits {
        let divisor: u64 = 10_u64.pow(log10 / 2 + 1);
        let first_half = stone_number / divisor;
        let second_half = stone_number % divisor;
        let amount_of_stones_first_half_after_blink = calculate_amount_of_stones_using_cache(first_half, number_of_blinks - 1, cache);
        let amount_of_stones_second_half_after_blink = calculate_amount_of_stones_using_cache(second_half, number_of_blinks - 1, cache);
        let amount_of_stones_after_blink = amount_of_stones_first_half_after_blink + amount_of_stones_second_half_after_blink;
        cache.get_mut(&stone_number).unwrap().insert(number_of_blinks, amount_of_stones_after_blink);
        return amount_of_stones_after_blink;
    }

    let amount_of_stones_after_blink = calculate_amount_of_stones_using_cache(stone_number * 2024, number_of_blinks - 1, cache);
    cache.get_mut(&stone_number).unwrap().insert(number_of_blinks, amount_of_stones_after_blink);
    amount_of_stones_after_blink
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_amount_of_stones() {
        let result = calculate_amount_of_stones(125, 6);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_calculate_amount_of_stones_using_cache() {
        let result = calculate_amount_of_stones_using_cache(125, 6, &mut HashMap::new());
        assert_eq!(result, 7);
    }

    #[test]
    fn test_calculate_amount_of_stones_with_more() {
        let first_result = calculate_amount_of_stones(125, 25);
        let second_result = calculate_amount_of_stones(17, 25);
        assert_eq!(first_result + second_result, 55312);
    }

    #[test]
    fn test_calculate_amount_of_stones_using_cache_with_more() {
        let mut cache = HashMap::new();
        let first_result = calculate_amount_of_stones_using_cache(125, 25, &mut cache);
        let second_result = calculate_amount_of_stones_using_cache(17, 25, &mut cache);
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
