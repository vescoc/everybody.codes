use std::collections::{HashSet, VecDeque};

use crate::JUMPS;

/// # Panics
#[must_use]
pub fn solve<const MOVES: usize>(data: &str) -> u32 {
    let board = data.as_bytes();
    let columns = board.iter().position(|c| *c == b'\n').unwrap();
    let rows = (board.len() + 1) / (columns + 1);

    let mut start = None;
    let mut hides = HashSet::with_capacity(rows * columns);
    let mut sheeps = HashSet::with_capacity(rows * columns);
    for (r, row) in board.chunks(columns + 1).enumerate() {
        for (c, tile) in row.iter().take(columns).enumerate() {
            match *tile {
                b'D' => {
                    assert!(
                        start.is_none(),
                        "Duplicate Dragon, old: {start:?}, new: ({r}, {c})"
                    );
                    start.replace((r, c));
                }
                b'#' => {
                    hides.insert((r, c));
                }
                b'S' => {
                    sheeps.insert((r, c));
                }
                _ => {}
            }
        }
    }

    let mut sheeps_eaten = 0;

    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((start.expect("Cannot find Dragon"), 0));
    for current_move in 1..=MOVES {
        let mut visited = HashSet::with_capacity(columns * rows);
        while let Some(((row, column), moves)) = queue.pop_front() {
            if moves >= current_move {
                if moves == current_move {
                    if !visited.insert((row, column)) {
                        continue;
                    }

                    if !hides.contains(&(row, column)) && sheeps.remove(&(row, column)) {
                        sheeps_eaten += 1;
                    }
                } else {
                    queue.push_back(((row, column), moves));
                    break;
                }
            }

            for (dr, dc) in JUMPS {
                match (row.checked_add_signed(dr), column.checked_add_signed(dc)) {
                    (Some(new_row), Some(new_column))
                        if new_row < rows
                            && new_column < columns
                            && !visited.contains(&(new_row, new_column)) =>
                    {
                        queue.push_back(((new_row, new_column), moves + 1));
                    }
                    _ => {}
                }
            }
        }

        sheeps = sheeps
            .into_iter()
            .filter_map(|(row, column)| {
                let new_row = row + 1;
                if new_row == rows {
                    return None;
                }

                if !hides.contains(&(new_row, column)) && visited.contains(&(new_row, column)) {
                    sheeps_eaten += 1;
                    return None;
                }

                Some((new_row, column))
            })
            .collect();
    }

    sheeps_eaten
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        assert_eq!(
            solve::<3>(
                r"...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S.."
            ),
            27,
        );
    }
}
