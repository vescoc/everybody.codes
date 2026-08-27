//! [Story 4 Quest 2](https://everybody.codes/story/4/quests/2)

use std::collections::{HashMap, HashSet, VecDeque};

/// Errors returned from the `part*` implementations
#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    /// Invalid input data
    #[error("invalid input data")]
    InvalidInputData(&'a str),
    /// Invalid input data missing part
    #[error("invalid input data missing part")]
    MissingPart(&'static str),
}

/// Parse `<digit>+,<digit>+]`
///
/// # Returns
///
/// Coordinate
fn parse_coordinate(data: &str) -> Result<(i32, i32), Error<'_>> {
    let mut parts = data.split(',');
    let x = parts
        .next()
        .ok_or(Error::InvalidInputData(data))?
        .parse()
        .map_err(|_| Error::InvalidInputData(data))?;
    let y = parts
        .next()
        .ok_or(Error::InvalidInputData(data))?
        .split(']')
        .next()
        .ok_or(Error::InvalidInputData(data))?
        .parse()
        .map_err(|_| Error::InvalidInputData(data))?;

    Ok((x, y))
}

/// Resolve part 1
///
/// # Errors
///
/// * [`Error::InvalidInputData`] -
/// * [`Error::MissingPart`] -
#[expect(
    clippy::cast_possible_truncation,
    reason = "Truncation is requested by quest description"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Precision loss is requested by quest description"
)]
pub fn part_1(data: &str) -> Result<usize, Error<'_>> {
    let mut start = None;
    let mut beacons = HashMap::with_capacity(8);
    let mut moves = None;
    for line in data.lines() {
        if let Some(stripped_prefix) = line.strip_prefix("START=[") {
            start = Some(parse_coordinate(stripped_prefix)?);
        } else if let Some(stripped_prefix) = line.strip_prefix("MOVES=") {
            moves = Some(stripped_prefix);
        } else {
            let mut parts = line.split("=[");
            let beacon = parts
                .next()
                .ok_or(Error::InvalidInputData(line))?
                .chars()
                .next()
                .ok_or(Error::InvalidInputData(line))?;
            let coordinate = parse_coordinate(parts.next().ok_or(Error::InvalidInputData(line))?)?;
            beacons.insert(beacon, coordinate);
        }
    }

    let (mut current_x, mut current_y) = start.ok_or(Error::MissingPart("START"))?;
    let moves = moves.ok_or(Error::MissingPart("MOVES"))?;

    let mut beetles = HashSet::with_capacity(1024);
    beetles.insert((current_x, current_y));

    for beacon in moves.chars() {
        let (beacon_x, beacon_y) = *beacons.get(&beacon).ok_or(Error::InvalidInputData(data))?;
        current_x = (current_x as f32 + (beacon_x as f32 - current_x as f32) / 2.0) as i32;
        current_y = (current_y as f32 + (beacon_y as f32 - current_y as f32) / 2.0) as i32;

        beetles.insert((current_x, current_y));
    }

    Ok(beetles.len())
}

