use std::collections::{HashSet, VecDeque};

use crate::JUMPS;

/// # Panics
#[must_use]
pub fn solve<const MOVES: usize>(data: &str) -> u32 {
    let board = data.as_bytes();
    let columns = board.iter().position(|c| *c == b'\n').unwrap();
    let rows = (board.len() + 1) / (columns + 1);

    let start = board
        .chunks(columns + 1)
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .take(columns)
                .enumerate()
                .find_map(|(c, tile)| if *tile == b'D' { Some((r, c)) } else { None })
        })
        .unwrap();

    let mut sheeps = 0;

    let mut visited = HashSet::with_capacity(columns * rows);
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((start, 0));
    while let Some(((row, column), moves)) = queue.pop_front() {
        if !visited.insert((row, column)) {
            continue;
        }

        sheeps += u32::from(board[row * (columns + 1) + column] == b'S');

        for (dr, dc) in JUMPS {
            match (row.checked_add_signed(dr), column.checked_add_signed(dc)) {
                (Some(new_row), Some(new_column))
                    if new_row < rows
                        && new_column < columns
                        && !visited.contains(&(new_row, new_column)) =>
                {
                    if moves < MOVES {
                        queue.push_back(((new_row, new_column), moves + 1));
                    }
                }
                _ => {}
            }
        }
    }

    sheeps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            solve::<3>(
                r"...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S.."
            ),
            27,
        );
    }
}
