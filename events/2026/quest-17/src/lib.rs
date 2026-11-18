use std::collections::VecDeque;

type Coord = (usize, usize);
type Direction = (isize, isize);

const DIRECTIONS: &[Direction] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let (map, (width, height)) = parse_map(data);

    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((0, (0, 0)));
    while let Some((steps, (x, y))) = queue.pop_front() {
        if (x, y) == (width - 1, height - 1) {
            return steps;
        }

        let visit = &mut visited[y][x];
        if *visit {
            continue;
        }
        *visit = true;

        for &(dx, dy) in DIRECTIONS {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y))
                    if x < width && y < height && map[y * (width + 1) + x] == b'#' =>
                {
                    queue.push_back((steps + 1, (x, y)));
                }
                _ => {}
            }
        }
    }

    unreachable!("invalid input data")
}

fn parse_map(data: &str) -> (&[u8], Coord) {
    let map = data.as_bytes();
    let width = map
        .iter()
        .position(|tile| *tile == b'\n')
        .expect("invalid input data");
    let height = (map.len() + 1) / (width + 1);

    (map, (width, height))
}

fn flood(map: &[u8], (width, height): Coord, mut edge: impl FnMut(Coord, Direction)) {
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        let visit = &mut visited[y][x];
        if *visit {
            continue;
        }
        *visit = true;

        for &(dx, dy) in DIRECTIONS {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(nx), Some(ny))
                    if nx < width && ny < height && map[ny * (width + 1) + nx] == b'#' =>
                {
                    queue.push_back((nx, ny));
                }
                _ => edge((x, y), (dx, dy)),
            }
        }
    }
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let (map, (width, height)) = parse_map(data);

    let mut edges = 0;
    flood(map, (width, height), |_, _| edges += 1);

    edges
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let (map, (width, height)) = parse_map(data);

    let tile = |(x, y): Coord, (dx, dy): Direction| match (
        x.checked_add_signed(dx),
        y.checked_add_signed(dy),
    ) {
        (Some(x), Some(y)) if x < width && y < height => map[y * (width + 1) + x],
        _ => b'.',
    };

    let mut edge_map = vec![vec![b' '; width * 3]; height * 3];
    flood(map, (width, height), |(x, y), (dx, dy)| {
        for d in [-1, 0, 1] {
            let (x, y) = (
                (x * 3 + 1).checked_add_signed(dx - dy * d).unwrap(),
                (y * 3 + 1).checked_add_signed(dy + dx * d).unwrap(),
            );
            edge_map[y][x] = if d == 0 { b'e' } else { b'#' };
        }
    });

    // fix the missing tiles
    for (y, row) in map.chunks(width + 1).enumerate() {
        for x in row
            .iter()
            .take(width)
            .enumerate()
            .filter_map(|(x, t)| (*t == b'.').then_some(x))
        {
            for (dx, dy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                if [(dx, dy), (0, dy), (dx, 0)]
                    .into_iter()
                    .all(|(dx, dy)| tile((x, y), (dx, dy)) == b'#')
                {
                    edge_map[(y * 3 + 1).checked_add_signed(dy * 2).unwrap()]
                        [(x * 3 + 1).checked_add_signed(dx * 2).unwrap()] = b'#';
                }
            }
        }
    }

    debug_assert_eq!(edge_map[0][0], b'#');
    debug_assert_eq!(edge_map[3 * height - 1][3 * width - 1], b'#');

    let mut visited = vec![vec![false; width * 3]; height * 3];
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((0, (0, 0)));
    while let Some((mut edges, (x, y))) = queue.pop_front() {
        if (x, y) == (width * 3 - 1, height * 3 - 1) {
            return edges;
        }

        let visit = &mut visited[y][x];
        if *visit {
            continue;
        }
        *visit = true;

        edges += u64::from(edge_map[y][x] == b'e');

        for &(dx, dy) in DIRECTIONS {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y)) if x < width * 3 && y < height * 3 && edge_map[y][x] != b' ' => {
                    queue.push_back((edges, (x, y)));
                }
                _ => {}
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##"
            ),
            21,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##"
            ),
            94,
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##"
            ),
            31,
        );
    }
}
