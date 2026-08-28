//! [Story 4 Quest 3](https://everybody.codes/story/4/quests/3)

/// Errors returned from the `part*` implementations
#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    /// Invalid input data
    #[error("invalid input data")]
    InvalidInputData(&'a str),
}

/// Input data parsed
struct GridInfo {
    width: usize,
    height: usize,
    horizontal_offsets: Vec<usize>,
    vertical_offsets: Vec<usize>,
}

fn parse_digit(c: u8) -> Option<usize> {
    if c.is_ascii_digit() {
        Some(usize::from(c - b'0'))
    } else {
        None
    }
}

impl GridInfo {
    /// Parse input data
    ///
    /// # Arguments
    ///
    /// * data - input data
    ///
    /// # Results
    ///
    /// * [`GridInfo`]
    ///
    /// # Errors
    ///
    /// * [`Error::InvalidInputData`] - if the input data format is invalid
    fn parse(data: &str) -> Result<Self, Error<'_>> {
        let mut width = None;
        let mut height = None;
        let mut horizontal_offsets = None;
        let mut vertical_offsets = None;
        for line in data.lines() {
            if let Some(stripped_prefix) = line.strip_prefix("width=") {
                width = Some(
                    stripped_prefix
                        .parse::<usize>()
                        .map_err(|_| Error::InvalidInputData(line))?,
                );
            } else if let Some(stripped_prefix) = line.strip_prefix("height=") {
                height = Some(
                    stripped_prefix
                        .parse::<usize>()
                        .map_err(|_| Error::InvalidInputData(line))?,
                );
            } else if let Some(stripped_prefix) = line.strip_prefix("horizontal-offsets=") {
                horizontal_offsets = Some(stripped_prefix.as_bytes());
            } else if let Some(stripped_prefix) = line.strip_prefix("vertical-offsets=") {
                vertical_offsets = Some(stripped_prefix.as_bytes());
            } else {
                return Err(Error::InvalidInputData(line));
            }
        }

