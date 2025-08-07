use std::collections::HashSet;

type Point = (usize, usize);

fn manhattan(&(xa, ya): &Point, &(xb, yb): &Point) -> usize {
    xa.abs_diff(xb) + ya.abs_diff(yb)
}

fn min_distance(
    q: &HashSet<Point>,
    current: &HashSet<Point>,
    check: impl Fn(usize) -> bool,
) -> Option<(Point, usize)> {
    q.iter()
        .filter_map(|target| {
            let distance = current
                .iter()
                .map(|source| manhattan(source, target))
                .min()
                .unwrap();
            if check(distance) {
                Some((*target, distance))
            } else {
                None
            }
        })
        .min_by_key(|(_, distance)| *distance)
}

/// # Panics
#[must_use]
pub fn solve(data: &[u8]) -> usize {
    let mut stars = data
        .split(|&c| c == b'\n')
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &tile)| if tile == b'*' { Some((r, c)) } else { None })
        })
        .collect::<HashSet<_>>();
    let size = stars.len();

    let start_star = stars.iter().copied().next().unwrap();
    let mut current = HashSet::from([start_star]);
    stars.remove(&start_star);

    let mut total = 0;
    while let Some((star, distance)) = min_distance(&stars, &current, |_| true) {
        total += distance;

        stars.remove(&star);
        current.insert(star);
    }

    total + size
}

pub use solve as part_1;
pub use solve as part_2;

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> usize {
    let mut stars = data
        .split(|&c| c == b'\n')
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &tile)| if tile == b'*' { Some((r, c)) } else { None })
        })
        .collect::<HashSet<_>>();

    let mut brilliants = std::iter::from_fn(move || {
        if stars.is_empty() {
            return None;
        }

        if stars.len() == 1 {
            stars.clear();
            return Some(0);
        }

        let start_star = stars.iter().copied().next().unwrap();
        let mut current = HashSet::from([start_star]);
        stars.remove(&start_star);

        let mut total = 0;
        while let Some((star, distance)) =
            min_distance(&stars, &current, |distance| distance < 6)
        {
            total += distance;

            stars.remove(&star);
            current.insert(star);
        }

        Some(total + current.len())
    })
    .fuse()
    .collect::<Vec<_>>();

    brilliants.sort_unstable();

    brilliants.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            16,
            part_1(
                br"*...*
..*..
.....
.....
*.*.."
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            16,
            part_2(
                br"*...*
..*..
.....
.....
*.*.."
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            15624,
            part_3(
                br".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
......................................."
            )
        );
    }
}
