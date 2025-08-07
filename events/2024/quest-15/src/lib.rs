use std::collections::VecDeque;

type Point = (usize, usize);

/// # Panics
#[must_use]
pub fn solve(data: &[u8]) -> usize {
    let map = data.split(|&c| c == b'\n').collect::<Vec<_>>();
    let width = map[0].len();
    let height = map.len();

    let start: Point = (0, map[0].iter().position(|&c| c == b'.').unwrap());

    let target = data.iter().fold(0_u32, |acc, &c| {
        if c.is_ascii_uppercase() {
            acc | 1 << (c - b'A')
        } else {
            acc
        }
    });

    let key = |(r, c), target| target as usize * width * height + r * width + c;

    let mut visited = vec![false; height * width * (target as usize + 1)];
    visited[key(start, target)] = true;

    let mut queue = VecDeque::new();
    queue.push_back((start, 0, target));

    while let Some(((r, c), distance, target)) = queue.pop_front() {
        if target == 0 && start.0 == r && start.1 == c {
            return distance;
        }

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .filter_map(
                |(dr, dc)| match (r.checked_add_signed(*dr), c.checked_add_signed(*dc)) {
                    (Some(r), Some(c)) => match map.get(r).and_then(|row| row.get(c)) {
                        _ if visited[key((r, c), target)] => None,
                        Some(tile) if tile.is_ascii_uppercase() => {
                            visited[key((r, c), target)] = true;

                            if target & (1 << (tile - b'A')) != 0 {
                                let target = target ^ (1 << (tile - b'A'));
                                Some(((r, c), target))
                            } else {
                                Some(((r, c), target))
                            }
                        }
                        Some(b'.') => {
                            visited[key((r, c), target)] = true;
                            Some(((r, c), target))
                        }
                        _ => None,
                    },
                    _ => None,
                },
            )
            .for_each(|(next, target)| {
                queue.push_back((next, distance + 1, target));
            });
    }

    unreachable!()
}

pub use solve as part_1;
pub use solve as part_2;
pub use solve as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            26,
            solve(
                br"#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            38,
            solve(
                br"##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################"
            )
        );
    }
}
