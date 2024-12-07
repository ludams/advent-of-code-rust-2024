use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = parse_two_lists(input);

    list1.sort_unstable();
    list2.sort_unstable();

    Some(
        list1
            .iter()
            .zip(list2)
            .map(|pair| pair.0.abs_diff(pair.1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_two_lists(input);

    let mut list2_count_map: HashMap<u32, u32> = HashMap::new();

    list2.iter().for_each(|location_id| {
        let current_count = list2_count_map.get(location_id).unwrap_or(&0);
        list2_count_map.insert(*location_id, current_count + 1);
    });

    Some(
        list1
            .iter()
            .map(|location_id| list2_count_map.get(location_id).unwrap_or(&0) * location_id)
            .sum(),
    )
}

fn parse_two_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let location_ids: Vec<&str> = line.split("   ").collect();
        if location_ids.len() == 2 {
            list1.push(location_ids[0].parse().unwrap());
            list2.push(location_ids[1].parse().unwrap());
        }
    });
    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
