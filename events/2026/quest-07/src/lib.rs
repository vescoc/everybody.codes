use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
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

fn parse_connection(input: &str) -> IResult<&str, usize> {
    let (input, value) = digit1(input)?;
    Ok((input, value.parse().expect("invalid id")))
}

fn parse_connections(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("connections=")(input)?;
    separated_list1(tag(","), parse_connection).parse(input)
}

/// # Panics
#[must_use]
fn solve<const IMPULSE: u64>(data: &str) -> u64 {
    fn calc(
        memoize: &mut HashMap<(usize, u64), u64>,
        clouds: &[Vec<usize>],
        id: usize,
        impulse: u64,
    ) -> u64 {
        let key = (id, impulse);
        if let Some(value) = memoize.get(&key) {
            return *value;
        }

        let value = if impulse > 1 {
            impulse
                + clouds[id - 1]
                    .iter()
                    .map(|id| calc(memoize, clouds, *id, impulse - 1))
                    .sum::<u64>()
        } else {
            1
        };

        memoize.insert(key, value);

        value
    }

    let mut start_id = usize::MAX;
    let mut max_connections = 0;
    let mut static_energy = 0u64;
    let clouds = data
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (_, (id, _, energy, _, connections)) = (
                parse_numeric_tag("id="),
                tag(", "),
                parse_numeric_tag::<u64>("energy="),
                tag(", "),
                parse_connections,
            )
                .parse(line)
                .expect("invalid line");

            assert_eq!(index + 1, id);

            static_energy += energy;

            if connections.len() > max_connections {
                start_id = id;
                max_connections = connections.len();
            }

            connections
        })
        .collect::<Vec<_>>();

    let mut memoize = HashMap::with_capacity(1024);

    let impulse_energy = calc(&mut memoize, &clouds, start_id, IMPULSE);

    static_energy + impulse_energy
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve::<5>(data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve::<10>(data)
}

/// # Panics
#[must_use]
pub fn part_3<const TRIGGER_ENERGY: u64>(data: &str) -> u64 {
    fn calc<const TRIGGER_ENERGY: u64>(
        clouds: &[Vec<usize>],
        running: &mut [bool],
        energy: &mut [u64],
        mut tokens: Vec<(u64, Vec<u64>)>,
    ) {
        let len = clouds.len();

        while !tokens.is_empty() {
            let mut new_tokens = vec![];

            // propagate impulses
            let mut add = false;
            for (level, token) in tokens {
                let mut new_token = vec![0; len];
                for ((_, connections), (energy, &impulses)) in running
                    .iter()
                    .zip(clouds.iter())
                    .zip(energy.iter_mut().zip(token.iter()))
                    .filter(|((running, _), (_, impulses))| **running && **impulses > 0)
                {
                    *energy += level * impulses;
                    if level > 1 {
                        for &id in connections {
                            add = true;
                            new_token[id - 1] += impulses;
                        }
                    }
                }

                if add {
                    new_tokens.push((level - 1, new_token));
                }
            }

            // handle explosions
            for (running, (_, connections)) in running
                .iter_mut()
                .zip(energy.iter().zip(clouds.iter()))
                .filter(|(running, (energy, _))| **running && **energy >= TRIGGER_ENERGY)
            {
                *running = false;

                let mut token = vec![0; len];
                for &id in connections {
                    token[id - 1] += 1;
                }

                new_tokens.push((10, token));
            }

            tokens = new_tokens;
        }
    }

    let mut start_energy = Vec::with_capacity(1024);
    let clouds = data
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (_, (id, _, energy, _, connections)) = (
                parse_numeric_tag("id="),
                tag(", "),
                parse_numeric_tag::<u64>("energy="),
                tag(", "),
                parse_connections,
            )
                .parse(line)
                .expect("invalid line");

            assert_eq!(index + 1, id);

            start_energy.push(energy);

            connections
        })
        .collect::<Vec<_>>();

    clouds
        .par_iter()
        .enumerate()
        .map(|(id, _)| {
            let len = clouds.len();
            let mut token = vec![0; len];
            token[id] = 1;

            let mut running = vec![true; len];
            let mut energy = start_energy.clone();

            calc::<TRIGGER_ENERGY>(&clouds, &mut running, &mut energy, vec![(10, token)]);

            running
                .into_iter()
                .zip(energy)
                .map(|(running, energy)| if running { energy } else { 0 })
                .sum()
        })
        .max()
        .expect("invalid input data")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5"
            ),
            102,
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            part_2(
                r"id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5"
            ),
            1316,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_2_2() {
        assert_eq!(
            part_2(
                r"id=1, energy=5, connections=2,3,5,7
id=2, energy=9, connections=1,3,4,5,6,7
id=3, energy=3, connections=1,2,4,6,7
id=4, energy=8, connections=1,2,7
id=5, energy=1, connections=2,3,4,6
id=6, energy=7, connections=1,2,3,4
id=7, energy=6, connections=1,2,4,6"
            ),
            1214633,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3::<100>(
                r"id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5"
            ),
            187,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3_2() {
        assert_eq!(
            part_3::<1000000>(include_str!("../data/example_part_3")),
            34777023,
        );
    }
}
