use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete;

fn parse_x_y(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = complete::u64(input)?;
    let (input, _) = tag(" y=")(input)?;
    let (input, y) = complete::u64(input)?;

    Ok((input, (x, y)))
}

fn parse_x_y_width(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = complete::u64(input)?;
    let (input, _) = tag(" y=")(input)?;
    let (input, y) = complete::u64(input)?;
    let (input, _) = tag(" width=")(input)?;
    let (input, width) = complete::u64(input)?;

    Ok((input, (x, y, width)))
}

fn accumulate((max_x, _): (u64, u64), stacks: impl Iterator<Item = (u64, u64)>) -> u64 {
    stacks
        .take_while(|(x, _)| *x != max_x)
        .fold(
            (0, 0, 0),
            |(mut total, mut current_y, mut delta_y), (x, y)| {
                if y > current_y {
                    delta_y = y - current_y;
                    current_y = y;
                    total += delta_y * x.abs_diff(max_x);
                }

                (total, current_y, delta_y)
            },
        )
        .0
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let (mut max_x, mut max_y) = (u64::MIN, u64::MIN);
    let stacks = data
        .lines()
        .map(|line| {
            let (_, (x, y)) = parse_x_y(line).expect("invalid line");

            if y > max_y {
                (max_x, max_y) = (x, y);
            }
            (x, y)
        })
        .collect::<Vec<_>>();

    accumulate((max_x, max_y), stacks.iter().copied())
        + accumulate((max_x, max_y), stacks.iter().rev().copied())
        + max_y
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let (mut max_x, mut max_y, mut max_width) = (u64::MIN, u64::MIN, u64::MIN);
    let stacks = data
        .lines()
        .map(|line| {
            let (_, (x, y, width)) = parse_x_y_width(line).expect("invalid line");

            if y > max_y {                
                (max_x, max_y, max_width) = (x, y, width);
            }
            (x, y, width)
        })
        .collect::<Vec<_>>();

    accumulate((max_x, max_y), stacks.iter().map(|(x, y, _)| (*x, *y))) + max_y * max_width + accumulate(
            (max_x + max_width, max_y),
            stacks.iter().rev().map(|(x, y, width)| (x + width, *y)),
        )
}

pub use part_2 as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"x=0 y=5
x=5 y=7
x=6 y=8
x=7 y=1
x=8 y=3
x=10 y=6
x=15 y=3
x=18 y=4"
            ),
            96,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"x=0 y=5 width=1
x=1 y=4 width=3
x=5 y=3 width=2
x=7 y=9 width=1
x=8 y=8 width=1
x=9 y=9 width=3
x=12 y=3 width=1
x=16 y=7 width=1
x=19 y=3 width=5
x=25 y=5 width=1
x=29 y=1 width=2
x=31 y=6 width=1"
            ),
            205,
        );
    }
}
