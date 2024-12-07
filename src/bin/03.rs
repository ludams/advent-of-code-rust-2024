use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\)").unwrap();
    Some(
        input
            .lines()
            .flat_map(|line| mul_regex.captures_iter(line))
            .map(|captures| {
                captures["left"].parse::<u32>().unwrap() * captures["right"].parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(
        r"mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();
    let mut is_mul_enabled = true;
    Some(
        input
            .lines()
            .flat_map(|line| mul_regex.captures_iter(line))
            .filter_map(|captures| {
                if captures.name("dont").is_some() {
                    is_mul_enabled = false;
                    return None;
                }
                if captures.name("do").is_some() {
                    is_mul_enabled = true;
                    return None;
                }
                if is_mul_enabled {
                    return Some(
                        captures["left"].parse::<u32>().unwrap()
                            * captures["right"].parse::<u32>().unwrap(),
                    );
                }
                None
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
