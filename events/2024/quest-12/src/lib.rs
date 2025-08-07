use std::collections::HashMap;

use rayon::prelude::*;

fn shots(t: u8) -> usize {
    match t {
        b'T' => 1,
        b'H' => 2,
        _ => unreachable!(),
    }
}

/// # Panics
#[must_use]
pub fn part_1_2(data: &[u8]) -> usize {
    let mut height = 0;
    let targets = data
        .split(|&c| c == b'\n')
        .enumerate()
        .flat_map(|(r, line)| {
            if line[0] == b'=' {
                height = r;
                vec![].into_iter()
            } else {
                line.iter()
                    .enumerate()
                    .filter_map(|(c, &v)| match v {
                        b'T' | b'H' => Some(((r, c - 1), v)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            }
        })
        .collect::<HashMap<_, _>>();

    targets
        .into_iter()
        .map(|((r, c), t)| {
            ((height - 3)..height)
                .enumerate()
                .find_map(|(k, height)| {
                    if r >= height {
                        let d = r - height;
                        if (c - d) % 3 == 0 {
                            return Some((c - d) / 3 * (3 - k) * shots(t));
                        }
                    } else {
                        let d = height - r;
                        if (c + d) % 3 == 0 {
                            return Some((c + d) / 3 * (3 - k) * shots(t));
                        }
                    }
                    None
                })
                .unwrap()
        })
        .sum()
}

pub use part_1_2 as part_1;
pub use part_1_2 as part_2;

fn parse(data: &[u8]) -> usize {
    data.iter().fold(0, |acc, &digit| {
        acc * 10 + u32::from(digit) as usize - u32::from(b'0') as usize
    })
}

pub use part_3_bf as part_3;

/// # Panics
#[must_use]
pub fn part_3_bf(data: &[u8]) -> usize {
    let meteors = data
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut parts = line.split(|&c| c == b' ');
            (parse(parts.next().unwrap()), parse(parts.next().unwrap()))
        })
        .collect::<Vec<_>>();

    let max = meteors.iter().map(|(x, _)| x).max().unwrap();

    let mut traces: [Vec<Vec<usize>>; 3] = [const { vec![] }; 3];
    for (l, traces) in traces.iter_mut().enumerate() {
        for p in 1..max / 2 {
            let mut trace = Vec::with_capacity(max / 2 + 1);

            trace.push(l);

            for y in 1..=p {
                trace.push(l + y);
            }
            for _ in 1..=p {
                trace.push(l + p);
            }
            for y in (0..l + p).rev() {
                trace.push(y);
            }

            traces.push(trace);
        }
    }

    meteors
        .par_iter()
        .map(|(x, y)| {
            let time = x / 2 + x % 2;

            assert!(*x >= time && *y >= time);

            let (mut current_x, mut current_y) = (x - time, y - time);

            let mut min = usize::MAX;
            let mut h_max = usize::MIN;
            loop {
                for (l, traces) in traces.iter().enumerate() {
                    for (p, trace) in traces.iter().enumerate() {
                        let score = (l + 1) * (p + 1);
                        if score > min {
                            break;
                        }

                        match trace.get(current_x) {
                            Some(&height) if height == current_y => {
                                if height > h_max {
                                    h_max = height;
                                    min = score;
                                } else if height == h_max && score < min {
                                    min = score;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if current_x > 0 && current_y > 0 {
                    current_x -= 1;
                    current_y -= 1;
                } else {
                    break;
                }
            }

            min
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            13,
            part_1(
                br".............
.C...........
.B......T....
.A......T.T..
============="
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            22,
            part_2(
                br".............
.C...........
.B......H....
.A......T.H..
============="
            )
        );
    }

    #[test]
    fn test_part_3_1_bf() {
        assert_eq!(
            11,
            part_3_bf(
                br"6 5
6 7
10 5"
            )
        );
    }

    #[test]
    fn test_part_3_2_bf() {
        assert_eq!(2, part_3_bf(br"5 5"));
    }
}
