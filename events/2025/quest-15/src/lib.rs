use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const ORIGIN: (isize, isize) = (0, 0);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Info(usize, (usize, usize));

fn manhattan((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    (x1 - x2).unsigned_abs() + (y1 - y2).unsigned_abs()
}

/// # Panics
#[must_use]
#[allow(clippy::similar_names, clippy::cast_possible_wrap, clippy::too_many_lines)]
fn solve(data: &str) -> usize {
    let mut current = ORIGIN;
    let mut direction = (0, -1);

    let mut xs = HashSet::with_capacity(1024);
    let mut ys = HashSet::with_capacity(1024);
    let mut insert_point = |(x, y)| {
        for d in -1..=1 {
            xs.insert(x + d);
            ys.insert(y + d);
        }
    };
    insert_point(current);

    let mut segments = HashSet::with_capacity(1024);
    let mut insert_segment = |start: (isize, isize), end| {
        segments.insert((start.min(end), start.max(end)));
    };

    for m in data.split(',') {
        let mut chars = m.chars();

        direction = match chars.next().expect("No direction") {
            'L' => (direction.1, -direction.0),
            'R' => (-direction.1, direction.0),
            _ => unreachable!(),
        };

        let steps = chars.as_str().parse::<usize>().expect("Invalid steps") as isize;
        let end = (
            current.0 + steps * direction.0,
            current.1 + steps * direction.1,
        );

        insert_point(end);
        insert_segment(current, end);

        current = end;
    }

    let xs = {
        let mut list = xs.into_iter().collect::<Vec<_>>();
        list.sort_unstable();
        list
    };

    let ys = {
        let mut list = ys.into_iter().collect::<Vec<_>>();
        list.sort_unstable();
        list
    };

    let idx2xs = xs.iter().copied().enumerate().collect::<HashMap<_, _>>();
    let xs2idx = xs
        .iter()
        .copied()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    let idx2ys = ys.iter().copied().enumerate().collect::<HashMap<_, _>>();
    let ys2idx = ys
        .iter()
        .copied()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    let mut board = vec![vec![true; xs.len()]; ys.len()];
    for ((mut start_x, mut start_y), (end_x, end_y)) in segments {
        let n = manhattan((start_x, start_y), (end_x, end_y));
        let direction = (
            (end_x - start_x).unsigned_abs() / n,
            (end_y - start_y).unsigned_abs() / n,
        );
        while (start_x, start_y) != (end_x, end_y) {
            let (mut x, mut y) = (xs2idx[&start_x], ys2idx[&start_y]);
            board[y][x] = false;

            x += direction.0;
            y += direction.1;

            start_x = idx2xs[&x];
            start_y = idx2ys[&y];
        }
        board[ys2idx[&start_y]][xs2idx[&start_x]] = false;
    }

    let end = (xs2idx[&ORIGIN.0], ys2idx[&ORIGIN.1]);
    let start = (xs2idx[&current.0], ys2idx[&current.1]);

    board[start.1][start.0] = true;
    board[end.1][end.0] = true;

    let mut scores = HashMap::with_capacity(xs.len() * ys.len());
    scores.insert(start, 0);

    let mut heap = BinaryHeap::with_capacity(xs.len() * ys.len());
    heap.push(Reverse(Info(manhattan(ORIGIN, current), start)));
    while let Some(Reverse(Info(_f, (x, y)))) = heap.pop() {
        if (x, y) == end {
            return scores[&(x, y)];
        }

        let xx = idx2xs[&x];
        let yy = idx2ys[&y];

        let current_score = scores.get(&(x, y)).copied().unwrap_or(usize::MAX);
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let Some(new_x) = x.checked_add_signed(dx) else {
                continue;
            };
            let Some(new_y) = y.checked_add_signed(dy) else {
                continue;
            };
            if !board
                .get(new_y)
                .and_then(|row| row.get(new_x))
                .copied()
                .unwrap_or_default()
            {
                continue;
            }

            let new_xx = idx2xs[&new_x];
            let new_yy = idx2ys[&new_y];

            let score = current_score.saturating_add(manhattan((xx, yy), (new_xx, new_yy)));
            if score < scores.get(&(new_x, new_y)).copied().unwrap_or(usize::MAX) {
                scores.insert((new_x, new_y), score);
                heap.push(Reverse(Info(
                    score + manhattan(ORIGIN, (new_xx, new_yy)),
                    (new_x, new_y),
                )));
            }
        }
    }

    unreachable!()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    solve(data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    solve(data)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    solve(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(DATA), 16);
    }
}
