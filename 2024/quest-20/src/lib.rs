use std::collections::{HashMap, VecDeque};

use rayon::prelude::*;

struct Info1 {
    time: usize,
    position: (usize, usize),
    direction: (isize, isize),
    altitude: usize,
}

struct Info2 {
    time: usize,
    position: (usize, usize),
    direction: (isize, isize),
    altitude: usize,
    collected: u8,
}

fn next((dr, dc): (isize, isize)) -> [(isize, isize); 3] {
    match (dr, dc) {
        (0, -1) => [(1, 0), (-1, 0), (0, -1)],
        (0, 1) => [(1, 0), (-1, 0), (0, 1)],
        (1, 0) => [(1, 0), (0, 1), (0, -1)],
        (-1, 0) => [(-1, 0), (0, 1), (0, -1)],
        _ => unreachable!(),
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> usize {
    const ALTITUDE: usize = 1000;

    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);

    let start_position = (0, data.iter().position(|&c| c == b'S').unwrap());

    let mut visited = HashMap::from([((start_position, (1, 0)), ALTITUDE)]);
    let mut results = [usize::MIN; 100];
    results[0] = ALTITUDE;

    let mut queue = VecDeque::new();
    queue.push_back(Info1 {
        time: 0,
        position: start_position,
        direction: (1, 0),
        altitude: ALTITUDE,
    });

    let mut result = usize::MIN;

    while let Some(Info1 {
        time,
        position: (r, c),
        direction,
        altitude,
    }) = queue.pop_front()
    {
        if time == 100 {
            result = result.max(altitude);
            continue;
        }

        if altitude + (100 - time) < results[time] {
            continue;
        }

        results[time] = results[time].max(altitude);

        for (dr, dc) in next(direction) {
            let Some(((r, c), altitude)) =
                (match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                    (Some(r), Some(c)) if r < height && c < width => {
                        match data[r * (width + 1) + c] {
                            b'.' | b'S' => {
                                altitude.checked_sub(1).map(|altitude| ((r, c), altitude))
                            }
                            b'-' => altitude.checked_sub(2).map(|altitude| ((r, c), altitude)),
                            b'+' => altitude.checked_add(1).map(|altitude| ((r, c), altitude)),
                            b'#' => None,
                            _ => unreachable!(),
                        }
                    }
                    _ => None,
                })
            else {
                continue;
            };

            let e = visited.entry(((r, c), (dr, dc))).or_default();
            if altitude > *e {
                *e = altitude;
                queue.push_back(Info1 {
                    time: time + 1,
                    position: (r, c),
                    direction: (dr, dc),
                    altitude,
                });
            }
        }
    }

    result
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> usize {
    const ALTITUDE: usize = 10000;

    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);

    let start_position = (0, data.iter().position(|&c| c == b'S').unwrap());

    let mut visited = HashMap::from([((start_position, (1, 0), b'S'), ALTITUDE)]);

    let mut queue = VecDeque::new();
    queue.push_back(Info2 {
        time: 0,
        position: start_position,
        direction: (1, 0),
        altitude: ALTITUDE,
        collected: b'S',
    });

    while let Some(Info2 {
        time,
        position: (r, c),
        direction,
        altitude,
        collected,
    }) = queue.pop_front()
    {
        for (dr, dc) in next(direction) {
            let Some(((r, c), altitude, collected)) =
                (match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                    (Some(r), Some(c)) if r < height && c < width => {
                        match data[r * (width + 1) + c] {
                            b'.' => altitude
                                .checked_sub(1)
                                .map(|altitude| ((r, c), altitude, collected)),
                            b'-' => altitude
                                .checked_sub(2)
                                .map(|altitude| ((r, c), altitude, collected)),
                            b'+' => altitude
                                .checked_add(1)
                                .map(|altitude| ((r, c), altitude, collected)),
                            b'S' => {
                                if (r, c) == start_position
                                    && altitude > ALTITUDE
                                    && collected == b'C'
                                {
                                    return time + 1;
                                }

                                altitude
                                    .checked_sub(1)
                                    .map(|altitude| ((r, c), altitude, collected))
                            }
                            b'A' if collected == b'S' => altitude
                                .checked_sub(1)
                                .map(|altitude| ((r, c), altitude, b'A')),
                            b'B' if collected == b'A' => altitude
                                .checked_sub(1)
                                .map(|altitude| ((r, c), altitude, b'B')),
                            b'C' if collected == b'B' => altitude
                                .checked_sub(1)
                                .map(|altitude| ((r, c), altitude, b'C')),
                            b'A' | b'B' | b'C' | b'#' => None,
                            _ => unreachable!(),
                        }
                    }
                    _ => None,
                })
            else {
                continue;
            };

            let e = visited.entry(((r, c), (dr, dc), collected)).or_default();
            if altitude > *e {
                *e = altitude;
                queue.push_back(Info2 {
                    time: time + 1,
                    position: (r, c),
                    direction: (dr, dc),
                    altitude,
                    collected,
                });
            }
        }
    }

    unreachable!()
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_3(data: &[u8]) -> usize {
    const ALTITUDE: usize = 384400;

    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);

    let alt = |(r, c), altitude: usize| match data[(r % height) * (width + 1) + c] {
        b'.' | b'S' => Ok(altitude.checked_sub(1)),
        b'-' => Ok(altitude.checked_sub(2)),
        b'+' => Ok(altitude.checked_add(1)),
        _ => Err("invalid tile"),
    };

    let start_column = data.iter().position(|&c| c == b'S').unwrap();
    (0..width)
        .par_bridge()
        .filter_map(|target_column| {
            let mut altitude = ALTITUDE;
            let dc = if start_column > target_column { -1 } else { 1 };
            let (mut r, mut c) = (0, start_column);
            while c != target_column {
                c = c.checked_add_signed(dc)?;
                altitude = alt((r, c), altitude).ok()??;
            }

            loop {
                r += 1;
                match alt((r, c), altitude) {
                    Ok(Some(alt)) => altitude = alt,
                    Ok(None) => return Some(r - 1),
                    Err(_) => return None,
                }
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            1045,
            part_1(
                br"#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#"
            )
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            24,
            part_2(
                br"####S####
#-.+++.-#
#.+.+.+.#
#-.+.+.-#
#A+.-.+C#
#.+-.-+.#
#.+.B.+.#
#########"
            )
        );
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(
            78,
            part_2(
                br"###############S###############
#+#..-.+.-++.-.+.--+.#+.#++..+#
#-+-.+-..--..-+++.+-+.#+.-+.+.#
#---.--+.--..++++++..+.-.#.-..#
#+-+.#+-.#-..+#.--.--.....-..##
#..+..-+-.-+.++..-+..+#-.--..-#
#.--.A.-#-+-.-++++....+..C-...#
#++...-..+-.+-..+#--..-.-+..-.#
#..-#-#---..+....#+#-.-.-.-+.-#
#.-+.#+++.-...+.+-.-..+-++..-.#
##-+.+--.#.++--...-+.+-#-+---.#
#.-.#+...#----...+-.++-+-.+#..#
#.---#--++#.++.+-+.#.--..-.+#+#
#+.+.+.+.#.---#+..+-..#-...---#
#-#.-+##+-#.--#-.-......-#..-##
#...+.-+..##+..+B.+.#-+-++..--#
###############################"
            )
        );
    }

    #[test]
    fn test_part_2_3() {
        assert_eq!(
            206,
            part_2(
                br"###############S###############
#-----------------------------#
#-------------+++-------------#
#-------------+++-------------#
#-------------+++-------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#--A-----------------------C--#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#-----------------------------#
#--------------B--------------#
#-----------------------------#
#-----------------------------#
###############################"
            )
        );
    }

    #[test]
    #[ignore]
    fn test_part_3() {
        assert_eq!(
            768790,
            part_3(
                br"#......S......#
#-...+...-...+#
#.............#
#..+...-...+..#
#.............#
#-...-...+...-#
#.............#
#..#...+...+..#"
            )
        );
    }
}
