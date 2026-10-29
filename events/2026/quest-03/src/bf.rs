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

            let bases_len = votes.len();
            let mut bases = [0; 1024];
            for (base, digit) in bases[..bases_len].iter_mut().rev().zip(
                iso_standard
                    .as_bytes()
                    .iter()
                    .map(|base| u64::from(base - b'0'))
                    .rev()
                    .cycle(),
            ) {
                *base = digit;
            }

            let iso = to_iso2(&bases[..bases_len], candidate);
            if iso == Some(target) {
                Some(candidate)
            } else {
                None
            }
        })
        .sum()
}

#[must_use]
#[expect(clippy::cast_possible_truncation, reason = "numbers are small")]
pub fn to_iso2(bases: &[u64], value: u64) -> Option<u64> {
    fn to_iso2_r(k: u32, infos: &[(u64, u64)], target: u64) -> Option<u64> {
        let Some(((weight, upperbound), infos)) = infos.split_first() else {
            if target == 0 {
                return Some(0);
            }
            return None;
        };

        for digit in 0..10 {
            let current = digit * weight;
            if current > target {
                break;
            }

            let remainder = target - current;
            if remainder > *upperbound {
                continue;
            }

            if let Some(iso) = to_iso2_r(k - 1, infos, remainder) {
                return Some(digit * 10u64.pow(k - 1) + iso);
            }
        }

        None
    }

    let len = bases.len();
    let mut infos = [(0, 0); 1024];
    for (index, (base, (weight, upperbound))) in bases.iter().zip(infos.iter_mut()).enumerate() {
        let e = (len - index - 1) as u32;

        *weight = base.pow(e);
        *upperbound = 10u64.pow(e) - 1;
    }

    to_iso2_r(len as u32, &infos[..len], value)
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

    #[test]
    fn test_23() {
        assert_eq!(to_iso2(&[3], 9), Some(9));
    }

    #[test]
    fn test_61() {
        assert_eq!(to_iso2(&[6, 1], 34), Some(54));
    }

    #[test]
    fn test_978() {
        assert_eq!(to_iso2(&[9, 7, 8], 72), Some(99));
    }

    #[test]
    fn test_5887() {
        assert_eq!(to_iso2(&[5, 8, 8, 7], 1220), Some(5923));
    }
}
