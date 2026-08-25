#![no_std]

mod bitset;
use bitset::BitSet;

type Deque<T> = heapless::Deque<T, 1024>;

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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

    let mut directions = DIRECTIONS.iter().cycle();

    let mut steps = 0;
    let mut set = BitSet::new();
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
    filler: &mut BitSet,
    set: &BitSet,
    min: (i64, i64),
    max: (i64, i64),
    position: (i64, i64),
) -> bool {
    filler.insert(position);

    let mut queue = Deque::try_from([position]).unwrap();
    while let Some(position) = queue.pop_front() {
        for (dr, dc) in DIRECTIONS {
            let position = (position.0 + dr, position.1 + dc);
            if !set.contains(&position) {
                if (min.0 + 1..max.0).contains(&position.0)
                    && (min.1 + 1..max.1).contains(&position.1)
                {
                    if filler.insert(position) {
                        queue.push_back(position).unwrap();
                    }
                } else {
                    return false;
                }
            }
        }
    }

    true
}

/// # Panics
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn solve<'a>(data: &str, mut directions: impl Iterator<Item = &'a (i64, i64)>) -> u64 {
    let mut steps = 0;

    let mut start = (0, 0);

    let mut set = BitSet::new();
    let mut perimeter = BitSet::new();
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);

    let mut init = |position| {
        set.insert(position);
        for (dr, dc) in DIRECTIONS {
            perimeter.insert((position.0 + dr, position.1 + dc));
        }

        min = p_min(min, position);
        max = p_max(max, position);
    };

    for (r, row) in data.lines().enumerate() {
        for (c, v) in row.chars().enumerate() {
            match v {
                '@' => {
                    start = (r as i64, c as i64);

                    init(start);
                }
                '#' => {
                    let bone = (r as i64, c as i64);

                    init(bone);
                }
                _ => {}
            }
        }
    }

    // filling holes
    for r in min.0..=max.0 {
        for c in min.1..=max.1 {
            if set.contains(&(r, c)) {
                continue;
            }

            let mut filler = BitSet::new();
            if fill(&mut filler, &set, min, max, (r, c)) {
                set.union(&filler);
            }
        }
    }

    // fix perimeter
    perimeter.difference(&set);

    while !perimeter.is_empty() {
        let (dr, dc) = directions.next().unwrap();
        let next_position = (start.0 + dr, start.1 + dc);
        if !set.insert(next_position) {
            continue;
        }

        start = next_position;
        perimeter.remove(&start);

        min = p_min(min, start);
        max = p_max(max, start);

        for (dr, dc) in DIRECTIONS {
            let position = (start.0 + dr, start.1 + dc);
            if set.contains(&position) {
                continue;
            }

            if (min.0 + 1..max.0).contains(&position.0) && (min.1 + 1..max.1).contains(&position.1)
            {
                let mut filler = BitSet::new();
                if fill(&mut filler, &set, min, max, position) {
                    set.union(&filler);
                    perimeter.difference(&filler);
                }
            }
        }

        steps += 1;
    }

    steps
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve(data, DIRECTIONS.iter().cycle())
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
