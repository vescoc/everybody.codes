use std::collections::HashMap;

type Nullghosts = HashMap<(usize, usize), Vec<((isize, isize), u64, usize)>>;
type Snapshot = Vec<(usize, (usize, usize), (isize, isize))>;

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

fn make_snapshot(nullghosts_state: &Nullghosts) -> Snapshot {
    let mut snaphost = nullghosts_state
        .iter()
        .flat_map(|(position, nullghosts)| {
            nullghosts
                .iter()
                .map(|(direction, _, id)| (*id, *position, *direction))
        })
        .collect::<Vec<_>>();

    snaphost.sort_unstable();

    snaphost
}

fn handle_cycles<const COLLISIONS: u64>(seen: &mut HashMap<Snapshot, (u64, Nullghosts)>, nullghosts_current: &mut Nullghosts, nullghosts_count: &mut usize, round: &mut u64) {
    let key = make_snapshot(nullghosts_current);
    let current_nullghosts_count = key.len();

    if current_nullghosts_count != *nullghosts_count {
        // invalidate the cache
        seen.clear();
        *nullghosts_count = current_nullghosts_count;
    } else if let Some((old_round, old_nullghosts_current)) = seen.get(&key) {
        let cycle = *round - old_round;

        let mut max_rep = u64::MAX;
        let mut ghosts = HashMap::with_capacity(1024);
        for (position, nullghosts) in nullghosts_current.iter() {
            for (_, current_collisions, current_id) in nullghosts {
                let old_collisions = old_nullghosts_current[position]
                    .iter()
                    .find_map(|(_, old_collisions, old_id)| {
                        if current_id == old_id {
                            Some(*old_collisions)
                        } else {
                            None
                        }
                    })
                    .unwrap();

                let delta_collisions = *current_collisions - old_collisions;
                ghosts.insert(*current_id, delta_collisions);

                if let Some(mut rep) =
                    (COLLISIONS - *current_collisions).checked_div(delta_collisions)
                {
                    if rep > 0
                        && (COLLISIONS - *current_collisions).is_multiple_of(delta_collisions)
                    {
                        rep -= 1;
                    }

                    max_rep = max_rep.min(rep);
                }
            }
        }

        if max_rep > 0 {
            *nullghosts_current = nullghosts_current
                .drain()
                .map(|(position, nullghosts)| {
                    let nullghosts = nullghosts
                        .into_iter()
                        .map(|(direction, mut collisions, id)| {
                            collisions += ghosts[&id] * max_rep;

                            (direction, collisions, id)
                        })
                        .collect::<Vec<_>>();

                    (position, nullghosts)
                })
                .collect();

            *round += max_rep * cycle;
        }
    }

    seen.insert(key, (*round, nullghosts_current.clone()));
}

/// # Panics
#[must_use]
pub fn part_2<const COLLISIONS: u64>(data: &str) -> u64 {
    let map = data.as_bytes();

    let mut width = usize::MAX;
    let mut height = 0;
    let mut nullghosts_current = map
        .iter()
        .enumerate()
        .filter_map(|(index, tile)| {
            let position = (index % (width.saturating_add(1)), height);
            match tile {
                b'<' => Some((position, vec![((-1, 0), 0, index)])),
                b'>' => Some((position, vec![((1, 0), 0, index)])),
                b'^' => Some((position, vec![((0, -1), 0, index)])),
                b'v' => Some((position, vec![((0, 1), 0, index)])),
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
        .collect::<Nullghosts>();
    height += usize::from(*map.last().unwrap() != b'\n');

    let mut seen = HashMap::with_capacity(1024);
    let mut nullghosts_count = usize::MAX;

    let mut round = 0u64;
    while !nullghosts_current.is_empty() {
        handle_cycles::<COLLISIONS>(&mut seen, &mut nullghosts_current, &mut nullghosts_count, &mut round);
        
        round += 1;

        let mut nullghosts_next: HashMap<_, Vec<_>> =
            HashMap::with_capacity(nullghosts_current.len());

        // move nullghosts
        for ((x, y), nullghosts) in nullghosts_current {
            for ((dx, dy), mut collisions, id) in nullghosts {
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
                            nullghosts_next
                                .entry(position)
                                .or_default()
                                .push((direction, collisions, id));
                        }
                    }
                    _ => unreachable!("out of map"),
                }
            }
        }

        // handle nullghosts collisions and replace with new state
        nullghosts_current = nullghosts_next
            .drain()
            .filter_map(|(position, nullghosts)| {
                let len = nullghosts.len();
                if len > 1 {
                    let nullghosts = nullghosts
                        .into_iter()
                        .filter_map(|(direction, mut collisions, id)| {
                            collisions += 1;
                            if collisions >= COLLISIONS {
                                return None;
                            }

                            Some((nullghosts_collision(direction), collisions, id))
                        })
                        .collect::<Vec<_>>();

                    if nullghosts.is_empty() {
                        None
                    } else {
                        Some((position, nullghosts))
                    }
                } else {
                    Some((position, nullghosts))
                }
            })
            .collect();
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
