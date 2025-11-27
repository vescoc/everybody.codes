use std::cmp;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64 as parse_i64, newline, usize as parse_usize},
    combinator::{map_res, opt},
    multi::many1,
};

trait Activation {
    fn incoming_energy(&self, id: usize) -> i64;
}

#[derive(Copy, Clone)]
struct FullActivation;

impl Activation for FullActivation {
    fn incoming_energy(&self, _: usize) -> i64 {
        1
    }
}

#[derive(Copy, Clone)]
struct SimpleActivation<'a>(&'a [i64]);

impl Activation for SimpleActivation<'_> {
    fn incoming_energy(&self, id: usize) -> i64 {
        self.0[id - 1]
    }
}

#[derive(Copy, Clone)]
struct OrdActivation<'a>(&'a [cmp::Ordering]);

impl Activation for OrdActivation<'_> {
    fn incoming_energy(&self, id: usize) -> i64 {
        i64::from(matches!(self.0[id - 1], cmp::Ordering::Greater))
    }
}

#[derive(Debug)]
struct Plant {
    id: usize,
    thickness: i64,
    info: PlantInfo,
}

#[derive(Debug)]
enum PlantInfo {
    Free,
    Branches(Vec<(usize, i64)>),
}

fn parse_branch(input: &str) -> IResult<&str, (usize, i64)> {
    let (input, (_, id, _, thickness, ..)) = (
        tag("- branch to Plant "),
        parse_usize,
        tag(" with thickness "),
        parse_i64,
        opt(newline),
    )
        .parse(input)?;

    Ok((input, (id, thickness)))
}

fn parse_free(input: &str) -> IResult<&str, PlantInfo> {
    let (input, _) = tag("- free branch with thickness 1").parse(input)?;

    Ok((input, PlantInfo::Free))
}

fn parse_branches(input: &str) -> IResult<&str, PlantInfo> {
    let (input, result) = many1(parse_branch).parse(input)?;

    Ok((input, PlantInfo::Branches(result)))
}

fn parse_plant(plant: &str) -> IResult<&str, Plant> {
    let (input, (_, id, _, thickness, _, _, info)) = (
        tag("Plant "),
        parse_usize,
        tag(" with thickness "),
        parse_i64,
        tag(":"),
        newline,
        alt((parse_free, parse_branches)),
    )
        .parse(plant)?;

    Ok((
        input,
        Plant {
            id,
            thickness,
            info,
        },
    ))
}

fn energy(
    plants: &[Plant],
    activation: impl Activation + Copy,
    Plant {
        id,
        info,
        thickness,
    }: &Plant,
) -> i64 {
    let energy = match info {
        PlantInfo::Free => activation.incoming_energy(*id),
        PlantInfo::Branches(branches) => branches
            .iter()
            .map(|(id, mul)| energy(plants, activation, &plants[*id - 1]) * mul)
            .sum(),
    };
    if energy >= *thickness { energy } else { 0 }
}

fn find_max(plants: &[Plant]) -> i64 {
    fn find_max_r(
        plants: &[Plant],
        mut activation: Vec<cmp::Ordering>,
        plant: &Plant,
        mut index: usize,
    ) -> i64 {
        loop {
            if index == activation.len() {
                return energy(plants, OrdActivation(&activation), plant);
            }

            match activation[index] {
                cmp::Ordering::Greater | cmp::Ordering::Less => {
                    index += 1;
                }
                cmp::Ordering::Equal => {
                    activation[index] = cmp::Ordering::Greater;
                    let a = find_max_r(plants, activation.clone(), plant, index);

                    activation[index] = cmp::Ordering::Less;
                    let b = find_max_r(plants, activation, plant, index);

                    return a.max(b);
                }
            }
        }
    }

    let activation = plants
        .iter()
        .filter_map(|Plant { id, info, .. }| match info {
            PlantInfo::Free => {
                let (up, down) = plants
                    .iter()
                    .filter_map(|Plant { info, .. }| {
                        if let PlantInfo::Branches(v) = info {
                            v.iter().find_map(|(source_id, source_thickness)| {
                                if source_id == id {
                                    Some(*source_thickness)
                                } else {
                                    None
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .partition::<Vec<_>, _>(|value| *value >= 0);

                Some(up.len().cmp(&down.len()))
            }
            PlantInfo::Branches(_) => None,
        })
        .collect::<Vec<_>>();

    find_max_r(plants, activation, plants.last().unwrap(), 0)
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> i64 {
    let (_, plants) = many1(map_res((parse_plant, opt(many1(newline))), |(plant, _)| {
        Ok::<_, nom::Err<&str>>(plant)
    }))
    .parse(data)
    .expect("Invalid input");
    assert!(
        plants
            .iter()
            .enumerate()
            .all(|(i, Plant { id, .. })| i + 1 == *id)
    );

    let last_plant = plants.last().expect("Cannot find last plant");

    energy(&plants, FullActivation, last_plant)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> i64 {
    let (input, plants) = many1(map_res((parse_plant, opt(many1(newline))), |(plant, _)| {
        Ok::<_, nom::Err<&str>>(plant)
    }))
    .parse(data)
    .expect("Invalid input");
    assert!(
        plants
            .iter()
            .enumerate()
            .all(|(i, Plant { id, .. })| i + 1 == *id)
    );

    let last_plant = plants.last().expect("Cannot find last plant");

    input
        .lines()
        .map(|line| {
            let activation = line
                .split_whitespace()
                .map(|value| value.parse::<i64>().expect("Invalid activation"))
                .collect::<Vec<_>>();

            energy(&plants, SimpleActivation(&activation), last_plant)
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> i64 {
    let (input, plants) = many1(map_res((parse_plant, opt(many1(newline))), |(plant, _)| {
        Ok::<_, nom::Err<&str>>(plant)
    }))
    .parse(data)
    .expect("Invalid input");
    assert!(
        plants
            .iter()
            .enumerate()
            .all(|(i, Plant { id, thickness, .. })| i + 1 == *id && *thickness > 0)
    );

    let max = find_max(&plants);

    let last_plant = plants.last().expect("Cannot find last plant");
    input
        .lines()
        .map(|line| {
            let activation = line
                .split_whitespace()
                .map(|value| value.parse::<i64>().expect("Invalid activation"))
                .collect::<Vec<_>>();

            let energy = energy(&plants, SimpleActivation(&activation), last_plant);
            if energy != 0 { max - energy } else { 0 }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34"
            ),
            774
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1"
            ),
            324
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 1:
- free branch with thickness 1

Plant 5 with thickness 8:
- branch to Plant 1 with thickness -8
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13
- branch to Plant 4 with thickness -7

Plant 6 with thickness 7:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -9
- branch to Plant 3 with thickness 12
- branch to Plant 4 with thickness 9

Plant 7 with thickness 23:
- branch to Plant 5 with thickness 17
- branch to Plant 6 with thickness 18


0 1 0 0
0 1 0 1
0 1 1 1
1 1 0 1"
            ),
            946
        );
    }
}
