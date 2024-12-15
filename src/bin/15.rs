use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

advent_of_code::solution!(15);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_coords_by_one(&self, coords: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (coords.0 - 1, coords.1),
            Direction::Down => (coords.0 + 1, coords.1),
            Direction::Left => (coords.0, coords.1 - 1),
            Direction::Right => (coords.0, coords.1 + 1),
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => false,
            Direction::Left | Direction::Right => true,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let char = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", char.to_string())
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("invalid direction \"{}\"", c),
        }
    }
}

struct RobotMovements {
    directions: Vec<Direction>,
}

impl FromStr for RobotMovements {
    type Err = ();

    fn from_str(movements_str: &str) -> Result<Self, Self::Err> {
        let directions: Vec<Direction> = movements_str
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Direction::from)
            .collect();
        Ok(RobotMovements { directions })
    }
}

struct Robot {
    y: usize,
    x: usize,
}

#[derive(Copy, Clone)]
enum WareHouseObstacle {
    None,
    Wall,
    Box,
}

impl From<char> for WareHouseObstacle {
    fn from(c: char) -> Self {
        match c {
            '#' => WareHouseObstacle::Wall,
            'O' => WareHouseObstacle::Box,
            '.' => WareHouseObstacle::None,
            _ => panic!("invalid warehouse obstacle \"{}\"", c),
        }
    }
}

impl Display for WareHouseObstacle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let char = match self {
            WareHouseObstacle::None => '.',
            WareHouseObstacle::Wall => '#',
            WareHouseObstacle::Box => 'O',
        };
        write!(f, "{}", char.to_string())
    }
}

struct Warehouse {
    size: usize,
    robot: Robot,
    obstacles: Vec<Vec<WareHouseObstacle>>,
}

impl Warehouse {
    pub(crate) fn sum_box_coords(&self) -> u64 {
        self.obstacles
            .iter()
            .enumerate()
            .flat_map(|(y, col)| {
                col.iter()
                    .enumerate()
                    .filter_map(move |(x, obstacle)| match obstacle {
                        WareHouseObstacle::Box => Some((y, x)),
                        _ => None,
                    })
            })
            .map(|(y, x)| (x + 100 * y) as u64)
            .sum()
    }
}

impl Warehouse {
    pub(crate) fn simulate_robot_movements(&mut self, movements: RobotMovements) {
        movements.directions.iter().cloned().for_each(|direction| {
            let (robot_next_y, robot_next_x) =
                direction.move_coords_by_one(&(self.robot.y, self.robot.x));

            let next_pos_obstacle = self.obstacles[robot_next_y][robot_next_x];
            let try_moving_box = match next_pos_obstacle {
                WareHouseObstacle::None => {
                    self.robot.y = robot_next_y;
                    self.robot.x = robot_next_x;
                    false
                }
                WareHouseObstacle::Wall => false,
                WareHouseObstacle::Box => true,
            };

            if try_moving_box {
                self.try_push_box(robot_next_y, robot_next_x, &direction);
            }
        })
    }

    fn try_push_box(&mut self, box_y: usize, box_x: usize, direction: &Direction) {
        let mut box_next_y = box_y;
        let mut box_next_x = box_x;

        while (0..self.size).contains(&box_next_y) && (0..self.size).contains(&box_next_x) {
            (box_next_y, box_next_x) = direction.move_coords_by_one(&(box_next_y, box_next_x));
            let obstacle = self.obstacles[box_next_y][box_next_x];
            match obstacle {
                WareHouseObstacle::None => {
                    self.obstacles[box_next_y][box_next_x] = WareHouseObstacle::Box;

                    self.obstacles[box_y][box_x] = WareHouseObstacle::None;
                    self.robot.y = box_y;
                    self.robot.x = box_x;
                    return;
                }
                WareHouseObstacle::Wall => {
                    // can't move box(es) because it (they) is (are) blocked by wall
                    return;
                }
                WareHouseObstacle::Box => {
                    continue;
                }
            }
        }
    }
}

