use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq)]
/// Note ID, current x position, sulphur count, steps
struct Info(usize, usize, u64, u64);

impl Ord for Info {
    fn cmp(&self, Info(_, _, count, steps): &Self) -> Ordering {
        // count.cmp(&self.2).then(self.3.cmp(steps))
        self.3.cmp(steps).then(count.cmp(&self.2))
    }
}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// # Results
///
/// - horizontal output position for the path
/// - sulphur count of the path
/// - length of the path (only internal)
fn visit_path(
    cavern: &[u8],
    (width, height): (usize, usize),
    bound: (usize, usize),
    (x, y): (usize, usize),
) -> (usize, u64, u64) {
    let mut out = 0;
    let mut count = 0;
    let mut len = 1;

    let mut visited = HashSet::with_capacity(1024);
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        visited.insert((x, y));
        count += u64::from(cavern[y * (width + 1) + x] == b'@');

        let mut check = 0;
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y))
                    if (bound.0..bound.1).contains(&x)
                        && y < height
                        && x < width
                        && !visited.contains(&(x, y))
                        && cavern[y * (width + 1) + x] != b'#' =>
                {
                    out = out.max(x);
                    len += 1;
                    queue.push_back((x, y));
                    check += 1;
                }
                _ => {}
            }
        }

        debug_assert!(check <= 1);
    }

    (out, count, len)
}

/// # Results
///
/// List of paths for the stage `[start..end]`
///
/// - `(<in horizontal position, out horizontal position>)`
/// - sulphur count of the path
/// - length of the path (only internal)
fn calc_paths(
    cavern: &[u8],
    (width, height): (usize, usize),
    start: usize,
    end: usize,
) -> Vec<((usize, usize), u64, u64)> {
    let mut paths = vec![];
    for index in 0..(end - start).div_ceil(2) {
        if cavern[(height / 2 - 1) * (width + 1) + start + index] != b'#' {
            let (out, count, len) = visit_path(
                &cavern[..(height / 2 - 1) * (width + 1) + width - 1],
                (width, height / 2),
                (start, end),
                (start + index, height / 2 - 1),
            );
            if count > 0 {
                paths.push(((start + index, out), count, len));
            }
        }

        if cavern[(height / 2 + 1) * (width + 1) + start + index] != b'#' {
            let (out, count, len) = visit_path(
                &cavern[(height / 2 + 1) * (width + 1)..],
                (width, height / 2),
                (start, end),
                (start + index, 0),
            );
            if count > 0 {
                paths.push(((start + index, out), count, len));
            }
        }
    }

    paths
}

/// # Results
///
/// List of nodes
///
/// - `(<in horizontal position, out horizontal position>)`
/// - sulphur count of the path
/// - length of the path (only internal)
fn calc_nodes(cavern: &[u8], (width, height): (usize, usize)) -> Vec<((usize, usize), u64, u64)> {
    let mut nodes = vec![];

    let mut x = 0;
    loop {
        if (0..height / 2 - 1).all(|y| cavern[y * (width + 1) + x] == b'#') {
            if x < width - 1 {
                let mut next_x = x + 1;
                while (0..height / 2 - 1).any(|y| cavern[y * (width + 1) + next_x] != b'#') {
                    next_x += 1;
                }

                nodes.append(&mut calc_paths(cavern, (width, height), x + 1, next_x));

                x = next_x;
            } else {
                break;
            }
        } else {
            unreachable!();
        }
    }

    nodes
}

