use rayon::prelude::*;

use super::parse_iso;

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    data.par_lines()
        .filter_map(|line| {
            let line = line
                .strip_prefix("ISO2-")
                .expect("'ISO2-' prefix not found");
            let (iso_standard, votes) = line.split_once(' ').expect("invalid line");
            let candidate = parse_iso(
                iso_standard,
                votes
                    .as_bytes()
                    .iter()
                    .map(|digit| u64::from(digit - b'0'))
                    .rev(),
            );

            let target = votes
                .as_bytes()
                .iter()
                .fold(0, |acc, digit| acc * 10 + u64::from(digit - b'0'));
            for current in 1..target {
                let mut current = current;
                let iso = parse_iso(
                    iso_standard,
                    std::iter::from_fn(move || {
                        if current == 0 {
                            return None;
                        }

                        let digit = current % 10;
                        current /= 10;

                        Some(digit)
                    }),
                );
                if iso == candidate {
                    return None;
                }
            }

            Some(candidate)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"ISO2-3 23
ISO2-61 149
ISO2-978 99
ISO2-5887 5923
ISO2-76988 123749
ISO2-435657 929977
ISO2-3949 870737"
            ),
            50987,
        );
    }
}
