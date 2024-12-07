use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_ordering_rules(split[0]);
    let correctly_ordered_lines = parse_pages_to_produce(split[1])
        .filter(|page_numbers| is_page_correctly_ordered(page_numbers, &rules))
        .collect::<Vec<Vec<u32>>>();
    Some(calculate_middle_numbers_sum(correctly_ordered_lines))
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_ordering_rules(split[0]);
    let updated_correctly_ordered_lines = parse_pages_to_produce(split[1])
        .filter(|page_numbers| !is_page_correctly_ordered(page_numbers, &rules))
        .map(|page_numbers| {
            let page_numbers_set: HashSet<u32> = HashSet::from_iter(page_numbers.iter().cloned());
            let mut scoped_rules = create_scoped_rules(&rules, &page_numbers, page_numbers_set);
            let mut updated_page_numbers = Vec::new();
            (0..page_numbers.len()).for_each(|_index| {
                let (&&next_page_number, _) = scoped_rules.iter()
                    .find(|(_, following_page_numbers)| following_page_numbers.len() == 0)
                    .unwrap();
                updated_page_numbers.push(next_page_number);
                scoped_rules.remove(&next_page_number);
                scoped_rules.iter_mut().for_each(|(_, following_page_numbers)| {
                    following_page_numbers.remove(&next_page_number);
                });
            });
            updated_page_numbers.reverse();
            updated_page_numbers
        })
        .collect::<Vec<_>>();
    Some(calculate_middle_numbers_sum(updated_correctly_ordered_lines))
}

fn create_scoped_rules<'a, 'b>(rules: &'a HashMap<u32, HashSet<u32>>, page_numbers: &'a Vec<u32>, page_numbers_set: HashSet<u32>) -> HashMap<&'a u32, HashSet<u32>> {
    page_numbers.iter().map(|page_number| {
        let following_page_numbers = rules.get(page_number)
            .map(|following_page_numbers| {
                let filtered_relevant_page_numbers_iter = following_page_numbers
                    .iter()
                    .cloned()
                    .filter(|page_number| page_numbers_set.contains(page_number));
                HashSet::from_iter(filtered_relevant_page_numbers_iter)
            })
            .unwrap_or(HashSet::new());
        (page_number, following_page_numbers)
    })
        .collect::<HashMap<_, _>>()
}

fn parse_ordering_rules(input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    input.lines().for_each(|line| {
        let mut split = line.split('|');
        let left = split.next().unwrap().parse::<u32>().unwrap();
        let right = split.next().unwrap().parse::<u32>().unwrap();
        rules.entry(left)
            .and_modify(|after_numbers| _ = after_numbers.insert(right))
            .or_insert(HashSet::from([right]));
    });
    rules
}

fn parse_pages_to_produce(input: &str) -> impl Iterator<Item = Vec<u32>> + use<'_> {
    input.lines()
        .map(|line| line.split(',')
            .map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>())
}

fn is_page_correctly_ordered(page_numbers: &[u32], ordering_rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut before_numbers: HashSet<u32> = HashSet::new();
    page_numbers.iter().all(|number| {
        let rule_violation = match ordering_rules.get(&number) {
            Some(forbidden_before) => forbidden_before.intersection(&before_numbers).count() > 0,
            None => false
        };
        before_numbers.insert(*number);
        !rule_violation
    })
}

fn calculate_middle_numbers_sum(correctly_ordered_lines: Vec<Vec<u32>>) -> u32 {
    let middle_numbers = correctly_ordered_lines.iter()
        .map(|numbers| numbers[numbers.len() / 2])
        .collect::<Vec<_>>();
    middle_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
