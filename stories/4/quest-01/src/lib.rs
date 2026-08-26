//! [Story 4 Quest 1](https://everybody.codes/story/4/quests/1)

use std::collections::HashSet;

use rayon::prelude::*;

/// Initial set/list capacity.
///
/// Euristic: 1024 is good for actual inputs.
const INITIAL_CAPACITY: usize = 1024;

/// Errors returned from the `part*` implementations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Invalid input number
    #[error("invalid number")]
    InvalidNumber,
}

/// Resolve part 1
///
/// # Errors
///
/// * [`Error::InvalidNumber`] - if a jump is invalid.
pub fn part_1(data: &str) -> Result<i32, Error> {
    data.par_lines()
        .map(|line| {
            let mut visited = HashSet::with_capacity(INITIAL_CAPACITY);
            visited.insert(0);

            line.split(',').try_fold(0, move |current, jump| {
                let jump = jump.parse::<i32>().map_err(|_| Error::InvalidNumber)?;

                // first, jump backward
                let candidate = current - jump;
                if candidate >= 0 && visited.insert(candidate) {
                    return Ok(candidate);
                }

                // if first failed, jump forward
                let candidate = current + jump;
                visited.insert(candidate);

                Ok(candidate)
            })
        })
        .sum()
}

/// Resolve part 2
///
/// # Errors
///
/// * [`Error::InvalidNumber`] - if a jump is invalid.
pub fn part_2(data: &str) -> Result<i32, Error> {
    data.par_lines()
        .map(|line| {
            let mut visited = HashSet::with_capacity(INITIAL_CAPACITY);
            visited.insert(0);

            line.split(',').try_fold(0, move |current, jump| {
                let jump = jump.parse::<i32>().map_err(|_| Error::InvalidNumber)?;

                // first, jump backward
                let candidate = current - jump;
                if candidate >= 0 && visited.insert(candidate) {
                    return Ok(candidate);
                }

                // if first failed, jump forward
                let mut candidate = current + jump;
                while !visited.insert(candidate) {
                    candidate += 1;
                }

                Ok(candidate)
            })
        })
        .sum()
}

/// Check if the candidate arc cross the existing arcs.
///
/// # Arguments
///
/// * `arcs` - list of exists arcs
/// * `(candidate_start, candidate_end)` - the arc to check.
///
/// # Returns
///
/// `true` if the candidate arc cross any of existing arcs
fn arc_cross(arcs: &[(i32, i32)], (candidate_start, candidate_end): (i32, i32)) -> bool {
    arcs.iter().any(|&(arc_start, arc_end)| {
        (arc_end > candidate_start && arc_start < candidate_end)
            && (((arc_start..arc_end).contains(&candidate_start)
                 && !(arc_start..arc_end).contains(&candidate_end))
                || ((arc_start..arc_end).contains(&candidate_end)
                    && !(arc_start..arc_end).contains(&candidate_start)))
    })
}

/// Resolve part 3
///
/// # Errors
///
/// * [`Error::InvalidNumber`] - if a jump is invalid.
#[expect(clippy::missing_panics_doc, reason = "Cannot panic")]
pub fn part_3(data: &str) -> Result<i32, Error> {
    data.par_lines()
        .map(|line| {
            // Set of visited edges
            let mut visited = HashSet::with_capacity(INITIAL_CAPACITY);
            visited.insert(0);

            // List of existing arcs. In index 0 the *lower* arcs. In
            // index 1 the *upper* arcs
            let mut arcs = [
                Vec::with_capacity(INITIAL_CAPACITY),
                Vec::with_capacity(INITIAL_CAPACITY),
            ];

            // Current arc type:
            // 0 - *lower*/*down* arc
            // 1 - *upper* arc
            let mut arc_type = [0, 1].iter().cycle().peekable();

            // Max edge index on the right
            let mut max_edge = 0;
            line.split(',').try_fold(0, move |current, jump| {
                let jump = jump.parse::<i32>().map_err(|_| Error::InvalidNumber)?;

                // current arc type as index (0, 1)
                // cannot panic: cyclic iterator
                let current_arc_type = **arc_type.peek().unwrap();

                // first, jump backward
                let candidate = current - jump;

                // cannot use `visited.insert` (as last check) because
                // `arc_cross` is O(n), performance degrades
                if candidate >= 0
                    && !visited.contains(&candidate)
                    && !arc_cross(&arcs[current_arc_type], (candidate, current))
                {
                    visited.insert(candidate);

                    arcs[current_arc_type].push((candidate, current));
                    max_edge = max_edge.max(current);

                    arc_type.next();

                    return Ok(candidate);
                }

                // if first failed, jump forward
                let mut candidate = current + jump;
                loop {
                    // cannot use `visited.insert` (as last check)
                    // because `arc_cross` is O(n), performance degrades
                    if !visited.contains(&candidate)
                        && !arc_cross(&arcs[current_arc_type], (current, candidate))
                    {
                        visited.insert(candidate);

                        arcs[current_arc_type].push((current, candidate));
                        max_edge = max_edge.max(candidate);

                        arc_type.next();

                        return Ok(candidate);
                    }

                    candidate += 1;
                    if candidate > max_edge + 1 {
                        return Ok(current);
                    }
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let data = r"1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30";
        assert_eq!(part_1(data).unwrap(), 66);
    }

    #[test]
    fn test_part_1_2() {
        let data = r"1,1,1,1,1
5,1,2,3,4,5,1,2,3,4
2,1,1,2,1,1,2,1,1,2,1,1
5,1,2,1,2,7,1,2,1,2,7,1,2,1,2";
        assert_eq!(part_1(data).unwrap(), 34);
    }

    #[test]
    fn test_part_2() {
        let data = r"1,1,1,1,1
5,1,2,3,4,5,1,2,3,4
2,1,1,2,1,1,2,1,1,2,1,1
5,1,2,1,2,7,1,2,1,2,7,1,2,1,2";
        assert_eq!(part_2(data).unwrap(), 43);
    }

    #[test]
    fn test_part_3_1() {
        let data = r"1,1,1,1,1
5,1,2,3,4,5,1,2,3,4
2,1,1,2,1,1,2,1,1,2,1,1
5,1,2,1,2,7,1,2,1,2,7,1,2,1,2";
        assert_eq!(part_3(data).unwrap(), 27);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"5,3,1,1
5,3,1,1,5,1,1,3,4,8,1,1
5,3,1,1,5,1,1,3,4,8,2,1
10,9,9,8,8,7,7,6,6,5,5,4,4,3,3,2,2,1";
        assert_eq!(part_3(data).unwrap(), 35);
    }

    #[test]
    fn test_part_3_2_1() {
        let data = r"5,3,1,1";
        assert_eq!(part_3(data).unwrap(), 6);
    }

    #[test]
    fn test_part_3_2_2() {
        let data = r"5,3,1,1,5,1,1,3,4,8,1,1";
        assert_eq!(part_3(data).unwrap(), 17);
    }
}
