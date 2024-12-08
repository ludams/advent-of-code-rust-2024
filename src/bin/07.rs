advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_input = parse_input(input);
    Some(parsed_input.iter()
        .filter(|(eq_result, eq_operands)| {
            is_equation_solvable(eq_result, eq_operands)
        })
        .map(|(eq_result, _)| eq_result)
        .sum())
}

fn is_equation_solvable(eq_result: &u64, eq_operands: &[u64]) -> bool {
    if *eq_result < eq_operands[0] {
        return false;
    }

    if eq_operands.len() == 1 {
        return *eq_result == eq_operands[0];
    }

    if eq_operands.len() == 2 {
        return
            is_equation_solvable(eq_result, &[eq_operands[0] + eq_operands[1]])
            || is_equation_solvable(eq_result, &[eq_operands[0] * eq_operands[1]]);
    }

    is_equation_solvable(eq_result, &[&[eq_operands[0] + eq_operands[1]], &eq_operands[2..]].concat())
        || is_equation_solvable(eq_result, &[&[eq_operands[0] * eq_operands[1]], &eq_operands[2..]].concat())
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_input = parse_input(input);
    Some(parsed_input.iter()
        .filter(|(eq_result, eq_operands)| {
            is_equation_solvable_with_concatenation(eq_result, eq_operands)
        })
        .map(|(eq_result, _)| eq_result)
        .sum())
}

fn is_equation_solvable_with_concatenation(eq_result: &u64, eq_operands: &[u64]) -> bool {
    if *eq_result < eq_operands[0] {
        return false;
    }

    if eq_operands.len() == 1 {
        return *eq_result == eq_operands[0];
    }

    if eq_operands.len() == 2 {
        return
            is_equation_solvable_with_concatenation(eq_result, &[eq_operands[0] + eq_operands[1]])
                || is_equation_solvable_with_concatenation(eq_result, &[eq_operands[0] * eq_operands[1]])
                || is_equation_solvable_with_concatenation(eq_result, &[concatenate(eq_operands[0], eq_operands[1])]);
    }

    is_equation_solvable_with_concatenation(eq_result, &[&[eq_operands[0] + eq_operands[1]], &eq_operands[2..]].concat())
        || is_equation_solvable_with_concatenation(eq_result, &[&[eq_operands[0] * eq_operands[1]], &eq_operands[2..]].concat())
        || is_equation_solvable_with_concatenation(eq_result, &[&[concatenate(eq_operands[0], eq_operands[1])], &eq_operands[2..]].concat())
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let eq_result = split
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let eq_operands = split
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            (eq_result, eq_operands)
        })
        .collect::<Vec<_>>()
}

fn concatenate(operand_1: u64, operand_2: u64) -> u64 {
    (operand_1.to_string() + &operand_2.to_string()).parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
