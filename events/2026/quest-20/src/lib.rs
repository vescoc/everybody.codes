use std::fmt;

use rayon::prelude::*;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator;
use nom::multi::fold_many1;
use nom::{IResult, Parser};

type Expr = (Op, i64, i64);

enum Op {
    Add,
    Sub,
}

struct U64(u64);

impl fmt::Binary for U64 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut value = self.0;

        let mut result = Vec::with_capacity(1024);
        while value != 0 {
            let digit = value % 11;
            value /= 11;

            let c = match digit {
                0..=9 => char::from(b'0' + digit as u8),
                10 => 'A',
                _ => unreachable!("illegal value: {digit}"),
            };

            result.push(c);
        }

        if result.is_empty() {
            write!(fmt, "0")?;
        } else {
            for c in result.into_iter().rev() {
                write!(fmt, "{c}")?;
            }
        }

        Ok(())
    }
}

fn value_base_11(input: &[u8]) -> IResult<&[u8], i64> {
    fold_many1(
        complete::one_of("0123456789A"),
        || 0i64,
        |acc, item| {
            let digit = item.to_digit(11).unwrap();
            acc * 11 + i64::from(digit)
        },
    )
    .parse(input)
}

fn parse_expr<'a>(
    input: &'a [u8],
    parse_value: impl Fn(&'a [u8]) -> IResult<&'a [u8], i64>,
) -> IResult<&'a [u8], Expr> {
    let (input, value1) = parse_value(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, op) = combinator::map(complete::one_of("+-"), |c| {
        if c == '+' { Op::Add } else { Op::Sub }
    })
    .parse(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, value2) = parse_value(input)?;

    Ok((input, (op, value1, value2)))
}

fn parse_line<'a>(
    input: &'a [u8],
    parse_value: impl (Fn(&'a [u8]) -> IResult<&'a [u8], i64>) + Copy,
) -> IResult<&'a [u8], (Expr, Expr)> {
    let (input, expr1) = parse_expr(input, parse_value)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, expr2) = parse_expr(input, parse_value)?;

    Ok((input, (expr1, expr2)))
}

fn segments(line: &[u8]) -> u64 {
    line.iter()
        .filter_map(|c| match c {
            b'-' => Some(1),
            b'1' | b'+' | b'=' => Some(2),
            b'7' => Some(3),
            b'4' => Some(4),
            b'2' | b'3' | b'5' => Some(5),
            b'0' | b'6' | b'9' | b'A' => Some(6),
            b'8' => Some(7),
            _ => None,
        })
        .sum()
}

fn move_segment(digit: u8) -> &'static [u8] {
    match digit {
        // 5 segments
        b'2' | b'5' => b"3",
        b'3' => b"25",

        // 6 segments
        b'0' => b"69A",
        b'6' => b"0A9",
        b'9' => b"06A",
        b'A' => b"069",

        // no move possible
        _ => &[],
    }
}

fn add_segment(digit: u8) -> &'static [u8] {
    match digit {
        // 1 segment
        b'-' => b"+",

        // 2 segments
        b'1' => b"7",

        // 5 segments
        b'3' => b"9",
        b'5' => b"69",

        // 6 segments
        b'0' | b'6' | b'9' | b'A' => b"8",

        // no add possible
        _ => &[],
    }
}

fn remove_segment(digit: u8) -> &'static [u8] {
    match digit {
        // 2 segment
        b'+' => b"-",

        // 3 segments
        b'7' => b"1",

        // 6 segments
        b'9' => b"35",
        b'6' => b"5",

        // 7 segments
        b'8' => b"069A",

        // no add possible
        _ => &[],
    }
}

fn eval((op, l, r): (Op, i64, i64)) -> i64 {
    match op {
        Op::Add => l + r,
        Op::Sub => l - r,
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    // no rayon if input is small
    data.lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (_, (expr1, expr2)) = parse_line(line, complete::i64).expect("invalid line");
            if eval(expr1) == eval(expr2) {
                Some(segments(line))
            } else {
                None
            }
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> String {
    let sum = data
        .par_lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (_, (expr1, expr2)) = parse_line(line, value_base_11).expect("invalid line");
            if eval(expr1) == eval(expr2) {
                Some(segments(line))
            } else {
                None
            }
        })
        .sum::<u64>();

    format!("{:b}", U64(sum))
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> String {
    fn try_move_segment(line: &[u8]) -> Option<Vec<u8>> {
        let mut line = line.to_vec();
        for index in 0..line.len() {
            let target = line[index];
            for candidate in move_segment(target) {
                line[index] = *candidate;

                let (_, (expr1, expr2)) = parse_line(&line, value_base_11).expect("invalid line");

                if eval(expr1) == eval(expr2) {
                    return Some(line);
                }
            }
            line[index] = target;
        }

        None
    }

    fn try_add_remove_segment(line: &[u8]) -> Option<Vec<u8>> {
        let mut line = line.to_vec();
        for index in 0..line.len() {
            let target1 = line[index];
            for candidate1 in remove_segment(target1) {
                line[index] = *candidate1;
                for other_index in 0..line.len() {
                    if index == other_index {
                        continue;
                    }

                    let target2 = line[other_index];
                    for candidate2 in add_segment(target2) {
                        line[other_index] = *candidate2;

                        let (_, (expr1, expr2)) =
                            parse_line(&line, value_base_11).expect("invalid line");

                        if eval(expr1) == eval(expr2) {
                            return Some(line);
                        }
                    }
                    line[other_index] = target2;
                }
            }
            line[index] = target1;
        }

        None
    }

    let sum = data
        .par_lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (_, (expr1, expr2)) = parse_line(line, value_base_11).expect("invalid line");

            (eval(expr1) == eval(expr2))
                .then(|| segments(line))
                .or_else(|| try_move_segment(line).map(|line| segments(&line)))
                .or_else(|| try_add_remove_segment(line).map(|line| segments(&line)))
        })
        .sum::<u64>();

    format!("{:b}", U64(sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"5 - 4 = 9 - 8
2 + 1 = 3 + 0
6 + 7 = 4 + 9
1 + 2 = 3 + 4"
            ),
            75,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"10 + 2A = 13 + 27
98 - 54 = 76 - 32
10 + 12 = 30 - 9
20 - 10 = 5 + 5"
            ),
            "101",
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"1 + 1 = 5 - 1
1 + 1 = 3 - 3
0 - 1 = 1 + 7
1 + 1 = 1 - 6
1 + 1 = 2 - 1
1 + 1 = 5 + 7
1 + 4 = 9 - 9
1 + 1 = 2 + 9
1 + 1 = 1 + 2"
            ),
            "125",
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"4469 - 5934 = 1631 - 2AA7
8471 - 2276 = 4510 + 2736
A767 - 6104 = 1825 + 2538
9961 + 1140 = 5963 + 2974
9188 - 406A = 3598 + 1533
96A6 - 1606 = 4676 - 3524
9114 - 15A2 = 5258 + 2371
7023 - 2209 = 1433 + 3436
921A - 2696 = 2A91 + 3688
8492 - 2055 = 4186 + 3260
1234 + 4567 = 8901 + 2345"
            ),
            "693",
        );
    }
}
