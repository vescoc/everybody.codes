use nom::combinator::map_res;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

type Hands = Vec<i64>;
type Directions = Vec<i64>;
type Current = Vec<i64>;
type Move = Vec<i64>;

fn parse_direction(input: &str) -> Result<i64, &str> {
    match input {
        "CW" => Ok(1),
        "CCW" => Ok(-1),
        _ => Err(input),
    }
}

fn parse_directions() -> impl Fn(&str) -> IResult<&str, Directions> {
    move |input| {
        let (input, _) = tag("direction")(input)?;
        let (input, _) = complete::space1(input)?;

        separated_list1(tag(":"), map_res(complete::alphanumeric1, parse_direction)).parse(input)
    }
}

fn parse_numerics(t: &str) -> impl Fn(&str) -> IResult<&str, Vec<i64>> {
    move |input| {
        let (input, _) = tag(t)(input)?;
        let (input, _) = complete::space1(input)?;

        separated_list1(tag(":"), complete::i64).parse(input)
    }
}

fn parse_data(input: &str) -> IResult<&str, (Hands, Directions, Current, Move)> {
    let (input, (hands, _, directions, _, currents, _, moves)) = (
        parse_numerics("hands"),
        complete::line_ending,
        parse_directions(),
        complete::line_ending,
        parse_numerics("current"),
        complete::line_ending,
        parse_numerics("move"),
    )
        .parse(input)?;

    Ok((input, (hands, directions, currents, moves)))
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> i64 {
    data.split("\n\n")
        .map(|data| {
            let (_, (hands, mut directions, mut currents, moves)) =
                parse_data(data).expect("invalid data");

            let n = hands.len();

            let lcm = hands.iter().copied().reduce(lcm).unwrap();

            for (hand, (current, direction)) in hands
                .iter()
                .zip(currents.iter_mut().zip(directions.iter_mut()))
            {
                *current = *current * lcm / hand;
                *direction = *direction * lcm / hand;
            }

            let mut times = moves
                .iter()
                .copied()
                .chain(std::iter::once(0))
                .collect::<Vec<_>>();
            times.sort_unstable();

            let mut signals = 0;
            let mut dv = vec![0; n];
            for (time_1, time_2) in times.iter().zip(times.iter().skip(1)) {
                for (dv, (&mv, &direction)) in dv.iter_mut().zip(moves.iter().zip(&directions)) {
                    *dv = if *time_2 <= mv {
                        (time_2 - time_1) * direction
                    } else {
                        0
                    };
                }

                for (i, (&current_1, &dv_1)) in currents.iter().zip(&dv).enumerate() {
                    for (&current_2, &dv_2) in currents.iter().zip(&dv).skip(i + 1) {
                        let d = i64::from(dv_1 < dv_2);

                        let signal = ((current_1 + dv_1 - (current_2 + dv_2) - d).div_euclid(lcm)
                            - (current_1 - current_2 - d).div_euclid(lcm))
                        .abs();

                        signals += signal;
                    }
                }

                for (current, dv) in currents.iter_mut().zip(&dv) {
                    *current = (*current + *dv).rem_euclid(lcm);
                }
            }

            signals
        })
        .sum()
}

pub use solve as part_1;
pub use solve as part_2;
pub use solve as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        assert_eq!(
            part_1(
                r"hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4"
            ),
            5,
        );
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(
            part_1(
                r"hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:15"
            ),
            7,
        );
    }

    #[test]
    fn test_part_1_3() {
        assert_eq!(
            part_1(
                r"hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:20"
            ),
            9,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4

hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:15

hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:20"
            ),
            21,
        );
    }
}
