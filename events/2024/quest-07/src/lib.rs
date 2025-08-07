use std::collections::HashSet;

use rayon::prelude::*;

pub const ROUND_2_TERRAIN: &[u8] =
    br"S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-";

pub const ROUND_3_TERRAIN: &[u8] =
    br"S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> String {
    let mut race = data
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut part = line.split(|&c| c == b':');
            let chariot = part.next().unwrap()[0];
            let value: i32 = part
                .next()
                .unwrap()
                .split(|&c| c == b',')
                .cycle()
                .scan(10, |state, segment| {
                    *state = match segment {
                        b"+" => *state + 1,
                        b"-" => (*state - 1).max(0),
                        b"=" => *state,
                        _ => unreachable!(),
                    };
                    Some(*state)
                })
                .take(10)
                .sum();
            (chariot, value)
        })
        .collect::<Vec<_>>();

    race.sort_unstable_by_key(|(_, value)| *value);

    race.iter()
        .rev()
        .map(|(chariot, _)| char::from(*chariot))
        .collect()
}

fn make_terrain(terrain: &[u8]) -> String {
    let width = terrain.iter().position(|&c| c == b'\n').unwrap();
    let height = (terrain.len() + 1) / (width + 1);

    let mut visited = HashSet::new();

    let (mut r, mut c) = (0, 1);
    let terrain = std::iter::from_fn(move || {
        let current = terrain[(width + 1) * r + c];

        if current == b'S' {
            visited.clear();
        } else {
            visited.insert((r, c));
        }

        (r, c) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .find_map(
                |(dr, dc)| match (r.checked_add_signed(*dr), c.checked_add_signed(*dc)) {
                    (Some(nr), _) if nr >= height => None,
                    (_, Some(nc)) if nc >= width => None,
                    (Some(nr), Some(nc)) => {
                        if visited.contains(&(nr, nc)) {
                            None
                        } else {
                            let tile = terrain[(width + 1) * nr + nc];
                            if tile == b' ' {
                                None
                            } else {
                                Some((nr, nc))
                            }
                        }
                    }
                    _ => None,
                },
            )
            .unwrap();

        Some(char::from(current))
    });

    terrain
        .take_while(|&c| c != 'S')
        .chain(std::iter::once('S'))
        .collect()
}

fn score_simple<const LAPS: usize>(line: impl Iterator<Item = char> + Clone, terrain: &str) -> u64 {
    let round_length = terrain.len();

    line.cycle()
        .zip(terrain.chars().cycle())
        .scan(10_u64, |state, (segment, terrain)| {
            *state = match (segment, terrain) {
                (_, '+') | ('+', '=' | 'S') => *state + 1,
                (_, '-') | ('-', '=' | 'S') => state.saturating_sub(1),
                _ => *state,
            };
            Some(*state)
        })
        .take(round_length * LAPS)
        .sum()
}

fn score<const LAPS: usize>(line: &[u8], terrain: &str) -> (char, u64) {
    let mut part = line.split(|&c| c == b':');

    let chariot = char::from(part.next().unwrap()[0]);

    let value = score_simple::<LAPS>(
        part.next()
            .unwrap()
            .split(|&c| c == b',')
            .map(|c| char::from(c[0])),
        terrain,
    );

    (chariot, value)
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8], terrain: &[u8]) -> String {
    let terrain = make_terrain(terrain);

    let mut race = data
        .par_split(|&c| c == b'\n')
        .map(|line| score::<10>(line, &terrain))
        .collect::<Vec<_>>();

    race.sort_unstable_by_key(|(_, value)| *value);

    race.iter().rev().map(|(chariot, _)| *chariot).collect()
}

