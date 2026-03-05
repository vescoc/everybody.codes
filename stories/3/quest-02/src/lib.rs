use std::collections::{HashSet, VecDeque};

/// # Panics
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn part_1(data: &str) -> u64 {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in data.lines().enumerate() {
        for (c, v) in row.chars().enumerate() {
            match v {
                '@' => {
                    start = (r as i64, c as i64);
                }
                '#' => {
                    end = (r as i64, c as i64);
                }
                _ => {}
            }
        }
    }

    let mut directions = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();

    let mut steps = 0;
    let mut set = HashSet::with_capacity(data.len());
    set.insert(start);
    loop {
        let (dr, dc) = directions.next().unwrap();
        let next_position = (start.0 + dr, start.1 + dc);
        if !set.insert(next_position) {
            continue;
        }
        steps += 1;
        if end == next_position {
            break;
        }
        start = next_position;
    }

    steps
}

fn p_min((r1, c1): (i64, i64), (r2, c2): (i64, i64)) -> (i64, i64) {
    (r1.min(r2), c1.min(c2))
}

fn p_max((r1, c1): (i64, i64), (r2, c2): (i64, i64)) -> (i64, i64) {
    (r1.max(r2), c1.max(c2))
}

fn fill(
    set: &HashSet<(i64, i64)>,
    min: (i64, i64),
    max: (i64, i64),
    position: (i64, i64),
) -> Option<HashSet<(i64, i64)>> {
    let mut filler = HashSet::with_capacity(2048);
    filler.insert(position);

    let mut queue = VecDeque::from([position]);
    while let Some(position) = queue.pop_front() {
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let position = (position.0 + dr, position.1 + dc);
            if !set.contains(&position) {
                if (min.0 + 1..max.0).contains(&position.0)
                    && (min.1 + 1..max.1).contains(&position.1)
                {
                    if filler.insert(position) {
                        queue.push_back(position);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    Some(filler)
}

/// # Panics
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn solve<'a>(data: &str, mut directions: impl Iterator<Item = &'a (i64, i64)>) -> u64 {
    let mut start = (0, 0);
    let mut bones = Vec::new();

    for (r, row) in data.lines().enumerate() {
        for (c, v) in row.chars().enumerate() {
            match v {
                '@' => {
                    start = (r as i64, c as i64);
                }
                '#' => {
                    bones.push((r as i64, c as i64));
                }
                _ => {}
            }
        }
    }

    let mut min = bones
        .iter()
        .copied()
        .chain(std::iter::once(start))
        .reduce(p_min)
        .unwrap();
    let mut max = bones
        .iter()
        .copied()
        .chain(std::iter::once(start))
        .reduce(p_max)
        .unwrap();

    let mut steps = 0;

    let mut set = bones
        .iter()
        .copied()
        .chain(std::iter::once(start))
        .collect::<HashSet<_>>();
    for r in min.0..=max.0 {
        for c in min.1..=max.1 {
            if set.contains(&(r, c)) {
                continue;
            }

            if let Some(filler) = fill(&set, min, max, (r, c)) {
                for position in filler {
                    set.insert(position);
                }
            }
        }
    }

    let bones_set = bones.iter().copied().collect::<HashSet<_>>();
    let perimeter = bones
        .iter()
        .copied()
        .flat_map(|(r, c)| [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)])
        .collect::<HashSet<_>>()
        .difference(&bones_set)
        .copied()
        .collect::<HashSet<_>>();

    loop {
        let (dr, dc) = directions.next().unwrap();
        let next_position = (start.0 + dr, start.1 + dc);
        if !set.insert(next_position) {
            continue;
        }

        start = next_position;

        min = p_min(min, start);
        max = p_max(max, start);

        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let position = (start.0 + dr, start.1 + dc);
            if set.contains(&position) {
                continue;
            }

            if (min.0 + 1..max.0).contains(&position.0)
                && (min.1 + 1..max.1).contains(&position.1)
                && let Some(filler) = fill(&set, min, max, position)
            {
                for position in filler {
                    set.insert(position);
                }
            }
        }

        steps += 1;

        if set.intersection(&perimeter).count() == perimeter.len() {
            break;
        }
    }

    steps
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve(data, [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle())
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    solve(
        data,
        [
            (-1, 0),
            (-1, 0),
            (-1, 0),
            (0, 1),
            (0, 1),
            (0, 1),
            (1, 0),
            (1, 0),
            (1, 0),
            (0, -1),
            (0, -1),
            (0, -1),
        ]
        .iter()
        .cycle(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r".......
.......
.......
.#.@...
.......
.......
.......";
        assert_eq!(part_1(data), 12);
    }

    #[test]
    fn test_part_2() {
        let data = r".......
.......
.......
.#.@...
.......
.......
.......";
        assert_eq!(part_2(data), 47);
    }

    #[test]
    fn test_part_3_1() {
        let data = r".......
.......
.......
.#.@...
.......
.......
.......";
        assert_eq!(part_3(data), 87);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"#..#.......#...
...#...........
...#...........
#######........
...#....#######
...#...@...#...
...#.......#...
...........#...
...........#...
#..........#...
##......#######";
        assert_eq!(part_3(data), 239);
    }

    #[test]
    fn test_part_3_3() {
        let data = r"................................................................
.........................###.........###........................
....................##...###########.#####......#.......###.....
.........##.............############....####.............##.....
.......######..............#############.###....................
.........##................#############.###.......##...........
...............##...........########....####....................
...............................####.#######...........##........
........................##################...........####.......
....#.........#########################.....##......######......
..............#.##......##....##..##.##...............##........
..............................##....##..........##..............
........####....#################..######...................##..
........###.....###...####..###..##...##.########...............
.................####....###..##.##.##..###....##.....##........
....##...........#######.....##..##..##......#####..........#...
...........##......#########......#....##.######..........#####.
...........##........###########################....#.......#...
.........######............##################.......#...........
...........##.............#########.............................
............#.........#############....................#........
.....#...........##..####......###......##........#.............
.............##................###..........#.....#.............
..................##...........##...................##..........
..........................###.####.####.........................
................#.###########..###.############.#...............
.....#####....###...............................###.............
.....#####...#############......@......#############............
.....#########.###################################.#............
...###########..##.....###################.....##..##...........
...######...#######.##...###.........##...##...###.##...........
.....##.########........#####..###..####.......#.########.......
............#########################################...........
..............#####################################.............
...............................###..............................
................................................................";
        assert_eq!(part_3(data), 1539);
    }
}
