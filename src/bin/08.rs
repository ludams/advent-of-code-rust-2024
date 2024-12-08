use std::collections::{HashMap, HashSet};
use gcd::Gcd;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let antenna_map_size = input.lines().filter(|line| !line.is_empty()).count() as i32;
    let antenna_map = parse_antenna_map(input);
    let mut found_antinodes = HashSet::<(i32, i32)>::new();
    antenna_map.iter().for_each(|(_freq, coords)| {
        coords.iter().enumerate().for_each(|(current_antenna_index, &(x, y))| {
            coords
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != current_antenna_index)
                .for_each(|(_, &(other_x, other_y))| {
                    found_antinodes.insert((2 * other_x - x, 2 * other_y - y));
                });
        })
    });
    Some(found_antinodes
        .iter()
        .filter(|(x, y)| (0..antenna_map_size).contains(x) && (0..antenna_map_size).contains(y))
        .count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let antenna_map_size = input.lines().filter(|line| !line.is_empty()).count() as i32;
    let antenna_map = parse_antenna_map(input);
    let mut found_antinodes = HashSet::<(i32, i32)>::new();
    antenna_map.iter().for_each(|(_freq, coords)| {
        let mut coords_for_frequency = coords.iter().collect::<HashSet<_>>();
        coords.iter().enumerate().for_each(|(_, current_antenna_coords)| {
            let (x, y) = *current_antenna_coords;
            coords_for_frequency.remove(current_antenna_coords);
            coords_for_frequency
                .iter()
                .enumerate()
                .for_each(|(_, &(other_x, other_y))| {
                    found_antinodes.insert(*current_antenna_coords);

                    let diff_y = other_y - y;
                    let diff_x = other_x - x;
                    let gcd = (diff_x.abs() as u32).gcd(diff_y.abs() as u32) as i32;

                    let diff_y = diff_y / gcd;
                    let diff_x = diff_x / gcd;

                    let mut next_antinode_x = x + diff_x;
                    let mut next_antinode_y = y + diff_y;
                    while (0..antenna_map_size).contains(&next_antinode_x) && (0..antenna_map_size).contains(&next_antinode_y) {
                        found_antinodes.insert((next_antinode_x, next_antinode_y));
                        next_antinode_x += diff_x;
                        next_antinode_y += diff_y;
                    }

                    let mut next_antinode_x = x - diff_x;
                    let mut next_antinode_y = y - diff_y;
                    while (0..antenna_map_size).contains(&next_antinode_x) && (0..antenna_map_size).contains(&next_antinode_y) {
                        found_antinodes.insert((next_antinode_x, next_antinode_y));
                        next_antinode_x -= diff_x;
                        next_antinode_y -= diff_y;
                    }
                });
        })
    });
    Some(found_antinodes.len() as u32)
}

fn parse_antenna_map(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antenna_map = HashMap::<char, Vec<(i32, i32)>>::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, freq)| {
            if freq != '.' {
                antenna_map
                    .entry(freq)
                    .and_modify(|coords| coords.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        })
    });
    antenna_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