impl FromStr for Warehouse {
    type Err = ();
    fn from_str(warehouse_str: &str) -> Result<Self, Self::Err> {
        let warehouse_size: usize = warehouse_str.lines().count();
        let warehouse_obstacles: Vec<Vec<WareHouseObstacle>> = warehouse_str
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char.clone() {
                        '@' => WareHouseObstacle::None,
                        non_robot_char => WareHouseObstacle::from(non_robot_char),
                    })
                    .collect()
            })
            .collect();

        let robot: Robot = warehouse_str
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '@' => Some(Robot { y, x }),
                        _ => None,
                    })
            })
            .next()
            .unwrap();

        let warehouse = Warehouse {
            size: warehouse_size,
            obstacles: warehouse_obstacles,
            robot,
        };
        Ok(warehouse)
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (y, row) in self.obstacles.iter().enumerate() {
            for (x, obstacle) in row.iter().enumerate() {
                if self.robot.x == x && self.robot.y == y {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", obstacle.to_string())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
enum LargeWareHouseObstacle {
    None,
    Wall,
    // bool = isRightPart
    Box(bool),
}

impl From<char> for LargeWareHouseObstacle {
    fn from(c: char) -> Self {
        match c {
            '#' => LargeWareHouseObstacle::Wall,
            '[' => LargeWareHouseObstacle::Box(false),
            ']' => LargeWareHouseObstacle::Box(true),
            '.' => LargeWareHouseObstacle::None,
            _ => panic!("invalid warehouse obstacle \"{}\"", c),
        }
    }
}

impl Display for LargeWareHouseObstacle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let char = match self {
            LargeWareHouseObstacle::None => '.',
            LargeWareHouseObstacle::Wall => '#',
            LargeWareHouseObstacle::Box(false) => '[',
            LargeWareHouseObstacle::Box(true) => ']',
        };
        write!(f, "{}", char.to_string())
    }
}

struct LargeWarehouse {
    robot: Robot,
    obstacles: Vec<Vec<LargeWareHouseObstacle>>,
}

impl LargeWarehouse {
    pub(crate) fn sum_box_coords(&self) -> u64 {
        self.obstacles
            .iter()
            .enumerate()
            .flat_map(|(y, col)| {
                col.iter()
                    .enumerate()
                    .filter_map(move |(x, obstacle)| match obstacle {
                        LargeWareHouseObstacle::Box(false) => Some((y, x)),
                        _ => None,
                    })
            })
            .map(|(y, x)| (x + 100 * y) as u64)
            .sum()
    }
}

impl LargeWarehouse {
    pub(crate) fn simulate_robot_movements(&mut self, movements: RobotMovements) {
        movements.directions.iter().cloned().for_each(|direction| {
            let (robot_next_y, robot_next_x) =
                direction.move_coords_by_one(&(self.robot.y, self.robot.x));

            let next_pos_obstacle = self.obstacles[robot_next_y][robot_next_x];
            let try_moving_box = match next_pos_obstacle {
                LargeWareHouseObstacle::None => {
                    self.robot.y = robot_next_y;
                    self.robot.x = robot_next_x;
                    false
                }
                LargeWareHouseObstacle::Wall => false,
                LargeWareHouseObstacle::Box(_) => true,
            };

            if try_moving_box {
                self.try_push_box(robot_next_y, robot_next_x, &direction);
            }
        })
    }

    fn boxes_to_push(
        &self,
        box_y: usize,
        box_x: usize,
        direction: &Direction,
    ) -> Option<Vec<((usize, usize), bool)>> {
        if direction.is_horizontal() {
            return self.boxes_to_push_horizontally(box_y, box_x, direction);
        }

        self.boxes_to_push_vertically(box_y, box_x, direction)
    }

    fn boxes_to_push_horizontally(
        &self,
        box_y: usize,
        box_x: usize,
        direction: &Direction,
    ) -> Option<Vec<((usize, usize), bool)>> {
        let LargeWareHouseObstacle::Box(is_right_part) = self.obstacles[box_y][box_x] else {
            panic!(
                "wrong argument, obstacle at (y={}, x={}) has to be a box",
                box_y, box_x
            );
        };
        let mut current_box = ((box_y, box_x), is_right_part);
        let mut boxes_to_push: Vec<((usize, usize), bool)> = Vec::new();

        loop {
            boxes_to_push.push(current_box);
            let (next_box_y, next_box_x) = direction.move_coords_by_one(&current_box.0);
            let obstacle = self.obstacles[next_box_y][next_box_x];
            match obstacle {
                LargeWareHouseObstacle::None => {
                    break;
                }
                LargeWareHouseObstacle::Wall => {
                    return None;
                }
                LargeWareHouseObstacle::Box(is_right_part) => {
                    current_box = ((next_box_y, next_box_x), is_right_part)
                }
            }
        }

        Some(boxes_to_push)
    }

    fn boxes_to_push_vertically(
        &self,
        box_y: usize,
        box_x: usize,
        direction: &Direction,
    ) -> Option<Vec<((usize, usize), bool)>> {
        let LargeWareHouseObstacle::Box(is_right_part) = self.obstacles[box_y][box_x] else {
            panic!("there is no box at (y={}, x={})", box_y, box_x);
        };
        // let warehouse_str = self.to_string();

        let mut boxes_to_check: HashSet<((usize, usize), bool)> = HashSet::from([((box_y, box_x), is_right_part)]);
        let mut boxes_to_push: HashSet<((usize, usize), bool)> = HashSet::new();
        while let Some(box_part) = boxes_to_check.iter().cloned().next() {
            boxes_to_check.remove(&box_part);
            if boxes_to_push.contains(&box_part) {
                continue;
            }
            let box_coords = box_part.0;
            let is_right_box_part = box_part.1;
            let direction_to_other_box_part = if is_right_box_part {
                Direction::Left
            } else {
                Direction::Right
            };
            let other_box_part_coords = direction_to_other_box_part.move_coords_by_one(&box_coords);
            boxes_to_check.insert((other_box_part_coords, !is_right_box_part));

            let (next_obstacle_y, next_obstacle_x) = direction.move_coords_by_one(&box_coords);
            let next_obstacle = self.obstacles[next_obstacle_y][next_obstacle_x];
            match next_obstacle {
                LargeWareHouseObstacle::None => {}
                LargeWareHouseObstacle::Wall => {
                    return None;
                }
                LargeWareHouseObstacle::Box(is_right_part) => {
                    boxes_to_check.insert(((next_obstacle_y, next_obstacle_x), is_right_part));
                }
            }
            boxes_to_push.insert(box_part);
        }

        Some(boxes_to_push.into_iter().collect())
    }

    fn try_push_box(&mut self, box_y: usize, box_x: usize, direction: &Direction) {

        let Some(boxes_to_push) = self.boxes_to_push(box_y, box_x, direction) else {
            return;
        };

        boxes_to_push.iter().for_each(|(box_coords, _)| {
            self.obstacles[box_coords.0][box_coords.1] = LargeWareHouseObstacle::None;
        });
        boxes_to_push.iter().for_each(|&(box_coords, is_right_part)| {
            let pushed_coords = direction.move_coords_by_one(&box_coords);
            self.obstacles[pushed_coords.0][pushed_coords.1] = LargeWareHouseObstacle::Box(is_right_part);
        });

        self.robot.y = box_y;
        self.robot.x = box_x;
    }
}

impl FromStr for LargeWarehouse {
    type Err = ();
    fn from_str(warehouse_str: &str) -> Result<Self, Self::Err> {
        let warehouse_obstacles: Vec<Vec<LargeWareHouseObstacle>> = warehouse_str
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char.clone() {
                        '@' => LargeWareHouseObstacle::None,
                        non_robot_char => LargeWareHouseObstacle::from(non_robot_char),
                    })
                    .collect()
            })
            .collect();

        let robot: Robot = warehouse_str
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        '@' => Some(Robot { y, x }),
                        _ => None,
                    })
            })
            .next()
            .unwrap();

        let warehouse = LargeWarehouse {
            obstacles: warehouse_obstacles,
            robot,
        };
        Ok(warehouse)
    }
}

impl Display for LargeWarehouse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (y, row) in self.obstacles.iter().enumerate() {
            for (x, obstacle) in row.iter().enumerate() {
                if self.robot.x == x && self.robot.y == y {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", obstacle.to_string())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut section_iter = input.split("\n\n");
    let warehouse_str = section_iter.next().unwrap();
    let robot_movement_str = section_iter.next().unwrap();

    let mut warehouse: Warehouse = warehouse_str.parse().unwrap();
    let movements: RobotMovements = robot_movement_str.parse().unwrap();

    warehouse.simulate_robot_movements(movements);
    Some(warehouse.sum_box_coords())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut section_iter = input.split("\n\n");
    let warehouse_str = section_iter.next().unwrap();
    let robot_movement_str = section_iter.next().unwrap();

    let large_warehouse_str = warehouse_str
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");

    let mut warehouse: LargeWarehouse = large_warehouse_str.parse().unwrap();
    let movements: RobotMovements = robot_movement_str.parse().unwrap();

    warehouse.simulate_robot_movements(movements);
    Some(warehouse.sum_box_coords())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