fn solve_stage(cavern: &[u8], wh: (usize, usize), start: usize, end: usize) -> u64 {
    fn solve_stage_r(nodes: &mut [((usize, usize), u64, u64)], start: usize, end: usize) -> u64 {
        let l = nodes.len();
        if l == 0 {
            return end.abs_diff(start) as u64 + 1;
        }

        let mut min = u64::MAX;
        for index in 0..l {
            let ((i, o), _, len) = nodes[index];

            nodes[index..].rotate_left(1);

            min = min.min(
                (start.abs_diff(i) as u64 + 1 + len + solve_stage_r(&mut nodes[..l - 1], o, end))
                    .min(
                        start.abs_diff(o) as u64
                            + 1
                            + len
                            + solve_stage_r(&mut nodes[..l - 1], i, end),
                    ),
            );

            nodes[index..].rotate_right(1);
        }

        min
    }

    let mut paths = calc_paths(cavern, wh, start, end);
    solve_stage_r(&mut paths, start, end)
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let cavern = data.as_bytes();
    let width = cavern
        .iter()
        .position(|&tile| tile == b'\n')
        .expect("invalid input data");
    let height = (cavern.len() + 1) / (width + 1);

    let mut x = 0;
    let mut moves = 0;
    loop {
        if (0..height / 2 - 1).all(|y| cavern[y * (width + 1) + x] == b'#') {
            if x < width - 1 {
                let mut next_x = x + 1;
                while (0..height / 2 - 1).any(|y| cavern[y * (width + 1) + next_x] != b'#') {
                    next_x += 1;
                }

                if x + 1 == next_x {
                    moves += 1;
                } else {
                    moves += solve_stage(cavern, (width, height), x + 1, next_x);
                }

                x = next_x;
            } else {
                return moves;
            }
        } else {
            unreachable!();
        }
    }
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let cavern = data.as_bytes();
    let width = cavern
        .iter()
        .position(|&tile| tile == b'\n')
        .expect("invalid input data");
    let height = (cavern.len() + 1) / (width + 1);

    let nodes = calc_nodes(cavern, (width, height));

    let mut result = u64::MAX;

    let mut visited = HashMap::with_capacity(4096);

    let mut set = BTreeSet::from([Info(usize::MAX, 1, 0, 0)]);
    while let Some(Info(current_id, current, count, steps)) = set.pop_first() {
        if steps > result {
            continue;
        }

        for (id, ((i, o), path_count, path_len)) in nodes.iter().enumerate() {
            for (target_in, target_out) in [(*i, *o), (*o, *i)] {
                if current == target_in && id == current_id {
                    continue;
                }

                let next_steps = steps + current.abs_diff(target_in) as u64 + path_len + 1;
                let next_count = count + path_count;
                if next_count >= 2026 {
                    let count = next_steps + target_out.abs_diff(width) as u64;

                    result = result.min(count);
                } else {
                    let value = visited
                        .entry((id, target_out, next_count))
                        .or_insert(u64::MAX);
                    if *value > next_steps {
                        *value = next_steps;
                        set.insert(Info(id, target_out, next_count, next_steps));
                    }
                }
            }
        }
    }

    result
}

#[expect(clippy::unreadable_literal, reason = "spec")]
const TARGET_SULPHUR: u64 = 202620262026;

