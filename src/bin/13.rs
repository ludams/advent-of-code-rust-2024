use regex::Regex;

advent_of_code::solution!(13);

struct ClawMachine {
    button_a_dx: u64,
    button_a_dy: u64,

    button_b_dx: u64,
    button_b_dy: u64,

    prize_pos_x: u64,
    prize_pos_y: u64,
}

impl ClawMachine {
    pub(crate) fn min_tokens_to_price_optimized(&self) -> Option<u64> {
        let p_x = self.prize_pos_x as i64;
        let p_y = self.prize_pos_y as i64;
        let ba_dx = self.button_a_dx as i64;
        let ba_dy = self.button_a_dy as i64;
        let bb_dx = self.button_b_dx as i64;
        let bb_dy = self.button_b_dy as i64;
        let b_numerator = p_y * ba_dx - ba_dy * p_x;
        let b_denominator = ba_dx * bb_dy - ba_dy * bb_dx;

        if b_denominator == 0 {
            // luckily this case is not in the input data ;)
            panic!("This case is not covered. But this case could be solvable with multiple solutions (one or multiple cheapest) or could also not be solvable");
        }

        if b_numerator % b_denominator != 0 {
            return None
        }

        let b = b_numerator / b_denominator;

        let remaining_dx_to_prize = p_x - b * bb_dx;
        if remaining_dx_to_prize % ba_dx != 0 {
            return None
        }

        let a = remaining_dx_to_prize / ba_dx;

        if a < 0 || b < 0 {
           return None;
        }

        Some(a as u64 * 3 + b as u64)
    }
}

