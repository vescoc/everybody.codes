use rayon::prelude::*;

pub mod bf;
pub mod z3;

#[expect(clippy::cast_possible_truncation, reason = "numbers are small")]
fn parse_iso(iso_standard: &str, i: impl Iterator<Item = u64>) -> u64 {
    i.zip(
        iso_standard
            .as_bytes()
            .iter()
            .map(|base| u64::from(base - b'0'))
            .rev()
            .cycle(),
    )
    .enumerate()
    .fold(0, |acc, (e, (digit, base))| {
        let e = e as u32;

        acc + digit * base.pow(e)
    })
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    data.par_lines()
        .map(|line| {
            let line = line.strip_prefix("ISO-").expect("'ISO-' prefix not found");
            let (iso_standard, votes) = line.split_once(' ').expect("invalid line");
            parse_iso(
                iso_standard,
                votes
                    .as_bytes()
                    .iter()
                    .map(|digit| u64::from(digit - b'0'))
                    .rev(),
            )
        })
        .sum()
}

pub use part_1 as part_2;

pub use bf::part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"ISO-3210 7654
ISO-8742 2468
ISO-1111 1111"
            ),
            1478,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"ISO-3210 7654
ISO-8742 2468
ISO-1 1111
ISO-13 777
ISO-34 140856
ISO-789 2465112
ISO-2341 945"
            ),
            1161463,
        );
    }

    #[test]
    fn test_part_3() {
        let data = include_str!("../data/part_3");

        assert_eq!(bf::part_3(data), z3::part_3(data),);
    }
}