/// # Panics
#[must_use]
#[expect(clippy::cast_possible_truncation, reason = "index < 5000")]
pub fn part_3(data: &str) -> u64 {
    fn find_period(results: &[u64]) -> Option<usize> {
        let len = results.len();
        for period in 1..100 {
            let steps_diff = results[len - 1]
                .checked_signed_diff(results[len - 1 - period])
                .unwrap();
            if (1..period * 20).all(|index| {
                results[len - 1 - index]
                    .checked_signed_diff(results[len - 1 - period - index])
                    .unwrap()
                    == steps_diff
            }) {
                return Some(period);
            }
        }

        None
    }

    let cavern = data.as_bytes();
    let width = cavern
        .iter()
        .position(|&tile| tile == b'\n')
        .expect("invalid input data");
    let height = (cavern.len() + 1) / (width + 1);

    let nodes = calc_nodes(cavern, (width, height));

    let mut results = vec![u64::MAX; 5000];
    results[0] = width as u64;

    let mut visited = HashMap::with_capacity(4096);

    let mut set = BTreeSet::from([Info(usize::MAX, 1, 0, 0)]);
    while let Some(Info(current_id, current, count, steps)) = set.pop_first() {
        let Some(result) = results.get_mut(count as usize) else {
            continue;
        };
        if steps > *result {
            continue;
        }

        for (id, ((i, o), path_count, path_len)) in nodes.iter().enumerate() {
            for (target_in, target_out) in [(*i, *o), (*o, *i)] {
                if current == target_in && id == current_id {
                    continue;
                }

                let next_count = count + path_count;
                let Some(result) = results.get_mut(next_count as usize) else {
                    continue;
                };

                let next_steps = steps + current.abs_diff(target_in) as u64 + path_len + 1;
                let steps_to_end = next_steps + target_out.abs_diff(width) as u64;
                *result = (*result).min(steps_to_end);

                let value = visited
                    .entry((id, target_out, next_count))
                    .or_insert(u64::MAX);
                if *value > next_steps {
                    *value = next_steps;
                    set.insert(Info(id, target_out, next_count, next_steps));
                }
            }
        }
    }

    let period = find_period(&results).expect("not periodic");

    let sulphur = results.len();
    let cycle_steps = results[sulphur - 1] - results[sulphur - 1 - period];

    let rep = (TARGET_SULPHUR - sulphur as u64).div_ceil(period as u64);

    let cycle_start = TARGET_SULPHUR - rep * period as u64;

    results[cycle_start as usize] + cycle_steps * rep
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bfs(data: &str, target: u64) -> u64 {
        fn forward(d: (isize, isize)) -> (isize, isize) {
            d
        }

        fn turn_left((dx, dy): (isize, isize)) -> (isize, isize) {
            (dy, -dx)
        }

        fn turn_right((dx, dy): (isize, isize)) -> (isize, isize) {
            (-dy, dx)
        }

        let cavern = data.as_bytes();
        let width = cavern
            .iter()
            .position(|&tile| tile == b'\n')
            .expect("invalid input data");
        let height = (cavern.len() + 1) / (width + 1);

        debug_assert_eq!(cavern[(height / 2) * (width + 1)], b'S');
        debug_assert_eq!(cavern[(height / 2) * (width + 1) + width - 1], b'E');

        let mut visited = HashSet::with_capacity(4096);
        let mut queue = VecDeque::with_capacity(4096);
        queue.push_back((1, 0, (0usize, height / 2), (1, 0)));

        while let Some((steps, mut count, (x, y), (dx, dy))) = queue.pop_front() {
            let Some(x) = x.checked_add_signed(dx) else {
                continue;
            };
            if x >= width {
                continue;
            }

            let Some(y) = y.checked_add_signed(dy) else {
                continue;
            };
            if y >= height {
                continue;
            }

            match cavern[y * (width + 1) + x] {
                b'#' => {
                    continue;
                }
                b'@' => {
                    count += 1;
                }
                b'E' => {
                    debug_assert_eq!((dx, dy), (1, 0));

                    if count >= target {
                        return steps;
                    }

                    continue; // cannot move from E
                }
                b'S' => {
                    debug_assert_eq!((dx, dy), (-1, 0));

                    continue; // cannot move from S
                }
                b'.' => {}
                _ => unreachable!(),
            }

            if !visited.insert((count, (x, y), (dx, dy))) {
                continue;
            }

            for (dx, dy) in [forward((dx, dy)), turn_left((dx, dy)), turn_right((dx, dy))] {
                queue.push_back((steps + 1, count, (x, y), (dx, dy)));
            }
        }

        unreachable!()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"#####################################################
#.@.........#................#.....................##
#.#########@#.##############.#.###################.##
#.#.......#.#.##..........##.#.###..@..@......@###.##
#.#.#####.#.#.##.########.##.#.###@###########.###.##
#.#.#@@@#.#.#.##.##....##.##@#.###.###.....###.###.##
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###.##
S...................................................E
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###@##
#.#.#...#.#.#.##@##.@..##.##.#.###.###@....###.###.##
#.#.#####.#.#.##.########.##.#.###.###########.###.##
#.#.......#.#.##..@.......##.#.###.............###.##
#.#########.#.##############.#.###################.##
#...........#................#................@....##
#####################################################"
            ),
            184,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"#####################################################
#.@.........#................#.....................##
#.#########@#.##############.#.###################.##
#.#.......#.#.##..........##.#.###..@..@......@###.##
#.#.#####.#.#.##.########.##.#.###@###########.###.##
#.#.#@@@#.#.#.##.##....##.##@#.###.###.....###.###.##
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###.##
S...................................................E
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###@##
#.#.#...#.#.#.##@##.@..##.##.#.###.###@....###.###.##
#.#.#####.#.#.##.########.##.#.###.###########.###.##
#.#.......#.#.##..@.......##.#.###.............###.##
#.#########.#.##############.#.###################.##
#...........#................#................@....##
#####################################################"
            ),
            5448,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"#####################################################
#.@.........#................#..........@..........##
#.#########@#.##############.#.###################.##
#.#...@...#.#.##.....@....##.#.###..@..@......@###.##
#.#.#####.#.#.##.########.##.#.###@###########.###.##
#.#.#@@@#.#.#.##.##..@.##.##@#.###.###..@..###.###.##
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###.##
S...................................................E
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###@##
#.#.#.@.#.#.#.##@##.@..##.##.#.###.###@....###.###.##
#.#.#####.#.#.##.########.##.#.###.###########.###.##
#.#...@...#.#.##..@.......##.#.###......@......###.##
#.#########.#.##############.#.###################.##
#.....@.....#........@.......#................@....##
#####################################################"
            ),
            540320698784
        );
    }

    #[test]
    fn bfs_part_2() {
        let data = include_str!("../data/part_2");

        assert_eq!(bfs(data, 2026), part_2(data));
    }

    #[test]
    fn bfs_part_3() {
        let data = include_str!("../data/part_3");

        assert_eq!(bfs(data, 2026), part_2(data));
    }
}
