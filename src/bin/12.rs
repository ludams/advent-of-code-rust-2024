use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

#[derive(Debug)]
struct Region {
    plant_type: char,
    garden_plots: Vec<(usize, usize)>
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn get_coords_shift(&self) -> (i8, i8) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn dy(&self) -> i8 {
        self.get_coords_shift().0
    }

    fn dx(&self) -> i8 {
        self.get_coords_shift().1
    }

    fn get_all() -> [Direction; 4] {
        const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        ALL_DIRECTIONS
    }
}

impl Region {
    pub fn area(&self) -> usize {
        self.garden_plots.len()
    }

    pub fn perimeter(&self) -> usize {
        self.garden_plots
            .iter()
            .map(|&(y, x)| {
                Direction::get_all()
                    .iter()
                    .map(|d| (y as i32 + d.dy() as i32, x as i32 + d.dx() as i32))
                    .filter(|&(y, x)|
                        y < 0 || x < 0 || !self.garden_plots.contains(&(y as usize, x as usize)))
                    .count()
            })
            .sum()
    }

    pub fn number_of_sides(&self) -> usize {
        self.garden_plots
            .iter()
            .flat_map(|&(y, x)| {
                let sides_of_plot = Direction::get_all()
                    .iter()
                    .copied()
                    .map(|d| (d, (y as i32 + d.dy() as i32, x as i32 + d.dx() as i32)))
                    .filter(|&(_d, (y, x))|
                        y < 0 || x < 0 || !self.garden_plots.contains(&(y as usize, x as usize)))
                    .map(|(d, (y, x))| {
                        match d {
                            Direction::Up => ((d, y), x),
                            Direction::Down => ((d, y), x),
                            Direction::Left => ((d, x), y),
                            Direction::Right => ((d, x), y)
                        }
                    }).collect::<Vec<_>>();
                sides_of_plot
            })
            .fold(HashMap::new(), |mut acc: HashMap<(Direction, i32), Vec<i32>>, (direction_with_coord_along, orthogonal_coord)| {
                acc.entry(direction_with_coord_along)
                    .or_insert_with(Vec::new)
                    .push(orthogonal_coord);
                acc
            })
            .values_mut()
            .map(|coords| {
                if coords.len() == 1 {
                    return 1;
                }
                coords.sort();
                let (_, number_of_sides) = coords.iter()
                    .skip(1)
                    .fold((coords[0], 1_usize), |(last_coord, number_of_sides), &current_coord| {
                        if current_coord > last_coord + 1 {
                            return (current_coord, number_of_sides + 1);
                        }
                        (current_coord, number_of_sides)
                    });
                number_of_sides
            })
            .sum()
    }

    pub fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    pub fn discounted_price(&self) -> usize {
        self.area() * self.number_of_sides()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = parse_map(input);
    let regions = get_regions(map);

    let price = regions
        .iter()
        .map(|r| r.price() as u64)
        .sum();
    Some(price)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = parse_map(input);
    let regions = get_regions(map);

    let price = regions
        .iter()
        .map(|r| r.discounted_price() as u64)
        .sum();
    Some(price)
}

fn get_regions(map: Vec<Vec<char>>) -> Vec<Region> {
    let map_size = map.len();
    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut unvisited: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);

    while let Some(region_initial_coords) = unvisited.iter().next().cloned() {
        unvisited.remove(&region_initial_coords);
        let (initial_y, initial_x) = region_initial_coords;
        let mut next_region = Region {
            plant_type: map[initial_y][initial_x],
            garden_plots: Vec::new(),
        };
        let mut unvisited_same_region: HashSet<(usize, usize)> = HashSet::from([(initial_y, initial_x)]);

        while let Some(region_next_coords) = unvisited_same_region.iter().next().cloned() {
            unvisited_same_region.remove(&region_next_coords);
            next_region.garden_plots.push(region_next_coords);

            visited.insert(region_next_coords);
            unvisited.remove(&region_next_coords);

            let (y, x) = region_next_coords;
            let coords_shifts: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let neighbor_plots: Vec<(usize, usize)> = coords_shifts
                .iter()
                .map(|&(dy, dx)| (y as i32 + dy as i32, x as i32 + dx as i32))
                .filter(|&(neighbor_y, neighbor_x)| (0..map_size as i32).contains(&neighbor_y) && (0..map_size as i32).contains(&neighbor_x))
                .map(|(neighbor_y, neighbor_x)| (neighbor_y as usize, neighbor_x as usize))
                .filter(|neighbor| !visited.contains(neighbor))
                .collect();

            neighbor_plots
                .iter()
                .for_each(|&(y, x)| {
                    if map[y][x] == next_region.plant_type {
                        unvisited_same_region.insert((y, x));
                    } else {
                        unvisited.insert((y, x));
                    }
                });
        }

        regions.push(next_region);
    }
    regions
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_get_regions() {
        let result = get_regions(parse_map(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result.len(), 11);
        assert_eq!(result.iter().map(|r| r.area()).sum::<usize>(), 100);
    }

    #[test]
    fn test_area() {
        let region = Region { plant_type: 'A', garden_plots: vec![(0, 0), (0, 1), (0, 2), (0, 3)] };
        let result = region.area();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_perimeter() {
        let region = Region { plant_type: 'A', garden_plots: vec![(0, 0), (0, 1), (0, 2), (0, 3)] };
        let result = region.perimeter();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_number_of_sides() {
        let region = Region { plant_type: 'A', garden_plots: vec![(0, 0), (0, 1), (0, 2), (0, 3)] };
        let result = region.number_of_sides();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_number_of_sides_complex() {
        let region = Region { plant_type: 'C', garden_plots: vec![(1, 2), (2, 2), (2, 3), (3, 3)] };
        let result = region.number_of_sides();
        assert_eq!(result, 8);
    }
}
