use std::collections::HashMap;

type Snapshot = Vec<((usize, usize), (isize, isize))>;

type Nullghost = ((usize, usize), (isize, isize), u64);
type Nullghosts = (usize, [Nullghost; 64]);
type Scratch = [Nullghosts; 2];

fn scratch_get<'a>(
    current_scratch: &mut usize,
    scratch: &'a mut Scratch,
) -> (&'a mut Nullghosts, &'a mut Nullghosts) {
    let (a, b) = if *current_scratch == 0 {
        let (a, b) = scratch.split_first_mut().unwrap();
        (a, &mut b[0])
    } else {
        let (a, b) = scratch.split_last_mut().unwrap();
        (a, &mut b[0])
    };
    *current_scratch = (*current_scratch + 1) % 2;

    (a, b)
}

fn scratch_current<'a>(
    current_scratch: &mut usize,
    scratch: &'a mut Scratch,
) -> &'a mut Nullghosts {
    if *current_scratch == 0 {
        scratch.first_mut().unwrap()
    } else {
        scratch.last_mut().unwrap()
    }
}

const fn wall_collision((dx, dy): (isize, isize)) -> (isize, isize) {
    (-dy, dx)
}

const fn nullghosts_collision((dx, dy): (isize, isize)) -> (isize, isize) {
    (-dx, -dy)
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let map = data.as_bytes();

    let mut width = usize::MAX;
    let mut height = 0;
    let mut nullghosts_current = map
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| {
            let position = (index % (width.saturating_add(1)), height);
            match tile {
                b'<' => Some((position, vec![(-1, 0)])),
                b'>' => Some((position, vec![(1, 0)])),
                b'^' => Some((position, vec![(0, -1)])),
                b'v' => Some((position, vec![(0, 1)])),
                b'.' | b'#' => None,
                b'\n' => {
                    if width == usize::MAX {
                        width = index;
                    }
                    height += 1;

                    None
                }
                _ => unreachable!("invalid tile '{tile}'"),
            }
        })
        .collect::<HashMap<_, _>>();
    height += usize::from(*map.last().unwrap() != b'\n');

    let mut collisions = 0;
    for _ in 0..10 {
        let mut nullghosts_next: HashMap<_, Vec<_>> =
            HashMap::with_capacity(nullghosts_current.len());

        // move nullghosts
        for ((x, y), nullghosts) in nullghosts_current {
            for (dx, dy) in nullghosts {
                match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    (Some(next_x), Some(next_y)) if x < width && y < height => {
                        let tile = map[next_y * (width + 1) + next_x];
                        let (position, direction) = if tile == b'#' {
                            collisions += 1;

                            ((x, y), wall_collision((dx, dy)))
                        } else {
                            ((next_x, next_y), (dx, dy))
                        };

                        nullghosts_next.entry(position).or_default().push(direction);
                    }
                    _ => unreachable!("out of map"),
                }
            }
        }

        // handle nullghosts collisions
        for nullghosts in nullghosts_next.values_mut() {
            let len = nullghosts.len();
            if len > 1 {
                collisions += len as u64;

                for direction in nullghosts.iter_mut() {
                    *direction = nullghosts_collision(*direction);
                }
            }
        }

        // replace with new state
        nullghosts_current = nullghosts_next;
    }

    collisions
}

fn handle_cycles<const COLLISIONS: u64>(
    seen: &mut HashMap<Snapshot, (u64, Vec<Nullghost>)>,
    nullghosts_current: &mut [Nullghost],
    round: &mut u64,
) {
    let key = nullghosts_current
        .iter()
        .map(|(position, direction, _)| (*position, *direction))
        .collect::<Vec<_>>();
    if let Some((old_round, old_nullghosts_current)) = seen.get(&key) {
        let cycle = *round - old_round;

        let mut max_rep = u64::MAX;
        for ((_, _, current_collisions), (_, _, old_collisions)) in
            nullghosts_current.iter().zip(old_nullghosts_current.iter())
        {
            let delta_collisions = *current_collisions - old_collisions;

            // minus 1 because I don't want to fade the ghost
            if let Some(rep) = (COLLISIONS - 1 - *current_collisions).checked_div(delta_collisions)
            {
                max_rep = max_rep.min(rep);
            }
        }

        if max_rep > 0 {
            for ((_, _, current_collisions), (_, _, old_collisions)) in nullghosts_current
                .iter_mut()
                .zip(old_nullghosts_current.iter())
            {
                let delta_collisions = *current_collisions - old_collisions;

                *current_collisions += delta_collisions * max_rep;
            }

            *round += max_rep * cycle;
        }
    }

    seen.insert(key, (*round, nullghosts_current.to_vec()));
}

