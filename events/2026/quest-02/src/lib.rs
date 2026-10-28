use std::cmp::Ordering;
use std::collections::HashSet;

use rayon::prelude::*;

type OrderingTable = [[Option<Ordering>; 9]; 9];

fn lookup_cmp(cmps: &OrderingTable, (a, b): &(usize, usize)) -> Ordering {
    fn lookup_cmp1(cmps: &OrderingTable, a: usize, b: usize, o: Ordering) -> Option<bool> {
        if let Some(cmp) = cmps[a][b] {
            return Some(cmp == o);
        }

        for (c, _) in cmps[a]
            .iter()
            .enumerate()
            .filter(|(_, o1)| matches!(o1, Some(o1) if *o1 == o))
        {
            if let Some(r) = lookup_cmp1(cmps, c, b, o) && r {
                return Some(r);
            }
        }

        None
    }

    if let Some(r) = lookup_cmp1(cmps, *a, *b, Ordering::Less) && r {
        Ordering::Less
    } else if let Some(r) = lookup_cmp1(cmps, *a, *b, Ordering::Greater) && r {
        Ordering::Greater
    } else {
        unreachable!("invalid groups")
    }
}

fn parse_pair(pair: &str) -> (usize, Ordering, usize) {
    let pair = pair.as_bytes();
    let a = usize::from(pair[0] - b'0');
    let b = usize::from(pair[2] - b'0');
    let cmp = match pair[1] {
        b'<' => Ordering::Less,
        b'=' => Ordering::Equal,
        b'>' => Ordering::Greater,
        c => unreachable!("invalid comparison {c}"),
    };

    (a, cmp, b)
}

/// # Panics
#[must_use]
fn solve(
    data: &str,
    lookup_cmp: impl Fn(&OrderingTable, &(usize, usize)) -> Ordering + Sync + Send,
) -> u64 {
    data.par_lines()
        .map(|line| {
            let mut blades = HashSet::with_capacity(1024);
            let mut cmps = [[None; 9]; 9];
            for (a, cmp, b) in line.split_ascii_whitespace().map(parse_pair) {
                blades.insert(a);
                blades.insert(b);

                // reversed ordering: "from hardest to the softest"
                cmps[a - 1][b - 1] = Some(cmp.reverse());
                cmps[b - 1][a - 1] = Some(cmp);
            }

            let mut blades = blades.into_iter().collect::<Vec<_>>();
            blades.sort_by(|a, b| lookup_cmp(&cmps, &(*a - 1, *b - 1)));

            blades
                .iter()
                .fold(0u64, |acc, digit| acc * 10 + *digit as u64)
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(data, |cmps, (a, b)| cmps[*a][*b].unwrap())
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve(data, lookup_cmp)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    solve(data, lookup_cmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"4<3 2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
4>1 2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 2<4 3>2 3<4 1>2 3>5 1<3 4>5"
            ),
            122157,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 3>2 3<4 1>2 3>5 1<3 4>5"
            ),
            122157,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"3>1 1>2 1<5 4<2 3>5
2>1 2<5 3>1 4>3 2<4 5<3
5>1 3<4 1>2 3>5"
            ),
            122157,
        );
    }
}
