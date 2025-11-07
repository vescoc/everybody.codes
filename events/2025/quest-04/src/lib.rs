use gcd::Gcd;
use itertools::Itertools;

/// # Panics
fn reduce(data: &str) -> Option<((u64, u64), u64)> {
    data.lines()
        .map(|gear| ((1, 1), gear.parse::<u64>().expect("invalid gear")))
        .reduce(|((n_1, d_1), gear_1), ((n_2, d_2), gear_2)| {
            let n = n_1 * n_2 * gear_1;
            let d = d_1 * d_2 * gear_2;
            let div = n.gcd(d);
            ((n / div, d / div), gear_2)
        })
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    reduce(data).map(|((n, d), _)| 2025 * n / d).unwrap()
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_2(data: &str) -> u64 {
    reduce(data)
        .map(|((n, d), _)| {
            let d = 10000000000000 * d;
            d / n + u64::from(d % n != 0)
        })
        .unwrap()
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
        .chunks(2)
        .into_iter()
        .fold((1, 1), |(n, d), mut chunk| {
            let gear_a = chunk.next().unwrap();
            let gear_b = chunk.next().unwrap();

            let n = n * gear_a;
            let d = d * gear_b;

            let div = n.gcd(d);

            (n / div, d / div)
        });

    100 * n / d
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
