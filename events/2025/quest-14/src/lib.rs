use std::{mem, ops};

trait CountOnes {
    fn count_ones(&self) -> usize;
}

impl CountOnes for u16 {
    fn count_ones(&self) -> usize {
        Self::count_ones(*self) as usize
    }
}

impl CountOnes for u64 {
    fn count_ones(&self) -> usize {
        Self::count_ones(*self) as usize
    }
}

trait Set {
    fn set(&mut self, coordinate: (usize, usize), value: bool);
    fn len(&self) -> usize;
}

trait TZero {
    const ZERO: Self;
}

impl TZero for u16 {
    const ZERO: Self = 0;
}

impl TZero for u64 {
    const ZERO: Self = 0;
}

trait TProperties
where
    Self: ops::BitAndAssign<Self>
        + ops::BitOrAssign<Self>
        + ops::Shl<usize, Output = Self>
        + ops::Not<Output = Self>,
    Self: Copy,
    Self: CountOnes,
    Self: From<u16>,
{
}

impl<T> TProperties for T
where
    T: ops::BitAndAssign<T>
        + ops::BitOrAssign<T>
        + ops::Shl<usize, Output = T>
        + ops::Not<Output = T>,
    T: Copy,
    T: CountOnes,
    T: From<u16>,
{
}

impl<const SIZE: usize, T: TProperties> Set for [T; SIZE] {
    fn set(&mut self, (row, column): (usize, usize), value: bool) {
        if value {
            self[row] |= T::from(1) << column;
        } else {
            self[row] &= !(T::from(1) << column);
        }
    }

    fn len(&self) -> usize {
        self.iter().map(CountOnes::count_ones).sum()
    }
}

/// # Panics
#[must_use]
fn solve<const ROUNDS: usize, const ROWS: usize, T>(data: &str) -> usize
where
    T: TProperties,
    T: TZero,
    T: ops::BitAnd<T, Output = T> + ops::Shr<usize, Output = T> + ops::BitXor<T, Output = T>,
{
    let mut tiles = [T::ZERO; ROWS];
    let mut rows = 0;
    let mut columns = 0;
    for line in data.lines() {
        columns = 0;
        for tile in line.chars() {
            if tile == '#' {
                tiles.set((rows, columns), true);
            }
            columns += 1;
        }
        rows += 1;
    }

    let (rows, columns) = (rows, columns);
    let mask = !T::ZERO >> (mem::size_of::<T>() * 8 - columns);

    let mut sum = 0;
    for _ in 0..ROUNDS {
        let mut new_tiles = tiles;
        for row in 0..rows {
            let previous_row = if row > 0 { tiles[row - 1] } else { T::ZERO };
            let current_row = tiles[row];
            let next_row = if row + 1 < rows {
                tiles[row + 1]
            } else {
                T::ZERO
            };

            new_tiles[row] = !(previous_row << 1
                ^ previous_row >> 1
                ^ current_row
                ^ next_row << 1
                ^ next_row >> 1)
                & mask;
        }

        tiles = new_tiles;

        sum += tiles.len();
    }

    sum
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    solve::<10, 10, u16>(data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    solve::<2025, 34, u64>(data)
}

const ROWS: usize = 34;
const COLUMNS: usize = 34;
const PATTERN_ROWS: usize = 8;
const PATTERN_COLUMNS: usize = 8;
const EMPTY_SET: [u64; ROWS] = [0u64; ROWS];
const MASK: u64 = !0u64 >> (64 - COLUMNS);

#[allow(clippy::unreadable_literal)]
const ROUNDS: usize = 1000000000;

const GENERATIONS: &[[u64; ROWS]; 4097] = &const {
    let mut list = [[0u64; 34]; 4097];

    let mut index = 1;
    while index < list.len() {
        let mut row = 0;
        while row < ROWS {
            let tiles = &list[index - 1];
            let previous_row = if row > 0 { tiles[row - 1] } else { 0 };
            let current_row = tiles[row];
            let next_row = if row + 1 < ROWS { tiles[row + 1] } else { 0 };

            list[index][row] = !((previous_row << 1)
                ^ (previous_row >> 1)
                ^ current_row
                ^ (next_row << 1)
                ^ (next_row >> 1))
                & MASK;

            row += 1;
        }

        index += 1;
    }

    list
};

const fn equals_tiles(a: &[u64; ROWS], b: &[u64; ROWS]) -> bool {
    let mut index = 0;
    while index < ROWS {
        if a[index] != b[index] {
            return false;
        }
        index += 1;
    }

    true
}

const fn cycle_info() -> (usize, usize) {
    let mut start_index = 0;
    while start_index < GENERATIONS.len() - 1 {
        let tiles = &GENERATIONS[start_index];
        let mut end_index = start_index + 1;
        while end_index < GENERATIONS.len() {
            if equals_tiles(tiles, &GENERATIONS[end_index]) {
                return (start_index, (end_index - start_index));
            }

            end_index += 1;
        }

        start_index += 1;
    }

    unreachable!()
}

const CYCLE_INFO: (usize, usize) = cycle_info();

/// # Panics
#[must_use]
#[allow(clippy::large_stack_arrays)]
pub fn part_3(data: &str) -> usize {
    let (start_cycle, cycle_length) = CYCLE_INFO;

    let mut mask = EMPTY_SET;
    let mut target = EMPTY_SET;
    for (row, line) in data.lines().enumerate() {
        for (column, tile) in line.chars().enumerate() {
            if tile == '#' {
                target.set(
                    (
                        row + (ROWS - PATTERN_ROWS) / 2,
                        column + (COLUMNS - PATTERN_COLUMNS) / 2,
                    ),
                    true,
                );
            }

            mask.set(
                (
                    row + (ROWS - PATTERN_ROWS) / 2,
                    column + (COLUMNS - PATTERN_COLUMNS) / 2,
                ),
                true,
            );
        }
    }

    let cycles = (ROUNDS - start_cycle) / cycle_length;
    let remainder = (ROUNDS - start_cycle) % cycle_length;

    let mut partial_sum = 0;
    for tiles in &GENERATIONS[start_cycle..start_cycle + remainder] {
        if tiles
            .iter()
            .zip(mask)
            .zip(target)
            .all(|((a, m), b)| a & m == b)
        {
            partial_sum += tiles.len();
        }
    }

    let mut remainder_sum = 0;
    for tiles in &GENERATIONS[start_cycle + remainder..start_cycle + cycle_length] {
        if tiles
            .iter()
            .zip(mask)
            .zip(target)
            .all(|((a, m), b)| a & m == b)
        {
            remainder_sum += tiles.len();
        }
    }

    (partial_sum + remainder_sum) * cycles + partial_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r".#.##.
##..#.
..##.#
.#.##.
.###..
###.##"
            ),
            200
        );
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"#......#
..#..#..
.##..##.
...##...
...##...
.##..##.
..#..#..
#......#"
            ),
            278388552
        );
    }
}
