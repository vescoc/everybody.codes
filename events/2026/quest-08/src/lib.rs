const DIRECTIONS: &[(isize, isize)] = &[(-1, -1), (1, -1), (2, 0), (1, 1), (-1, 1), (-2, 0)];

#[expect(clippy::cast_possible_wrap, reason = "usize value <= 9")]
#[expect(clippy::cast_possible_truncation, reason = "usize value <= 9")]
fn energy(
    map: &[u8],
    (width, height): (usize, usize),
    (dx, dy): (isize, isize),
    mut new_warrior: impl FnMut((usize, usize)),
    mut grow_warrior: impl FnMut((usize, usize)),
) -> u64 {
    map.chunks(width + 1)
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .take(width)
                .enumerate()
                .filter(|(x, tile)| {
                    if tile.is_ascii_digit() {
                        let ent = **tile;
                        if ent == b'9' {
                            new_warrior((*x, y));
                            return true;
                        }

                        let r = (ent..=b'9').enumerate().all(|(d, _)| {
                            d == 0
                                || (match (
                                    x.checked_add_signed(d as isize * dx),
                                    y.checked_add_signed(d as isize * dy),
                                ) {
                                    (Some(x), Some(y)) if x < width && y < height => {
                                        let tile = map[y * (width + 1) + x];
                                        if tile.is_ascii_digit() {
                                            tile - (d as u8) < ent
                                        } else {
                                            true
                                        }
                                    }
                                    _ => true,
                                })
                        });
                        if r {
                            grow_warrior((*x, y));
                        }

                        r
                    } else {
                        false
                    }
                })
                .count() as u64
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let map = data.as_bytes();
    let width = map
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid input data");
    let height = (map.len() + 1) / (width + 1);

    energy(map, (width, height), (-1, -1), |_| {}, |_| {})
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let map = data.as_bytes();
    let width = map
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid input data");
    let height = (map.len() + 1) / (width + 1);

    DIRECTIONS
        .iter()
        .map(|d| energy(map, (width, height), *d, |_| {}, |_| {}))
        .sum()
}

/// # Panics
#[must_use]
#[expect(clippy::unreadable_literal, reason = "spec")]
pub fn part_3(data: &str) -> u64 {
    let mut map = data.as_bytes().to_vec();
    let width = map
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid input data");
    let height = (map.len() + 1) / (width + 1);

    let mut total_warriors = 0;
    let mut total_energy = 0;

    let mut warriors_to_grow = Vec::with_capacity(1024);

    let mut seeds = [
        Vec::with_capacity(1024),
        Vec::with_capacity(1024),
        Vec::with_capacity(1024),
        Vec::with_capacity(1024),
    ];
    let mut seeds_head = 1;
    let mut seeds_back = 0;
    for d in DIRECTIONS.iter().cycle() {
        let energy = energy(
            &map,
            (width, height),
            *d,
            |p| seeds[seeds_back].push(p),
            |p| warriors_to_grow.push(p),
        );

        total_energy += energy;

        total_warriors += seeds[seeds_back].len();

        // check if we reach the target
        if total_warriors >= 100000 {
            break;
        }

        // remove warriors
        for (x, y) in &seeds[seeds_back] {
            map[y * (width + 1) + x] = b'.';
        }

        // grow warriors
        for (x, y) in warriors_to_grow.drain(..) {
            map[y * (width + 1) + x] += 1;
        }

        // plant seeds
        for (x, y) in seeds[seeds_head].drain(..) {
            map[y * (width + 1) + x] = b'1';
        }

        seeds_head = (seeds_head + 1) % seeds.len();
        seeds_back = (seeds_back + 1) % seeds.len();
    }

    total_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8...."
            ),
            32,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8...."
            ),
            187,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8...."
            ),
            899889,
        );
    }
}
