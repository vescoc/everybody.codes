use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

fn lookup_cmp(cmps: &HashMap<(u64, u64), Ordering>, (a, b): &(u64, u64)) -> Ordering {
    fn lookup_cmp1(cmps: &HashMap<(u64, u64), Ordering>, a: u64, b: u64, o: Ordering) -> bool {
        if let Some(cmp) = cmps.get(&(a, b))
            && *cmp == o
        {
            return true;
        }

        for ((_, c), _) in cmps.iter().filter(|((a1, _), o1)| *a1 == a && **o1 == o) {
            if lookup_cmp1(cmps, *c, b, o) {
                return true;
            }
        }

        false
    }

    if lookup_cmp1(cmps, *a, *b, Ordering::Less) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn parse_pair(pair: &str) -> (u64, Ordering, u64) {
    let pair = pair.as_bytes();
    let a = u64::from(pair[0] - b'0');
    let b = u64::from(pair[2] - b'0');
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
    lookup_cmp: impl Fn(&HashMap<(u64, u64), Ordering>, &(u64, u64)) -> Ordering + Sync + Send,
) -> u64 {
    data.par_lines()
        .map(|line| {
            let mut blades = HashSet::with_capacity(1024);
            let mut cmps = HashMap::with_capacity(1024);
            for (a, cmp, b) in line.split_ascii_whitespace().map(parse_pair) {
                blades.insert(a);
                blades.insert(b);
                cmps.insert((a, b), cmp.reverse());
                cmps.insert((b, a), cmp);
            }

            let mut blades = blades.into_iter().collect::<Vec<_>>();
            blades.sort_by(|a, b| lookup_cmp(&cmps, &(*a, *b)));

            blades.iter().fold(0, |acc, digit| acc * 10 + digit)
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(data, |cmps, pair| cmps[pair])
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
