use std::collections::VecDeque;

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const TREE_STAMP: &[(isize, isize)] = &const {
    let mut stamp = [(0isize, 0isize); { 5 * 5 - 5 }];

    let mut index = 0;
    let mut x = -2isize;
    while x < 3 {
        let mut y = -2isize;
        while y < 3 {
            if (x != 0 || y != 0) && x.abs() + y.abs() < 4 {
                stamp[index] = (x, y);
                index += 1;
            }

            y += 1;
        }

        x += 1;
    }

    stamp
};

/// # Panics
#[must_use]
pub fn parse_map(data: &str) -> (Vec<u8>, (usize, usize)) {
    let data = data.as_bytes();
    let width = data.iter().position(|c| *c == b'\n').expect("invalid map");
    let height = (data.len() + 1) / (width + 1);

    let map = data.to_vec();

    (map, (width, height))
}

/// # Panics
pub fn calc_land(map: &mut [u8], (width, height): (usize, usize)) -> u64 {
    let mut land_cells = 1;

    let start_cell = (width / 2, height / 2);
    map[start_cell.1 * (width + 1) + start_cell.0 / 2] = b'#';

    let mut queue = VecDeque::with_capacity(1024);
    for (dx, dy) in DIRECTIONS {
        if let Some(x) = start_cell.0.checked_add_signed(*dx)
            && let Some(y) = start_cell.1.checked_add_signed(*dy)
            && x < width
            && y < height
            && map[y * (width + 1) + x] != b'#'
        {
            queue.push_back((x, y));
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if map[y * (width + 1) + x] == b'#' {
            continue;
        }

        // check count
        let count = DIRECTIONS
            .iter()
            .filter(|(dx, dy)| {
                matches!(
                    (x.checked_add_signed(*dx), y.checked_add_signed(*dy)),
                    (Some(x), Some(y)) if x < width && y < height && matches!(map[y * (width + 1) + x], b'#' | b'1'),
                )
            })
            .count();

        // if count >= 5 change to land and analyse neighbors
        if count >= 5 {
            land_cells += 1;

            map[y * (width + 1) + x] = b'#';

            for (dx, dy) in DIRECTIONS {
                if let Some(x) = x.checked_add_signed(*dx)
                    && let Some(y) = y.checked_add_signed(*dy)
                    && x < width
                    && y < height
                    && map[y * (width + 1) + x] != b'#'
                {
                    queue.push_back((x, y));
                }
            }
        }
    }

    land_cells
}

/// # Panics
pub fn calc_sand(map: &mut [u8], (width, height): (usize, usize)) -> u64 {
    let mut queue = VecDeque::with_capacity(1024);

    let mut sand_cells = 0;

    // first step: mark land as sand if is near water
    for x in 0..width {
        for y in 0..height {
            let tile = map[y * (width + 1) + x];
            if tile == b'#'
                && DIRECTIONS.iter().any(|(dx, dy)| {
                    match (x.checked_add_signed(*dx), y.checked_add_signed(*dy)) {
                        (Some(x), Some(y)) if x < width && y < height => {
                            matches!(map[y * (width + 1) + x], b'0' | b'1' | b'.')
                        }
                        _ => true,
                    }
                })
            {
                sand_cells += 1;

                map[y * (width + 1) + x] = b'=';

                for (dx, dy) in DIRECTIONS {
                    if let Some(x) = x.checked_add_signed(*dx)
                        && let Some(y) = y.checked_add_signed(*dy)
                        && x < width
                        && y < height
                    {
                        let tile = map[y * (width + 1) + x];
                        if tile != b'0' && tile != b'1' && tile != b'.' && tile != b'=' {
                            queue.push_back((x, y));
                        }
                    }
                }
            }
        }
    }

    // second step: propagate sand
    while let Some((x, y)) = queue.pop_front() {
        if map[y * (width + 1) + x] == b'=' {
            continue;
        }

        // check count
        let count = DIRECTIONS
            .iter()
            .filter(|(dx, dy)| {
                matches!(
                    (x.checked_add_signed(*dx), y.checked_add_signed(*dy)),
                    (Some(x), Some(y)) if x < width && y < height && matches!(map[y * (width + 1) + x], b'='),
                )
            })
            .count();

        // if count >= 5 change to sand and analyse neighbors
        if count >= 5 {
            sand_cells += 1;

            map[y * (width + 1) + x] = b'=';

            for (dx, dy) in DIRECTIONS {
                if let Some(x) = x.checked_add_signed(*dx)
                    && let Some(y) = y.checked_add_signed(*dy)
                    && x < width
                    && y < height
                {
                    let tile = map[y * (width + 1) + x];
                    if tile != b'0' && tile != b'1' && tile != b'.' && tile != b'=' {
                        queue.push_back((x, y));
                    }
                }
            }
        }
    }

    sand_cells
}

pub fn calc_trees(map: &mut [u8], (width, height): (usize, usize)) -> u64 {
    let mut tree_cells = 0;

    // first step: mark land as tree if it satisfy stamp pattern
    for y in 0..height {
        for x in 0..width {
            let tile = map[y * (width + 1) + x];
            if tile == b'#'
                && TREE_STAMP.iter().all(|(dx, dy)| {
                    match (x.checked_add_signed(*dx), y.checked_add_signed(*dy)) {
                        (Some(x), Some(y)) if x < width && y < height => {
                            matches!(map[y * (width + 1) + x], b'#' | b'@')
                        }
                        _ => false,
                    }
                })
            {
                tree_cells += 1;

                map[y * (width + 1) + x] = b'@';
            }
        }
    }

    // second step: grow forest for two steps
    for _step in 0..2 {
        let mut added = vec![];
        for y in 0..height {
            'out: for x in 0..width {
                let tile = map[y * (width + 1) + x];
                if tile == b'#' {
                    // check how many trees neighbors and not near to sand
                    let mut count = 0;
                    for (dx, dy) in DIRECTIONS {
                        let Some(x) = x.checked_add_signed(*dx) else {
                            continue 'out;
                        };
                        let Some(y) = y.checked_add_signed(*dy) else {
                            continue 'out;
                        };

                        if x >= width || y >= height {
                            continue 'out;
                        }

                        let tile = map[y * (width + 1) + x];
                        match tile {
                            b'=' => continue 'out,
                            b'@' => {
                                count += 1;
                            }
                            b'#' => {}
                            _ => unreachable!(),
                        }
                    }

                    if count >= 4 {
                        added.push((x, y));
                    }
                }
            }
        }

        tree_cells += added.len() as u64;
        for (x, y) in added {
            map[y * (width + 1) + x] = b'@';
        }
    }

    tree_cells
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let (mut map, dimensions) = parse_map(data);

    calc_land(&mut map, dimensions)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let (mut map, dimensions) = parse_map(data);

    calc_land(&mut map, dimensions);

    calc_sand(&mut map, dimensions)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let (mut map, dimensions) = parse_map(data);

    calc_land(&mut map, dimensions);

    calc_sand(&mut map, dimensions);

    calc_trees(&mut map, dimensions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"010100100100001101000
111101001000001110000
101111011000101010011
001000101010000100000
000110100110011001100
000001110111111000000
101100010101011110000
001101100000001010011
001001001111010001001
100011011110011010101
001100101110010010000"
            ),
            137,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"010100100100001101000
111101001000001110000
101111011000101010011
001000101010000100000
000110100110011001100
000001110111111000000
101100010101011110000
001101100000001010011
001001001111010001001
100011011110011010101
001100101110010010000"
            ),
            69,
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"010100100100001101000
111101001000001110000
101111011000101010011
001000101010000100000
000110100110011001100
000001110111111000000
101100010101011110000
001101100000001010011
001001001111010001001
100011011110011010101
001100101110010010000"
            ),
            14,
        );
    }
}
