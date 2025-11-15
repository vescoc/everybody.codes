use std::collections::HashMap;

use crate::JUMPS;

type Sheeps = u64;
type Hides = u64;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Turn {
    Sheeps,
    Dragon,
}

struct Board {
    rows: usize,
    columns: usize,
    hides: Hides,
}

impl Board {
    fn eat(&self, sheeps: Sheeps, (row, column): (usize, usize)) -> Option<Sheeps> {
        let mask = 1 << (row * self.columns + column);
        if (sheeps & !self.hides) & mask != 0 {
            Some(sheeps & !mask)
        } else {
            None
        }
    }

    fn sheep_positions<'a>(
        &self,
        buffer: &'a mut [(usize, usize)],
        sheeps: Sheeps,
    ) -> &'a [(usize, usize)] {
        let mut index = 0;
        for row in 0..self.rows {
            for column in 0..self.columns {
                if sheeps & (1 << (row * self.columns + column)) != 0 {
                    buffer[index] = (row, column);
                    index += 1;
                }
            }
        }

        &buffer[0..index]
    }

    fn sheep_escaped(&self, (mut row, column): (usize, usize)) -> bool {
        while row < self.rows {
            if self.hides & (1 << (row * self.columns + column)) == 0 {
                return false;
            }

            row += 1;
        }

        true
    }

    fn move_sheep(
        &self,
        sheeps: Sheeps,
        (start_row, start_column): (usize, usize),
        (end_row, end_column): (usize, usize),
    ) -> Sheeps {
        (sheeps & !(1 << (start_row * self.columns + start_column)))
            | (1 << (end_row * self.columns + end_column))
    }

    fn is_hide(&self, (row, column): (usize, usize)) -> bool {
        self.hides & (1 << (row * self.columns + column)) != 0
    }
}

fn unique_sequences(
    memoize: &mut HashMap<(Turn, Sheeps, (usize, usize)), u64>,
    board: &Board,
    turn: Turn,
    sheeps: Sheeps,
    dragon: (usize, usize),
) -> u64 {
    let key = (turn, sheeps, dragon);
    if let Some(result) = memoize.get(&key) {
        return *result;
    }

    let result = if sheeps == 0 {
        1
    } else {
        match turn {
            Turn::Dragon => JUMPS
                .iter()
                .map(|&(dr, dc)| {
                    match (
                        dragon.0.checked_add_signed(dr),
                        dragon.1.checked_add_signed(dc),
                    ) {
                        (Some(new_row), Some(new_column))
                            if new_row < board.rows && new_column < board.columns =>
                        {
                            board
                                .eat(sheeps, (new_row, new_column))
                                .map(|sheeps| {
                                    unique_sequences(
                                        memoize,
                                        board,
                                        Turn::Sheeps,
                                        sheeps,
                                        (new_row, new_column),
                                    )
                                })
                                .unwrap_or_else(|| {
                                    unique_sequences(
                                        memoize,
                                        board,
                                        Turn::Sheeps,
                                        sheeps,
                                        (new_row, new_column),
                                    )
                                })
                        }
                        _ => 0,
                    }
                })
                .sum(),
            Turn::Sheeps => {
                let hide = board.is_hide(dragon);

                let mut result = 0;

                let mut found_move = false;

                let mut buffer = [(0, 0); 8];
                let positions = board.sheep_positions(&mut buffer, sheeps);
                for start in positions {
                    let end = (start.0 + 1, start.1);
                    if !hide && end == dragon {
                        continue;
                    }

                    found_move = true;
                    if board.sheep_escaped(end) {
                        continue;
                    }

                    let sheeps = board.move_sheep(sheeps, *start, end);
                    result += unique_sequences(memoize, board, Turn::Dragon, sheeps, dragon);
                }

                if !found_move && positions.len() == 1 {
                    result += unique_sequences(memoize, board, Turn::Dragon, sheeps, dragon);
                }

                result
            }
        }
    };

    memoize.insert(key, result);

    result
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> u64 {
    let board = data.as_bytes();
    let columns = board.iter().position(|c| *c == b'\n').unwrap();
    let rows = (board.len() + 1) / (columns + 1);

    assert!(rows * columns < 64 && columns < 8);

    let mut start = None;
    let mut hides = 0;
    let mut sheeps = 0;
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
                    hides |= 1 << (r * columns + c);
                }
                b'S' => {
                    sheeps |= 1 << (r * columns + c);
                }
                _ => {}
            }
        }
    }

    let board = Board {
        rows,
        columns,
        hides,
    };

    let mut memoize = HashMap::with_capacity(200 * 1024);
    unique_sequences(
        &mut memoize,
        &board,
        Turn::Sheeps,
        sheeps,
        start.expect("Cannot find Dragon"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_moves() {
        assert_eq!(
            solve(
                r"S.
..
#.
#D"
            ),
            1,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            solve(
                r"SSS
..#
#.#
#D."
            ),
            15,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            solve(
                r"SSS
..#
..#
.##
.D#"
            ),
            8,
        );
    }

    #[test]
    fn test_part_3_3() {
        assert_eq!(
            solve(
                r"..S..
.....
..#..
.....
..D.."
            ),
            44,
        );
    }

    #[test]
    fn test_part_3_4() {
        assert_eq!(
            solve(
                r".SS.S
#...#
...#.
##..#
.####
##D.#"
            ),
            4406,
        );
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_3_5() {
        assert_eq!(
            solve(
                r"SSS.S
.....
#.#.#
.#.#.
#.D.#"
            ),
            13033988838,
        );
    }
}
