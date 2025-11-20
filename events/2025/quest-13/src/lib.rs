use std::ops;

fn parse_range(line: &str) -> (ops::RangeInclusive<u64>, u64) {
    let (start, end) = line.split_once('-').expect("invalid range");
    let start = start.parse::<u64>().expect("invalid start number range");
    let end = end.parse::<u64>().expect("invalid end number range");

    (ops::RangeInclusive::new(start, end), end - start + 1)
}

fn parse_single(line: &str) -> (ops::RangeInclusive<u64>, u64) {
    let value = line.parse().expect("invalid number");
    
    (ops::RangeInclusive::new(value, value), 1)
}

#[allow(clippy::cast_possible_truncation)]
fn solve<const TURNS: u64>(
    data: &str,
    parse: impl Fn(&str) -> (ops::RangeInclusive<u64>, u64),
) -> u64 {
    let mut insert_left = false;
    let mut right = vec![ops::RangeInclusive::new(1, 1)];
    let mut left = vec![];
    let mut len = 1;
    for value in data
        .lines()
        .map(|line| {
            let (range, l) = parse(line);
            len += l;
            range
        })
        .collect::<Vec<_>>()
    {
        if insert_left {
            left.push(value);
        } else {
            right.push(value);
        }
        insert_left = !insert_left;
    }

    right
        .into_iter()
        .flatten()
        .chain(left.into_iter().flatten().rev())
        .nth((TURNS % len) as usize)
        .expect("no values")
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
