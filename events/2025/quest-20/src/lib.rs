use std::collections::VecDeque;
use std::{mem, ops};

trait Set {
    fn is_set(&self, p: (usize, usize)) -> bool;
    fn set(&mut self, p: (usize, usize), value: bool) -> bool;
}

trait Num
where
    Self: Copy,
    Self: ops::Shl<usize, Output = Self>
        + ops::BitAnd<Output = Self>
        + ops::BitOrAssign
        + ops::BitAndAssign
        + ops::Not<Output = Self>,
    Self: PartialEq,
{
    const BITS: usize;
    const ZERO: Self;
    const ONE: Self;
}

impl Num for u128 {
    const BITS: usize = const { mem::size_of::<u128>() * 8 };
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl<const ROWS: usize, T> Set for [[T; 2]; ROWS]
where
    T: Num,
{
    fn is_set(&self, (x, y): (usize, usize)) -> bool {
        let (v, mask) = if x >= T::BITS {
            (&self[y][1], { T::ONE } << (x - T::BITS))
        } else {
            (&self[y][0], { T::ONE } << x)
        };

        *v & mask != T::ZERO
    }

    fn set(&mut self, (x, y): (usize, usize), value: bool) -> bool {
        let (v, mask) = if x >= T::BITS {
            (&mut self[y][1], { T::ONE } << (x - T::BITS))
        } else {
            (&mut self[y][0], { T::ONE } << x)
        };

        let r = *v & mask != T::ZERO;
        if value {
            *v |= mask;
        } else {
            *v &= !mask;
        }
        r
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let data = data.as_bytes();
    let columns = data
        .iter()
        .position(|t| *t == b'\n')
        .expect("Invalid input");
    let rows = (data.len() + 1) / columns;

    let mut count = 0;
    for r in 0..rows {
        for w in data[r * (columns + 1) + r..r * (columns + 1) + columns - r].windows(2) {
            count += usize::from(w == b"TT");
        }
    }

    for c in 0..columns {
        if c <= columns / 2 {
            let mut i = (0..=c)
                .map(|r| data[r * (columns + 1) + c])
                .skip(usize::from(c % 2 == 0));
            while let (Some(a), Some(b)) = (i.next(), i.next()) {
                count += usize::from([a, b] == [b'T', b'T']);
            }
        } else {
            let mut i = (0..rows - (c - columns / 2))
                .map(|r| data[r * (columns + 1) + c])
                .skip(usize::from(c % 2 == 0));
            while let (Some(a), Some(b)) = (i.next(), i.next()) {
                count += usize::from([a, b] == [b'T', b'T']);
            }
        }
    }

    count
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let data = data.as_bytes();
    let columns = data
        .iter()
        .position(|t| *t == b'\n')
        .expect("Invalid input");
    let rows = (data.len() + 1) / columns;

    let start = (columns / 2, rows - 1);
    assert!(data[start.1 * (columns + 1) + start.0] == b'S');

    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((start, 0));
    let mut visited = [[0u128; 2]; 80];
    visited.set(start, true);
    while let Some(((x, y), cost)) = queue.pop_front() {
        let cost = cost + 1;
        for (dx, dy) in [(-1, 0), (1, 0), (0, if (x + y) % 2 == 0 { -1 } else { 1 })] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y)) if x < columns && y < rows && !visited.is_set((x, y)) => {
                    match data[y * (columns + 1) + x] {
                        b'E' => {
                            return cost;
                        }
                        b'T' => {
                            visited.set((x, y), true);
                            queue.push_back(((x, y), cost));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    unreachable!()
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let rot0 = data.as_bytes();
    let columns = rot0
        .iter()
        .position(|t| *t == b'\n')
        .expect("Invalid input");
    let rows = (rot0.len() + 1) / columns;

    let start = (columns / 2, rows - 1);
    assert!(rot0[start.1 * (columns + 1) + start.0] == b'S');

    let rot1 = &rotate(rot0, (columns, rows));
    let rot2 = &rotate(rot1, (columns, rows));

    let rot = [rot0, rot1, rot2];

    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((start, 0, 0));
    let mut visited = [[[0u128; 2]; 79]; 3];
    visited[0].set(start, true);
    while let Some(((x, y), r, cost)) = queue.pop_front() {
        let r = (r + 1) % 3;
        let cost = cost + 1;
        for (dx, dy) in [
            (-1, 0),
            (1, 0),
            (0, if (x + y) % 2 == 0 { -1 } else { 1 }),
            (0, 0),
        ] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y)) if x < columns && y < rows && !visited[r].is_set((x, y)) => {
                    match rot[r][y * (columns + 1) + x] {
                        b'E' => {
                            return cost;
                        }
                        b'T' => {
                            visited[r].set((x, y), true);
                            queue.push_back(((x, y), r, cost));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    unreachable!()
}

fn rotate(data: &[u8], (columns, rows): (usize, usize)) -> Vec<u8> {
    let mut rot = data.to_vec();
    for y in 0..rows {
        for (i, x) in (y..columns - y).step_by(2).enumerate() {
            rot[i * (columns + 1) + columns - 1 - i - 2 * y] = data[y * (columns + 1) + x];
        }
        for (i, x) in (y..columns - y).skip(1).step_by(2).enumerate() {
            rot[i * (columns + 1) + columns - 1 - i - 2 * y - 1] = data[y * (columns + 1) + x];
        }
    }
    rot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        assert_eq!(
            part_1(
                r"T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#....."
            ),
            7
        );
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(
            part_1(
                r"T#T#T#T#T#T
.T#T#T#T#T.
..T#T#T#T..
...T#T#T...
....T#T....
.....T....."
            ),
            0
        );
    }

    #[test]
    fn test_part_1_3() {
        assert_eq!(
            part_1(
                r"T#T#T#T#T#T
.#T#T#T#T#.
..#T###T#..
...##T##...
....#T#....
.....#....."
            ),
            0
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........"
            ),
            32
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S........."
            ),
            23
        );
    }
}