/// Resolve part 2
///
/// # Errors
///
/// * [`Error::InvalidInputData`] -
/// * [`Error::MissingPart`] -
#[expect(
    clippy::cast_possible_truncation,
    reason = "Truncation is requested by quest description"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Precision loss is requested by quest description"
)]
pub fn part_2(data: &str) -> Result<usize, Error<'_>> {
    let mut start = None;
    let mut beacons = HashMap::with_capacity(8);
    let mut moves = None;
    for line in data.lines() {
        if let Some(stripped_prefix) = line.strip_prefix("START=[") {
            start = Some(parse_coordinate(stripped_prefix)?);
        } else if let Some(stripped_prefix) = line.strip_prefix("MOVES=") {
            moves = Some(stripped_prefix);
        } else {
            let mut parts = line.split("=[");
            let beacon = parts
                .next()
                .ok_or(Error::InvalidInputData(line))?
                .chars()
                .next()
                .ok_or(Error::InvalidInputData(line))?;
            let coordinate = parse_coordinate(parts.next().ok_or(Error::InvalidInputData(line))?)?;
            beacons.insert(beacon, coordinate);
        }
    }

    let (mut current_x, mut current_y) = start.ok_or(Error::MissingPart("START"))?;
    let moves = moves.ok_or(Error::MissingPart("MOVES"))?;

    let mut beetles = HashSet::with_capacity(1024 * 8);
    beetles.insert((current_x, current_y));

    for beacon in moves.chars() {
        let (beacon_x, beacon_y) = *beacons.get(&beacon).ok_or(Error::InvalidInputData(data))?;
        current_x = (current_x as f32 + (beacon_x as f32 - current_x as f32) / 2.0) as i32;
        current_y = (current_y as f32 + (beacon_y as f32 - current_y as f32) / 2.0) as i32;

        beetles.insert((current_x, current_y));
    }

    let fireflies = beetles
        .iter()
        .flat_map(|&(x, y)| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)])
        .collect::<HashSet<_>>();

    Ok(fireflies.difference(&beetles).count())
}

/// Resolve part 2
///
/// # Errors
///
/// * [`Error::InvalidInputData`] -
/// * [`Error::MissingPart`] -
#[expect(
    clippy::cast_possible_truncation,
    reason = "Truncation is requested by quest description"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Precision loss is requested by quest description"
)]
pub fn part_3(data: &str) -> Result<usize, Error<'_>> {
    let mut start = None;
    let mut beacons = HashMap::with_capacity(8);
    for line in data.lines() {
        if let Some(stripped_prefix) = line.strip_prefix("START=[") {
            start = Some(parse_coordinate(stripped_prefix)?);
        } else {
            let mut parts = line.split("=[");
            let beacon = parts
                .next()
                .ok_or(Error::InvalidInputData(line))?
                .chars()
                .next()
                .ok_or(Error::InvalidInputData(line))?;
            let coordinate = parse_coordinate(parts.next().ok_or(Error::InvalidInputData(line))?)?;
            beacons.insert(beacon, coordinate);
        }
    }

    let (current_x, current_y) = start.ok_or(Error::MissingPart("START"))?;

    let mut beetles = HashSet::with_capacity(1024 * 32);
    beetles.insert((current_x, current_y));

    let mut queue = VecDeque::with_capacity(1024 * 32);
    queue.push_back((current_x, current_y));

    while let Some((current_x, current_y)) = queue.pop_front() {
        for &(beacon_x, beacon_y) in beacons.values() {
            let new_x = (current_x as f32 + (beacon_x as f32 - current_x as f32) / 2.0) as i32;
            let new_y = (current_y as f32 + (beacon_y as f32 - current_y as f32) / 2.0) as i32;

            if beetles.insert((new_x, new_y)) {
                queue.push_back((new_x, new_y));
            }
        }
    }

    let fireflies = beetles
        .iter()
        .flat_map(|&(x, y)| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)])
        .collect::<HashSet<_>>();

    Ok(fireflies.difference(&beetles).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]
MOVES=ABCCBABCA";
        assert_eq!(part_1(data).unwrap(), 8);
    }

    #[test]
    fn test_part_2_1() {
        let data = r"START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]
MOVES=ABCCBABCA";
        assert_eq!(part_2(data).unwrap(), 25);
    }

    #[test]
    fn test_part_2_2() {
        let data = r"START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]
MOVES=BABCAABBCABCCCBBABCCCAAACABABCBCBBCAABBABBCACCBAABCBCBBBCBBBBBCCCAACAACB";
        assert_eq!(part_2(data).unwrap(), 46);
    }

    #[test]
    fn test_part_3_1() {
        let data = r"START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]";
        assert_eq!(part_3(data).unwrap(), 42);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"START=[0,0]
A=[0,0]
B=[80,15]
C=[5,30]";
        assert_eq!(part_3(data).unwrap(), 432);
    }
}
