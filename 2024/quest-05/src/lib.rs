use std::collections::VecDeque;

use hashbrown::HashMap;

type Columns = [VecDeque<u32>; 4];

fn parse(data: &[u8]) -> u32 {
    data.iter()
        .fold(0, |acc, value| acc * 10 + u32::from(value - b'0'))
}

fn parse_input(data: &[u8]) -> Columns {
    let mut columns = [
        VecDeque::with_capacity(200),
        VecDeque::with_capacity(200),
        VecDeque::with_capacity(200),
        VecDeque::with_capacity(200),
    ];

    for row in data.split(|&c| c == b'\n') {
        for (i, c) in row.split(|&c| c == b' ').enumerate() {
            columns[i].push_back(parse(c));
        }
    }

    columns
}

fn round(columns: &mut Columns, c: usize) {
    let clapper = columns[c].pop_front().unwrap();

    let front = (c + 1) % 4;

    let column_len = columns[front].len();

    let index = (clapper - 1) as usize % (column_len * 2);
    if index < column_len {
        columns[front].insert(index, clapper);
    } else {
        columns[front].insert(column_len * 2 - index, clapper);
    }
}

fn number(columns: &Columns) -> u64 {
    columns.iter().fold(0, |acc, column| {
        let front = u64::from(*column.front().unwrap());
        if front > 999 {
            acc * 10000 + front
        } else if front > 99 {
            acc * 1000 + front
        } else if front > 9 {
            acc * 100 + front
        } else {
            acc * 10 + front
        }
    })
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> u64 {
    let mut columns = parse_input(data);

    (0..4).cycle().take(10).for_each(|c| round(&mut columns, c));

    number(&columns)
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> u64 {
    let mut columns = parse_input(data);

    let mut numbers = HashMap::<u64, usize>::new();
    for (i, c) in (0..4).cycle().enumerate() {
        round(&mut columns, c);

        let number = number(&columns);

        let entry = numbers.entry(number).or_default();
        *entry += 1;
        if *entry == 2024 {
            return number * (i + 1) as u64;
        }
    }

    unreachable!()
}

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> u64 {
    let mut columns = parse_input(data);

    let mut max = 0;
    let mut d = 0;
    for c in (0..4).cycle() {
        round(&mut columns, c);

        let number = number(&columns);
        if number > max {
            max = number;
            d = 0;
        } else {
            d += 1;
            if d > 1_000 {
                return max;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            2323,
            part_1(
                br"2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            50877075,
            part_2(
                br"2 3 4 5
6 7 8 9"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            6584,
            part_3(
                br"2 3 4 5
6 7 8 9"
            )
        );
    }
}