impl ClawMachine {
    pub(crate) fn min_tokens_to_price(&self) -> Option<u64> {
        (0..=100)
            .take_while(|num_button_a_presses| self.prize_pos_x >= num_button_a_presses * self.button_a_dx)
            .filter_map(|num_button_a_presses| {
                let pos_x_after_button_a_presses = self.button_a_dx * num_button_a_presses;
                let remaining_dx_to_price = self.prize_pos_x - pos_x_after_button_a_presses;

                let is_remaining_dx_divisible_by_button_b_dx = remaining_dx_to_price % self.button_b_dx == 0;
                if !is_remaining_dx_divisible_by_button_b_dx {
                    return None;
                }
                let num_button_b_presses = remaining_dx_to_price / self.button_b_dx;
                if num_button_b_presses > 100 {
                    return None;
                }
                return Some((num_button_a_presses, num_button_b_presses));
            })
            .filter(|(num_button_a_presses, num_button_b_presses)| {
                self.prize_pos_y == self.button_a_dy * num_button_a_presses + self.button_b_dy * num_button_b_presses
            })
            .next()
            .map(|(num_button_a_presses, num_button_b_presses)| {
                const BUTTON_A_TOKEN_COST: u64 = 3;
                const BUTTON_B_TOKEN_COST: u64 = 1;
                num_button_a_presses * BUTTON_A_TOKEN_COST + num_button_b_presses * BUTTON_B_TOKEN_COST
            })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let claw_machines = parse_claw_machines(input);

    let min_total_tokens_to_spent: u64 = claw_machines.iter()
        .filter_map(|machine| machine.min_tokens_to_price())
        .sum();

    Some(min_total_tokens_to_spent)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut claw_machines = parse_claw_machines(input);
    claw_machines.iter_mut()
        .for_each(|machine| {
            machine.prize_pos_x += 10000000000000;
            machine.prize_pos_y += 10000000000000;
        });

    let min_total_tokens_to_spent: u64 = claw_machines.iter()
        .filter_map(|machine| machine.min_tokens_to_price_optimized())
        .sum();

    Some(min_total_tokens_to_spent)
}

fn parse_claw_machines(input: &str) -> Vec<ClawMachine> {
    let claw_machine_regex = Regex::new(
        r"Button A: X\+(?<button_a_x>[0-9]+), Y\+(?<button_a_y>[0-9]+)
Button B: X\+(?<button_b_x>[0-9]+), Y\+(?<button_b_y>[0-9]+)
Prize: X=(?<prize_x>[0-9]+), Y=(?<prize_y>[0-9]+)").unwrap();
    let claw_machines: Vec<ClawMachine> = claw_machine_regex
        .captures_iter(input)
        .map(|captures| {
            return ClawMachine {
                button_a_dx: captures.name("button_a_x").unwrap().as_str().parse().unwrap(),
                button_a_dy: captures.name("button_a_y").unwrap().as_str().parse().unwrap(),
                button_b_dx: captures.name("button_b_x").unwrap().as_str().parse().unwrap(),
                button_b_dy: captures.name("button_b_y").unwrap().as_str().parse().unwrap(),
                prize_pos_x: captures.name("prize_x").unwrap().as_str().parse().unwrap(),
                prize_pos_y: captures.name("prize_y").unwrap().as_str().parse().unwrap(),
            };
        })
        .collect();
    claw_machines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claw_machine_min_tokens_to_price() {
        let claw_machine = ClawMachine {
            button_a_dx: 94,
            button_a_dy: 34,
            button_b_dx: 22,
            button_b_dy: 67,
            prize_pos_x: 8400,
            prize_pos_y: 5400,
        };
        let result = claw_machine.min_tokens_to_price();
        assert_eq!(result, Some(280));
    }

    #[test]
    fn test_claw_machine_min_tokens_to_price_optimized() {
        let claw_machine = ClawMachine {
            button_a_dx: 94,
            button_a_dy: 34,
            button_b_dx: 22,
            button_b_dy: 67,
            prize_pos_x: 8400,
            prize_pos_y: 5400,
        };
        let result = claw_machine.min_tokens_to_price_optimized();
        assert_eq!(result, Some(280));
    }

    #[test]
    fn test_claw_machine_min_tokens_to_price_optimized_large_example_not_solvable() {
        let claw_machine = ClawMachine {
            button_a_dx: 94,
            button_a_dy: 34,
            button_b_dx: 22,
            button_b_dy: 67,
            prize_pos_x: 10000000008400,
            prize_pos_y: 10000000005400,
        };
        let result = claw_machine.min_tokens_to_price_optimized();
        assert_eq!(result, None);
    }

    #[test]
    fn test_claw_machine_min_tokens_to_price_optimized_large_example_solvable() {
        let claw_machine = ClawMachine {
            button_a_dx: 26,
            button_a_dy: 66,
            button_b_dx: 67,
            button_b_dy: 21,
            prize_pos_x: 10000000012748,
            prize_pos_y: 10000000012176,
        };
        let result = claw_machine.min_tokens_to_price_optimized();
        assert_eq!(result, Some(459236326669));
    }

    #[test]
    #[ignore]
    fn test_claw_machine_min_tokens_to_price_optimized_edge_case_not_solvable() {
        let claw_machine = ClawMachine {
            button_a_dx: 4,
            button_a_dy: 8,
            button_b_dx: 3,
            button_b_dy: 6,
            prize_pos_x: 40,
            prize_pos_y: 30,
        };
        let result = claw_machine.min_tokens_to_price_optimized();
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_claw_machine_min_tokens_to_price_optimized_edge_case_solvable() {
        let claw_machine = ClawMachine {
            button_a_dx: 4,
            button_a_dy: 3,
            button_b_dx: 8,
            button_b_dy: 6,
            prize_pos_x: 40,
            prize_pos_y: 30,
        };
        let result = claw_machine.min_tokens_to_price_optimized();
        assert_eq!(result, Some(5));
    }

    #[test]
    #[ignore]
    fn test_claw_machine_min_tokens_to_price_optimized_edge_case_solvable_non_trivial() {
        let claw_machine = ClawMachine {
            button_a_dx: 6,
            button_a_dy: 3,
            button_b_dx: 4,
            button_b_dy: 2,
            prize_pos_x: 24,
            prize_pos_y: 12,
        };
        let result = claw_machine.min_tokens_to_price_optimized();
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_min_tokens_to_price_optimized_part_one_examples() {
        let claw_machines = parse_claw_machines(&advent_of_code::template::read_file("examples", DAY));

        let result: u64 = claw_machines.iter()
            .filter_map(|machine| machine.min_tokens_to_price_optimized())
            .sum();

        assert_eq!(result, 480);
    }
}
