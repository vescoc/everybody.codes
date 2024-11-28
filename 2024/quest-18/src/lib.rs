use std::collections::{HashSet, VecDeque};

use rayon::prelude::*;

#[must_use]
fn solve(
    data: &[u8],
    (width, height): (usize, usize),
    starts: impl Iterator<Item = (usize, usize)>,
    mut done: impl FnMut(&((usize, usize), u32)) -> Option<u32>,
) -> u32 {
    let mut visited = HashSet::with_capacity(data.len());
    let mut queue = VecDeque::new();

    for start in starts {
        visited.insert(start);
        queue.push_back((start, 0));
    }

    while let Some(((r, c), time)) = queue.pop_front() {
        if let Some(result) = done(&((r, c), time)) {
            return result;
        }
        visited.insert((r, c));

        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                (Some(r), Some(c)) if r < height && c < width && !visited.contains(&(r, c)) => {
                    if data[r * (width + 1) + c] != b'#' {
                        queue.push_back(((r, c), time + 1));
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
pub fn solve_1_2(data: &[u8]) -> u32 {
    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);

    let starts = data
        .chunks_exact(width + 1)
        .enumerate()
        .filter_map(|(r, row)| {
            if r == 0 || r == height - 1 {
                row.iter()
                    .enumerate()
                    .find_map(|(c, &tile)| if tile == b'.' { Some((r, c)) } else { None })
            } else if row[0] == b'.' {
                Some((r, 0))
            } else if row[width - 1] == b'.' {
                Some((r, width - 1))
            } else {
                None
            }
        });

    let mut palms = bytecount::count(data, b'P');
    solve(data, (width, height), starts, move |&((r, c), time)| {
        if data[r * (width + 1) + c] == b'P' {
            palms -= 1;
            if palms == 0 {
                return Some(time);
            }
        }
        None
    })
}

pub use solve_1_2 as part_1;
pub use solve_1_2 as part_2;

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> u32 {
    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);

    let holes = data
        .chunks_exact(width + 1)
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &tile)| if tile == b'.' { Some((r, c)) } else { None })
        })
        .par_bridge();

    let mut palms = bytecount::count(data, b'P');

    holes
        .map(|hole| {
            let mut total_time = 0;
            solve(
                data,
                (width, height),
                std::iter::once(hole),
                move |&((r, c), time)| {
                    if data[r * (width + 1) + c] == b'P' {
                        total_time += time;
                        palms -= 1;
                        if palms == 0 {
                            return Some(total_time);
                        }
                    }
                    None
                },
            )
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            11,
            part_1(
                br"##########
..#......#
#.P.####P#
#.#...P#.#
##########"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            21,
            part_2(
                br"#######################
...P..P...#P....#.....#
#.#######.#.#.#.#####.#
#.....#...#P#.#..P....#
#.#####.#####.#########
#...P....P.P.P.....P#.#
#.#######.#####.#.#.#.#
#...#.....#P...P#.#....
#######################"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            12,
            part_3(
                br"##########
#.#......#
#.P.####P#
#.#...P#.#
##########"
            )
        );
    }
}