/// # Panics
#[must_use]
pub fn part_2<const COLLISIONS: u64>(data: &str) -> u64 {
    let mut current_scratch = 0;
    let mut scratch = [(0usize, [((0usize, 0usize), (0isize, 0isize), 0u64); 64]); 2];

    let map = data.as_bytes();

    let (count, current) = scratch_current(&mut current_scratch, &mut scratch);

    let mut width = usize::MAX;
    let mut height = 0;

    *count = map
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| {
            let position = (index % (width.saturating_add(1)), height);
            match tile {
                b'<' => Some((position, (-1, 0), 0)),
                b'>' => Some((position, (1, 0), 0)),
                b'^' => Some((position, (0, -1), 0)),
                b'v' => Some((position, (0, 1), 0)),
                b'.' | b'#' => None,
                b'\n' => {
                    if width == usize::MAX {
                        width = index;
                    }
                    height += 1;

                    None
                }
                _ => unreachable!("invalid tile '{tile}'"),
            }
        })
        .zip(current.iter_mut())
        .map(|(value, element)| {
            *element = value;
        })
        .count();
    height += usize::from(*map.last().unwrap() != b'\n');

    let mut seen = HashMap::with_capacity(1024);

    let mut round = 0;

    let mut positions: HashMap<(usize, usize), usize> = HashMap::with_capacity(1024);
    loop {
        {
            let ((current_count, current), (next_count, next)) =
                scratch_get(&mut current_scratch, &mut scratch);
            if *current_count == 0 {
                break;
            }

            // handle cycles
            handle_cycles::<COLLISIONS>(&mut seen, &mut current[..*current_count], &mut round);

            // init next
            *next_count = 0;

            // move nullghosts
            for ((x, y), (dx, dy), mut collisions) in current.iter().take(*current_count).copied() {
                match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    (Some(next_x), Some(next_y)) if x < width && y < height => {
                        let tile = map[next_y * (width + 1) + next_x];
                        let (position, direction) = if tile == b'#' {
                            collisions += 1;

                            ((x, y), wall_collision((dx, dy)))
                        } else {
                            ((next_x, next_y), (dx, dy))
                        };

                        if collisions < COLLISIONS {
                            next[*next_count] = (position, direction, collisions);
                            *positions.entry(position).or_default() += 1;

                            *next_count += 1;
                        } else {
                            seen.clear();
                        }
                    }
                    _ => unreachable!("out of map"),
                }
            }
        }

        // handle nullghosts collisions
        {
            let ((current_count, current), (next_count, next)) =
                scratch_get(&mut current_scratch, &mut scratch);
            // init next
            *next_count = 0;

            for (position, direction, mut collisions) in
                current.iter().take(*current_count).copied()
            {
                if positions[&position] > 1 {
                    collisions += 1;
                    if collisions < COLLISIONS {
                        next[*next_count] = (position, nullghosts_collision(direction), collisions);
                        *next_count += 1;
                    } else {
                        seen.clear();
                    }
                } else {
                    next[*next_count] = (position, direction, collisions);
                    *next_count += 1;
                }
            }

            // clear positions
            positions.clear();
        }

        round += 1;
    }

    round
}

pub use part_2 as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"###########
#####.#####
###.....###
##.......##
##.>>.<<.##
#v...>...v#
##...v...##
##.......##
###.....###
#####.#####
###########"
            ),
            29,
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            part_2::<10>(
                r"###########
#####.#####
###.....###
##.......##
##.>>.<<.##
#v...>...v#
##...v...##
##.......##
###.....###
#####.#####
###########"
            ),
            53,
        );
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(
            part_2::<10000>(
                r"###########
#####.#####
###.....###
##.......##
##.>>.<<.##
#v...>...v#
##...v...##
##.......##
###.....###
#####.#####
###########"
            ),
            50000,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3() {
        assert_eq!(
            part_3::<1000000000000>(
                r"#######################
###########.###########
#######.^>.^.<^.#######
#####..<.>.v^>>^^<#####
####..v<^v.>.>v..^.####
###.................###
###.................###
##.......<...<.<....>##
##...v....>....v.v<.^##
##........###........##
##v......#####.......##
#.<......#####......>.#
##.......#####.......##
##........###........##
##...................##
##...................##
###.................###
###.................###
####...............####
#####.............#####
#######....v....#######
###########.###########
#######################"
            ),
            13314695352588,
        );
    }

    #[test]
    fn test_wall_collision() {
        assert_eq!(wall_collision((-1, 0)), (0, -1));
        assert_eq!(wall_collision((1, 0)), (0, 1));
        assert_eq!(wall_collision((0, 1)), (-1, 0));
        assert_eq!(wall_collision((0, -1)), (1, 0));
    }

    #[test]
    fn test_nullghosts_collision() {
        assert_eq!(nullghosts_collision((-1, 0)), (1, 0));
    }
}
