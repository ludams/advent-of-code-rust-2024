use std::collections::HashSet;
use regex::Regex;

advent_of_code::solution!(14);

#[derive(Clone)]
struct Robot {
    x: usize,
    y: usize,
    dx: i64,
    dy: i64,
}

struct BathRoom {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl BathRoom {
    fn simulate(&mut self, seconds: u64) {
        self.robots.iter_mut().for_each(|robot| {
            robot.x = (robot.x as i64 + robot.dx * seconds as i64).rem_euclid(self.width as i64) as usize;
            robot.y = (robot.y as i64 + robot.dy * seconds as i64).rem_euclid(self.height as i64) as usize;
        });
    }
    
    fn safety_factor(&self) -> u64 {
        let middle_vertical = self.width / 2;
        let middle_horizontal = self.height / 2;

        let num_robots_in_q1 =  self.robots.iter()
            .filter(|robot| (0..middle_vertical).contains(&robot.x) && (0..middle_horizontal).contains(&robot.y))
            .count() as u64;
        let num_robots_in_q2 =  self.robots.iter()
            .filter(|robot| (0..middle_vertical).contains(&robot.x) && ((middle_horizontal + 1).. self.height).contains(&robot.y))
            .count() as u64;
        let num_robots_in_q3 =  self.robots.iter()
            .filter(|robot| ((middle_vertical + 1)..self.width).contains(&robot.x) && (0..middle_horizontal).contains(&robot.y))
            .count() as u64;
        let num_robots_in_q4 =  self.robots.iter()
            .filter(|robot| ((middle_vertical + 1)..self.width).contains(&robot.x) && ((middle_horizontal + 1).. self.height).contains(&robot.y))
            .count() as u64;

        num_robots_in_q1 * num_robots_in_q2 * num_robots_in_q3 * num_robots_in_q4
    }
    
    fn print(&self) {
        let mut bathroom = vec![vec![false; self.width]; self.height];
        self.robots.iter()
            .for_each(|robot| {
                bathroom[robot.y][robot.x] = true;
            });

        let bathroom_string = bathroom.iter().fold(String::new(), |string, row| {
            string + &row.iter().map(|cell| if *cell { "██" } else { "  " }).collect::<String>() + "\n"
        });
        println!("{}", bathroom_string);
    }
}

const BATHROOM_WIDTH: usize = 101;
const BATHROOM_HEIGHT: usize = 103;

pub fn part_one(input: &str) -> Option<u64> {
    let robots = parse_robots(input);
    let mut bathroom = BathRoom { robots, width: BATHROOM_WIDTH, height: BATHROOM_HEIGHT };
    bathroom.simulate(100);
    Some(bathroom.safety_factor())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bathroom = BathRoom { robots: parse_robots(input), width: BATHROOM_WIDTH, height: BATHROOM_HEIGHT };
    let mut seconds_passed: u32 = 0;
    loop {
        bathroom.simulate(1);
        seconds_passed += 1;
        let robots_on_upper_bound_box_edge_row = bathroom.robots.iter()
            .filter(|robot| robot.y == 26)
            .map(|robot| robot.x)
            .collect::<HashSet<_>>();

        let is_christmas_tree_arranged = (37..68).all(|i| robots_on_upper_bound_box_edge_row.contains(&i));
        if is_christmas_tree_arranged {
            // Uncomment the following lines to print out christmas tree
            // println!("\nAfter {} seconds:\n", seconds_passed);
            // bathroom.print();
            break;
        }
    }

    Some(seconds_passed)
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let robot_regex = Regex::new(r"p=(?<x>[0-9]+),(?<y>[0-9]+) v=(?<dx>-?[0-9]+),(?<dy>-?[0-9]+)").unwrap();
    robot_regex.captures_iter(input)
        .map(|captures| {
            Robot {
                x: captures.name("x").unwrap().as_str().parse().unwrap(),
                y: captures.name("y").unwrap().as_str().parse().unwrap(),
                dx: captures.name("dx").unwrap().as_str().parse().unwrap(),
                dy: captures.name("dy").unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let robots = parse_robots(&advent_of_code::template::read_file("examples", DAY));
        let mut bathroom = BathRoom { robots, width: 11, height: 7 };
        bathroom.simulate(100);
        let result = bathroom.safety_factor();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7861));
    }
}
