use gcd::Gcd;

trait Chunks2<I: Iterator> {
    fn chunks2(self) -> Chunks2Impl<I>;
}

struct Chunks2Impl<I: Iterator> {
    iter: std::iter::Fuse<I>,
}

impl<I: Iterator> Chunks2<I> for I {
    fn chunks2(self) -> Chunks2Impl<I> {
        Chunks2Impl { iter: self.fuse() }
    }
}

impl<T, I: Iterator<Item = T>> Iterator for Chunks2Impl<I> {
    type Item = [T; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let second = self.iter.next()?;

        Some([first, second])
    }
}

/// # Panics
fn solve(data: &str, f: impl FnOnce(u64, u64) -> u64) -> u64 {
    let (first, last) = data
        .lines()
        .map(|gear| gear.parse::<u64>().expect("invalid gear"))
        .fold((None, None), |(first, _), current| {
            if first.is_some() {
                (first, Some(current))
            } else {
                (Some(current), Some(current))
            }
        });

    f(first.unwrap(), last.unwrap())
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(data, |n, d| 2025 * n / d)
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve(data, |n, d| (10000000000000 * d).div_ceil(n))
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let (n, d) = data
        .lines()
        .flat_map(|gear| {
            gear.split('|')
                .map(|gear| gear.parse::<u64>().expect("invalid gear"))
        })
        .chunks2()
        .fold((100, 1), |(n, d), [gear_a, gear_b]| {
            let n = n * gear_a;
            let d = d * gear_b;

            let div = n.gcd(d);

            (n / div, d / div)
        });

    n / d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        assert_eq!(
            part_1(
                r"128
64
32
16
8"
            ),
            32400,
        );
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(
            part_1(
                r"102
75
50
35
13"
            ),
            15888,
        );
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_2_1() {
        assert_eq!(
            part_2(
                r"128
64
32
16
8"
            ),
            625000000000,
        );
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_2_2() {
        assert_eq!(
            part_2(
                r"102
75
50
35
13"
            ),
            1274509803922,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"5
5|10
10|20
5"
            ),
            400,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"5
7|21
18|36
27|27
10|50
10|50
11"
            ),
            6818,
        );
    }
}
