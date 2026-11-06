use std::fmt::Debug;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::{IResult, Parser};

use rayon::prelude::*;

fn parse_numeric_tag<F>(t: &str) -> impl Fn(&str) -> IResult<&str, F>
where
    F: FromStr,
    F::Err: Debug,
{
    move |input| {
        let (input, _) = tag(t)(input)?;
        let (input, value) = digit1(input)?;
        Ok((input, value.parse().expect("invalid number")))
    }
}

fn parse_line<F>(input: &str) -> IResult<&str, (usize, F, u64)>
where
    F: FromStr,
    F::Err: Debug,
{
    let (input, (id, _, growth, _, limit)) = (
        parse_numeric_tag("id="),
        tag(", "),
        parse_numeric_tag::<F>("growth="),
        tag(", "),
        parse_numeric_tag("limit="),
    )
        .parse(input)?;

    Ok((input, (id, growth, limit)))
}

fn solve(data: &str, condition: impl Fn(&[u64], u64) -> bool + Sync + Send) -> u64 {
    data.par_lines()
        .map(|line| {
            let (_, (_, growth, limit)) = parse_line::<usize>(line).expect("invalid line");

            debug_assert!(growth % 2 == 1 && growth > 1 && limit > 0);

            let mut time = 1;

            let mut plant = vec![0; growth];
            plant[1] = 1;

            while condition(&plant, limit) {
                time += 1;

                let mut next_plant = vec![0; growth];
                next_plant[0] = plant[0];

                for index in 1..growth {
                    if index == growth - 1 {
                        let branches = plant[index];

                        next_plant[0] += branches;
                        next_plant[1] += branches;
                        next_plant[growth / 2] += branches * 2;
                    } else {
                        next_plant[index + 1] += plant[index];
                    }
                }

                plant = next_plant;
            }

            time
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(data, |plant, limit| {
        plant
            .iter()
            .enumerate()
            .map(|(index, &parts)| {
                if index == 0 {
                    parts
                } else {
                    parts * index as u64
                }
            })
            .sum::<u64>()
            < limit
    })
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    use std::collections::BTreeMap;

    data.par_lines()
        .map(|line| {
            let (_, (_, growth, limit)) = parse_line(line).expect("invalid line");

            debug_assert!(growth % 2 == 1 && growth > 1 && limit > 0);

            let mut timeline = BTreeMap::from([(growth, 1)]);

            let mut time = 1;
            let mut leaves = 1;

            while leaves < limit {
                let (current_time, count) = timeline.pop_first().unwrap();

                *timeline.entry(current_time + growth / 2 + 1).or_default() += count * 2;
                *timeline.entry(current_time + growth - 1).or_default() += count;

                leaves += count;

                time = current_time;
            }

            time
        })
        .sum()
}

pub use part_2 as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"id=1, growth=3, limit=10
id=2, growth=5, limit=20
id=3, growth=7, limit=30"
            ),
            27,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"id=1, growth=3, limit=10
id=2, growth=5, limit=20
id=3, growth=7, limit=30"
            ),
            42,
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"id=1, growth=3, limit=10
id=2, growth=5, limit=20
id=3, growth=7, limit=30"
            ),
            42,
        );
    }
}
