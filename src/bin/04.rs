advent_of_code::solution!(4);

#[derive(Clone, Eq, PartialEq)]
enum Char {
    X,
    M,
    A,
    S,
}

pub fn part_one(input: &str) -> Option<u32> {
    let horizontal = parse_char_matrix(input);
    let puzzle_size = horizontal.len();
    let puzzle_size_i32 = puzzle_size as i32;
    let vertical = (0..puzzle_size)
        .map(|col_index| {
            (0..puzzle_size)
                .map(|row_index| horizontal[row_index][col_index].clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let diag_right_down = ((-puzzle_size_i32)..puzzle_size_i32)
        .map(|row_index| {
            (row_index..(row_index + puzzle_size_i32))
                .zip(0..puzzle_size)
                .filter(|(row_index, _)| *row_index >= 0 && *row_index < puzzle_size_i32)
                .map(|(row_index, col_index)| horizontal[row_index as usize][col_index].clone())
                .collect::<Vec<_>>()
        })
        .filter(|diag| diag.len() >= 4)
        .collect::<Vec<_>>();
    let diag_right_up = (0..puzzle_size_i32 * 2)
        .map(|row_index| {
            ((row_index - puzzle_size_i32 + 1)..=row_index)
                .rev()
                .zip(0..puzzle_size)
                .filter(|(row_index, _)| *row_index >= 0 && *row_index < puzzle_size_i32)
                .map(|(row_index, col_index)| horizontal[row_index as usize][col_index].clone())
                .collect::<Vec<_>>()
        })
        .filter(|diag| diag.len() >= 4)
        .collect::<Vec<_>>();
    let all_directions_iter = horizontal
        .iter()
        .chain(&vertical)
        .chain(&diag_right_down)
        .chain(&diag_right_up);
    let counts = all_directions_iter.map(|line| -> u32 {
        let mut dfa_forward_state: Option<Char> = None;
        let mut dfa_reverse_state: Option<Char> = None;
        let mut count: u32 = 0;
        for char in line {
            match (dfa_forward_state, char) {
                (_, Char::X) => dfa_forward_state = Some(Char::X),
                (Some(Char::X), Char::M) => dfa_forward_state = Some(Char::M),
                (Some(Char::M), Char::A) => dfa_forward_state = Some(Char::A),
                (Some(Char::A), Char::S) => {
                    dfa_forward_state = None;
                    count += 1;
                }
                _ => dfa_forward_state = None,
            }
            match (dfa_reverse_state, char) {
                (_, Char::S) => dfa_reverse_state = Some(Char::S),
                (Some(Char::S), Char::A) => dfa_reverse_state = Some(Char::A),
                (Some(Char::A), Char::M) => dfa_reverse_state = Some(Char::M),
                (Some(Char::M), Char::X) => {
                    dfa_reverse_state = None;
                    count += 1;
                }
                _ => dfa_reverse_state = None,
            }
        }
        count
    });
    Some(counts.sum())
}

fn parse_char_matrix(input: &str) -> Vec<Vec<Char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|str| {
            str.chars()
                .map(|char| match char {
                    'X' => Char::X,
                    'M' => Char::M,
                    'A' => Char::A,
                    'S' => Char::S,
                    _ => {
                        panic!("Every character has to be one of: X, M, A, S")
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_two(_input: &str) -> Option<u32> {
    let char_matrix = parse_char_matrix(_input);
    let possible_x_center_coords = (1..(char_matrix.len() - 1))
        .flat_map(|row_index| {
            (1..(char_matrix.len() - 1))
                .map(move |col_index| (row_index.clone(), col_index))
        })
        .filter(|(row_index, col_index)| char_matrix[*row_index][*col_index] == Char::A)
        .collect::<Vec<_>>();
    Some(
        possible_x_center_coords.iter()
            .map(|(row_index, col_index)| {
                [
                    (*row_index - 1, *col_index - 1),
                    (*row_index - 1, *col_index + 1),
                    (*row_index + 1, col_index - 1),
                    (*row_index + 1, col_index + 1),
                ]
                    .iter()
                    .map(|(row_index, col_index)| char_matrix[*row_index][*col_index].clone())
                    .collect::<Vec<_>>()
            })
            .filter(|chars| chars.iter().all(|char| *char == Char::M || *char == Char::S))
            .filter(|chars| chars[0] != chars[3] && chars[1] != chars[2] )
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
