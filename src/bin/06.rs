use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
    
    fn get_shift_coords(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map_size = input.lines().count() as i32;
    let obstacle_coords = parse_obstacle_coords(input);
    let starting_coords: (i32, i32) = parse_starting_coords(input);
    
    let walk_result = walk_until_outside_or_loop(
        map_size,
        &obstacle_coords,
        starting_coords,
        Direction::Up,
    );
    match walk_result {
        WalkResult::OutsideMap(distinct_fields_visited) => Some(distinct_fields_visited.len() as u32),
        WalkResult::Loop => None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map_size = input.lines().count() as i32;
    let obstacle_coords = parse_obstacle_coords(input);
    let starting_coords: (i32, i32) = parse_starting_coords(input);

    let walk_result = walk_until_outside_or_loop(
        map_size,
        &obstacle_coords,
        starting_coords,
        Direction::Up,
    );

    let mut distinct_fields_visited = match walk_result {
        WalkResult::Loop => return None,
        WalkResult::OutsideMap(distinct_fields_visited) => distinct_fields_visited,
    };

    // remove starting position (obstacle not allowed)
    distinct_fields_visited.remove(&starting_coords);
    let amount_of_obstacles_leading_to_loop = distinct_fields_visited.iter()
        .filter(|&&coords_in_path| {
            let mut obstacles_with_added_obstacle = obstacle_coords.iter().cloned().collect::<HashSet<_>>();
            obstacles_with_added_obstacle.insert(coords_in_path);
            
            match walk_until_outside_or_loop(
                map_size,
                &obstacles_with_added_obstacle,
                starting_coords,
                Direction::Up,
            ) {
                WalkResult::OutsideMap(_) => false,
                WalkResult::Loop => true,
            }
        })
        .count() as u32;
    Some(amount_of_obstacles_leading_to_loop)
}

enum WalkResult {
    OutsideMap(HashSet<(i32, i32)>),
    Loop
}

fn walk_until_outside_or_loop(
    map_size: i32,
    obstacle_coords: &HashSet<(i32, i32)>,
    starting_coords: (i32, i32),
    starting_direction: Direction) -> WalkResult {
    let mut visited_coords: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();

    let mut current_coords: (i32, i32) = starting_coords;
    let mut current_direction = starting_direction;
    let mut current_shift = current_direction.get_shift_coords();
    while (0..map_size).contains(&current_coords.0) && (0..map_size).contains(&current_coords.1) {
        if let Some(already_walked_directions) = visited_coords.get_mut(&current_coords) {
            if already_walked_directions.contains(&current_direction) {
                return WalkResult::Loop;
            }
            already_walked_directions.insert(current_direction);
        } else {
            visited_coords.insert(current_coords, HashSet::from([current_direction]));
        }

        let mut next_coords = (current_coords.0 + current_shift.0, current_coords.1 + current_shift.1);
        while obstacle_coords.contains(&next_coords) {
            current_direction = current_direction.turn_right();
            current_shift = current_direction.get_shift_coords();
            next_coords = (current_coords.0 + current_shift.0, current_coords.1 + current_shift.1);
        }
        current_coords = next_coords;
    }
    WalkResult::OutsideMap(visited_coords.into_keys().collect())
}

fn parse_starting_coords(input: &str) -> (i32, i32) {
    input.lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find(|(_, info_char)| *info_char == '^')
                .map(|(x, _)| (x as i32, y as i32))
        })
        .expect("There should be one instance of '^' in the input data")
}

fn parse_obstacle_coords(input: &str) -> HashSet<(i32, i32)> {
    let obstacle_coords: HashSet<(i32, i32)> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, info_char)| *info_char == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();
    obstacle_coords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
