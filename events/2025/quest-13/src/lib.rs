use std::ops;

fn parse_range(line: &str) -> (ops::RangeInclusive<u64>, u64) {
    let (start, end) = line.split_once('-').expect("invalid line");
    let start = start.parse::<u64>().expect("invalid start number range");
    let end = end.parse::<u64>().expect("invalid end number range");

    (ops::RangeInclusive::new(start, end), end - start + 1)
}

fn parse_single(line: &str) -> (ops::RangeInclusive<u64>, u64) {
    let value = line.parse().expect("invalid number");

    (ops::RangeInclusive::new(value, value), 1)
}

struct Info(Vec<ops::RangeInclusive<u64>>, u64);

impl Default for Info {
    fn default() -> Self {
	Self(Vec::with_capacity(512), 0)
    }
}

impl Extend<(usize, (ops::RangeInclusive<u64>, u64))> for Info {
    fn extend<II: IntoIterator<Item = (usize, (ops::RangeInclusive<u64>, u64))>>(
        &mut self,
        ii: II,
    ) {
        for (_, (r, l)) in ii {
            self.0.push(r);
            self.1 += l;
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::match_single_binding)]
fn solve<const TURNS: u64>(
    data: &str,
    parse: impl Fn(&str) -> (ops::RangeInclusive<u64>, u64),
) -> u64 {
    match data
        .lines()
        .map(parse)
        .enumerate()
        .partition(|(n, _)| n % 2 == 0)
    {
        (Info(right, right_len), Info(left, left_len)) => std::iter::once(1)
            .chain(right.into_iter().flatten())
            .chain(left.into_iter().flatten().rev())
            .nth((TURNS % (right_len + left_len + 1)) as usize)
            .expect("No result!!!"),
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve::<2025>(data, parse_single)
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve::<20252025>(data, parse_range)
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_3(data: &str) -> u64 {
    solve::<202520252025>(data, parse_range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"72
58
47
61
67"
            ),
            67
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"10-15
12-13
20-21
19-23
30-37"
            ),
            30
        );
    }
}
