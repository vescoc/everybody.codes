use rayon::prelude::*;

use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i32, i32, i32);

fn parse(data: &[u8]) -> i32 {
    data.iter()
        .fold(0, |acc, &c| acc * 10 + i32::from(c - b'0'))
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> i32 {
    data.split(|&c| c == b',')
        .filter_map(|m| match m[0] {
            b'U' => Some(parse(&m[1..])),
            b'D' => Some(-parse(&m[1..])),
            _ => None,
        })
        .scan(0, |acc, value| {
            *acc += value;
            Some(*acc)
        })
        .max()
        .unwrap()
}

fn segments(data: &[u8], mut handle_leaf: impl FnMut(&Point)) -> HashSet<Point> {
    let mut segments = HashSet::new();
    for row in data.split(|&c| c == b'\n') {
        let leaf = row
            .split(|&c| c == b',')
            .map(|m| (m[0], parse(&m[1..])))
            .scan((0, 0, 0), |acc, (direction, len)| {
                let (dx, dy, dz) = match direction {
                    b'U' => (0, 1, 0),
                    b'D' => (0, -1, 0),
                    b'R' => (1, 0, 0),
                    b'L' => (-1, 0, 0),
                    b'F' => (0, 0, 1),
                    b'B' => (0, 0, -1),
                    _ => unreachable!("invalid direction {direction}"),
                };

                Some(
                    std::iter::once(*acc)
                        .chain((0..len).map(move |_| {
                            acc.0 += dx;
                            acc.1 += dy;
                            acc.2 += dz;
                            *acc
                        }))
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .inspect(|leaf| {
                segments.insert(*leaf);
            })
            .last()
            .unwrap();

        handle_leaf(&leaf);
    }

    segments
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> usize {
    segments(data, |_| {}).len() - 1
}

fn bsf(segments: &HashSet<Point>, start: &Point) -> HashMap<Point, usize> {
    let mut distances = HashMap::with_capacity(segments.len());

    let mut visited = HashSet::with_capacity(segments.len());
    visited.insert(*start);

    let mut queue = VecDeque::with_capacity(segments.len());
    queue.push_back((*start, 0));

    while let Some(((x, y, z), distance)) = queue.pop_front() {
        for (dx, dy, dz) in [
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let p = (x + dx, y + dy, z + dz);
            if !visited.contains(&p) && segments.contains(&p) {
                visited.insert(p);
                queue.push_back((p, distance + 1));
                distances.insert(p, distance + 1);
            }
        }
    }

    distances
}

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> usize {
    let mut leaves = HashSet::new();
    let segments = segments(data, |leaf| {
        leaves.insert(*leaf);
    });

    let trunk = {
        let mut i = 0;
        let segments = &segments;
        std::iter::from_fn(move || {
            let p = (0, i, 0);
            i += 1;
            if segments.contains(&p) {
                Some(p)
            } else {
                None
            }
        })
        .fuse()
    };

    trunk
        .par_bridge()
        .map(|current| {
            let distances = bsf(&segments, &current);

            leaves.iter().filter_map(|leaf| distances.get(leaf)).sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(7, part_1(br"U5,R3,D2,L5,U4,R5,D2"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            32,
            part_2(
                br"U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1"
            )
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            5,
            part_3(
                br"U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1"
            )
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            46,
            part_3(
                br"U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1"
            )
        );
    }
}