#[allow(clippy::cast_sign_loss)]
fn score_fast(laps: usize, plan: &str, terrain: &str) -> u64 {
    let terrain_length = terrain.len();
    let plan_length = plan.len();

    let length = lcm(terrain_length, plan_length);

    let sums = plan
        .chars()
        .cycle()
        .zip(terrain.chars().cycle())
        .scan(0_i32, |state, (segment, terrain)| {
            *state = match (segment, terrain) {
                (_, '+') | ('+', '=' | 'S') => *state + 1,
                (_, '-') | ('-', '=' | 'S') => *state - 1,
                _ => *state,
            };
            Some(*state)
        })
        .take(length)
        .collect::<Vec<_>>();

    assert!(matches!(sums.last(), Some(v) if *v > 0));

    let s = sums.iter().sum::<i32>() as u64;

    let mut sum = 0_u64;
    let mut state = 10_u64;
    for _ in 0..laps * terrain_length / length {
        let new_state = state * length as u64 + s;

        assert!(new_state >= state);

        state = new_state;
        sum += state;
    }

    let remainder = laps * terrain_length % length;

    sum + state * remainder as u64 + sums[0..remainder].iter().sum::<i32>() as u64
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        let t = a;
        a = b % a;
        b = t;
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub use part_3_slow as part_3;

/// # Panics
#[must_use]
pub fn part_3_slow(data: &[u8], terrain: &[u8]) -> usize {
    const LAPS: usize = 11;

    let terrain = make_terrain(terrain);

    let target_score = score::<LAPS>(data, &terrain).1;

    let mut stack = vec![(String::new(), (5, 3, 3))];
    let permutations = std::iter::from_fn(move || {
        while let Some((result, (plus, minus, equals))) = stack.pop() {
            if plus > 0 {
                stack.push((result.clone() + "+", (plus - 1, minus, equals)));
            }
            if minus > 0 {
                stack.push((result.clone() + "-", (plus, minus - 1, equals)));
            }
            if equals > 0 {
                stack.push((result.clone() + "=", (plus, minus, equals - 1)));
            }
            if plus == 0 && minus == 0 && equals == 0 {
                return Some(result);
            }
        }
        None
    });

    permutations
        .par_bridge()
        .filter(|line| score_simple::<LAPS>(line.chars(), &terrain) > target_score)
        .count()
}

/// # Panics
#[must_use]
pub fn part_3_fast(data: &[u8], terrain: &[u8]) -> usize {
    const LAPS: usize = 11;

    let terrain = make_terrain(terrain);
    let plan = data
        .split(|&c| c == b':')
        .skip(1)
        .map(|data| {
            data.split(|&c| c == b',')
                .map(|c| char::from(c[0]))
                .collect::<String>()
        })
        .next()
        .unwrap();

    let target_score = score_fast(LAPS, &plan, &terrain);

    let mut stack = vec![(String::new(), (5, 3, 3))];
    let permutations = std::iter::from_fn(move || {
        while let Some((result, (plus, minus, equals))) = stack.pop() {
            if plus > 0 {
                stack.push((result.clone() + "+", (plus - 1, minus, equals)));
            }
            if minus > 0 {
                stack.push((result.clone() + "-", (plus, minus - 1, equals)));
            }
            if equals > 0 {
                stack.push((result.clone() + "=", (plus, minus, equals - 1)));
            }
            if plus == 0 && minus == 0 && equals == 0 {
                return Some(result);
            }
        }
        None
    });

    permutations
        .par_bridge()
        .filter(|line| score_fast(LAPS, line, &terrain) > target_score)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TERRAIN: &[u8] = br"S+===
-   +
=+=-+";

    #[test]
    fn test_part_1() {
        assert_eq!(
            "BDCA",
            part_1(
                br"A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "DCBA",
            part_2(
                br"A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+",
                SAMPLE_TERRAIN
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            6580,
            part_3(include_bytes!("../data/part_3"), ROUND_3_TERRAIN)
        );
    }

    #[test]
    fn test_same_result() {
        let plan = "-+++==+-=-+";
        let terrain = make_terrain(ROUND_3_TERRAIN);

        let target_score = score_simple::<11>(plan.chars(), &terrain);
        let candidate_score = score_fast(11, plan, &terrain);

        assert_eq!(target_score, candidate_score);
    }
}