        let width = width.ok_or(Error::InvalidInputData(data))?;
        let height = height.ok_or(Error::InvalidInputData(data))?;
        let horizontal_offsets = horizontal_offsets
            .ok_or(Error::InvalidInputData(data))?
            .iter()
            .map(|d| parse_digit(*d).ok_or(Error::InvalidInputData(data)))
            .collect::<Result<Vec<_>, _>>()?;
        let vertical_offsets = vertical_offsets
            .ok_or(Error::InvalidInputData(data))?
            .iter()
            .map(|d| parse_digit(*d).ok_or(Error::InvalidInputData(data)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            width,
            height,
            horizontal_offsets,
            vertical_offsets,
        })
    }

    /// Returns the horizontal offset at `index`
    ///
    /// # Arguments
    ///
    /// * `index`
    ///
    /// # Returns
    ///
    /// * offset value at `index`    
    fn horizontal_offset(&self, index: usize) -> usize {
        self.horizontal_offsets[index % self.horizontal_offsets.len()]
    }

    /// Returns the vertical offset at `index`
    ///
    /// # Arguments
    ///
    /// * `index`
    ///
    /// # Returns
    ///
    /// * vertical value at `index`    
    fn vertical_offset(&self, index: usize) -> usize {
        self.vertical_offsets[index % self.vertical_offsets.len()]
    }

    /// Returns the counts of isolated tiles
    ///
    /// # Errors
    ///
    /// * [`std::num::TryFromIntError`] - if cannot convert `usize` to `u64`
    fn solve(&self) -> Result<[u64; 2], std::num::TryFromIntError> {
        let rows = self.horizontal_offsets.len() * 2;
        let columns = self.vertical_offsets.len() * 2;

        let mut total = [0, 0];

        let mut row_parity = 0;
        for y in 0..rows.min(self.height) {
            if y > 0 && self.horizontal_offset(y) == 0 {
                row_parity ^= 1;
            }

            if self.horizontal_offset(y) != self.horizontal_offset(y + 1) {
                continue;
            }

            let mut column_parity = 0;
            for x in 0..columns.min(self.width) {
                if x > 0 && self.vertical_offset(x) == y % 2 {
                    column_parity ^= 1;
                }

                if self.horizontal_offset(y) != x % 2
                    || self.vertical_offset(x) != y % 2
                    || self.vertical_offset(x + 1) != y % 2
                {
                    continue;
                }

                total[row_parity ^ column_parity] +=
                    u64::try_from((self.width - x).div_ceil(columns))?
                        * u64::try_from((self.height - y).div_ceil(rows))?;
            }
        }

        Ok(total)
    }

    /// Returns the counts of isolated tiles by coloring the grid
    /// (slow method)
    #[expect(unused, reason = "Here for historical reasons :-)")]
    fn solve_slow(&self) -> [u64; 2] {
        use std::collections::{HashMap, VecDeque};

        let mut grid = vec![vec![0u8; self.width]; self.height];

        let horizontal_offsets = self
            .horizontal_offsets
            .iter()
            .copied()
            .cycle()
            .take(self.height + 1)
            .collect::<Vec<_>>();
        for (offsets, row) in horizontal_offsets.windows(2).zip(grid.iter_mut()) {
            for (index, tile) in row.iter_mut().enumerate() {
                if index % 2 == offsets[0] {
                    *tile |= 0b1000;
                }
                if index % 2 == offsets[1] {
                    *tile |= 0b0100;
                }
            }
        }

        let vertical_offsets = self
            .vertical_offsets
            .iter()
            .copied()
            .cycle()
            .take(self.width + 1)
            .collect::<Vec<_>>();
        for (column, offsets) in vertical_offsets.windows(2).enumerate() {
            for (index, row) in grid.iter_mut().enumerate() {
                let tile = &mut row[column];
                if index % 2 == offsets[0] {
                    *tile |= 0b0010;
                }
                if index % 2 == offsets[1] {
                    *tile |= 0b0001;
                }
            }
        }

        let mut colors = HashMap::with_capacity(self.width * self.height);
        let mut queue = VecDeque::with_capacity(1024);
        queue.push_back(((0, 0), 0));

        while let Some(((x, y), color)) = queue.pop_front() {
            let std::collections::hash_map::Entry::Vacant(entry) = colors.entry((x, y)) else {
                continue;
            };
            entry.insert(color);

            let tile = grid[y][x];

            if x > 0 {
                queue.push_back(((x - 1, y), (color + usize::from(tile & 0b0010 != 0)) % 2));
            }
            if x + 1 < self.width {
                queue.push_back(((x + 1, y), (color + usize::from(tile & 0b0001 != 0)) % 2));
            }
            if y > 0 {
                queue.push_back(((x, y - 1), (color + usize::from(tile & 0b1000 != 0)) % 2));
            }
            if y + 1 < self.height {
                queue.push_back(((x, y + 1), (color + usize::from(tile & 0b0100 != 0)) % 2));
            }
        }

        let mut groups = [0, 0];
        for (y, row) in grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 0b1111 {
                    groups[colors[&(x, y)]] += 1;
                }
            }
        }

        groups
    }
}

/// Resolve part 1
///
/// # Errors
///
/// * [`Error::InvalidInputData`] - if the input data is invalid
pub fn part_1(data: &str) -> Result<u64, Error<'_>> {
    Ok(GridInfo::parse(data)?
        .solve()
        .map_err(|_| Error::InvalidInputData(data))?
        .iter()
        .sum())
}

/// Resolve part 2
///
/// # Errors
///
/// * [`Error::InvalidInputData`] - if the input data is invalid
pub fn part_2(data: &str) -> Result<u64, Error<'_>> {
    GridInfo::parse(data)?
        .solve()
        .map_err(|_| Error::InvalidInputData(data))?
        .into_iter()
        .max()
        .ok_or(Error::InvalidInputData(data))
}

/// Resolve part 3
///
/// # Errors
///
/// * [`Error::InvalidInputData`] - if the input data is invalid
pub fn part_3(data: &str) -> Result<u64, Error<'_>> {
    GridInfo::parse(data)?
        .solve()
        .map_err(|_| Error::InvalidInputData(data))?
        .into_iter()
        .max()
        .ok_or(Error::InvalidInputData(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"width=30
height=10
horizontal-offsets=10011
vertical-offsets=11011";
        assert_eq!(part_1(data).unwrap(), 27);
    }

    #[test]
    fn test_part_2_1() {
        let data = r"width=30
height=10
horizontal-offsets=10011
vertical-offsets=11011";
        assert_eq!(part_2(data).unwrap(), 15);
    }

    #[test]
    fn test_part_2_2() {
        let data = r"width=40
height=12
horizontal-offsets=11100
vertical-offsets=001101";
        assert_eq!(part_2(data).unwrap(), 7);
    }

    #[test]
    fn test_part_2_3() {
        let data = r"width=100
height=70
horizontal-offsets=111101111101101111000100100110
vertical-offsets=110100001110111011101000001111";
        assert_eq!(part_2(data).unwrap(), 269);
    }
}
