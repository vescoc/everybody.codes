type Segment = (u64, u64, usize, usize);

#[allow(clippy::cast_possible_truncation)]
fn make_segments(flock: &[u64]) -> Vec<Segment> {
    let mut segments = vec![];

    let mut start = 0;
    while start < flock.len() {
        let end = start + 1;
        let segment_sum = flock[start];

        let segment_len = end - start;
        let segment_mean = segment_sum / segment_len as u64;
        let segment_right_count = (segment_sum % segment_len as u64) as usize;
        if let Some((
            last_segment_sum,
            last_segment_mean,
            last_segment_right_count,
            last_segment_len,
        )) = segments.last_mut()
            && segment_mean < *last_segment_mean
        {
            // merge
            *last_segment_sum += segment_sum;
            *last_segment_len += segment_len;
            *last_segment_mean = *last_segment_sum / *last_segment_len as u64;
            *last_segment_right_count = (*last_segment_sum % *last_segment_len as u64) as usize;
        } else {
            segments.push((segment_sum, segment_mean, segment_right_count, segment_len));
        }

        start = end;
    }

    segments
}

#[allow(clippy::cast_possible_truncation)]
fn merge_segments(segments: &[Segment]) -> Vec<Segment> {
    let mut result = vec![];

    for segment in segments {
        if let Some((
            last_segment_sum,
            last_segment_mean,
            last_segment_right_count,
            last_segment_len,
        )) = result.last_mut()
            && segment.1 < *last_segment_mean
        {
            // merge
            *last_segment_sum += segment.0;
            *last_segment_len += segment.3;
            *last_segment_mean = *last_segment_sum / *last_segment_len as u64;
            *last_segment_right_count = (*last_segment_sum % *last_segment_len as u64) as usize;
        } else {
            result.push(*segment);
        }
    }

    result
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let mut flock = data
        .lines()
        .map(|line| line.parse::<u64>().expect("invalid ducks' number"))
        .collect::<Vec<_>>();

    let mut round = 0;

    // Phase 1
    while round < 10 {
        let mut moves = false;
        for (i, j) in (0..flock.len() - 1).zip(1..flock.len()) {
            if flock[j] < flock[i] {
                moves = true;
                flock[i] -= 1;
                flock[j] += 1;
            }
        }

        if !moves {
            break;
        }
        round += 1;
    }

    // Phase 2
    while round < 10 {
        let mut moves = false;
        for (j, i) in (0..flock.len() - 1).zip(1..flock.len()) {
            if flock[j] < flock[i] {
                moves = true;
                flock[i] -= 1;
                flock[j] += 1;
            }
        }

        if !moves {
            break;
        }
        round += 1;
    }

    flock
        .into_iter()
        .enumerate()
        .map(|(i, ducks)| (i + 1) as u64 * ducks)
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let flock = data
        .lines()
        .map(|line| line.parse::<u64>().expect("Invalid ducks' number"))
        .collect::<Vec<_>>();

    let target = flock.iter().sum::<u64>() / flock.len() as u64;

    let mut segments = make_segments(&flock);
    while segments
        .iter()
        .take(segments.len() - 1)
        .zip(&segments[1..])
        .any(|((_, mean_a, ..), (_, mean_b, ..))| mean_a > mean_b)
    {
        segments = merge_segments(&segments);
    }

    let phase_1_rounds = segments
        .iter()
        .flat_map(|&(_, mean, count, len)| {
            std::iter::repeat_n(mean, len - count)
                .take(len - count)
                .chain(std::iter::repeat_n(mean + 1, count))
        })
        .zip(flock)
        .fold((u64::MIN, 0u64), |(max, curry), (mean, ducks)| {
            let diff = ducks + curry - mean;
            (max.max(diff), diff)
        })
        .0;

    let phase_2_rounds = segments
        .into_iter()
        .flat_map(|(_, segment_mean, segment_right_count, segment_len)| {
            std::iter::repeat_n(segment_mean, segment_len - segment_right_count)
                .chain(std::iter::repeat_n(segment_mean + 1, segment_right_count))
        })
        .filter_map(|ducks| {
            if ducks < target {
                Some(target - ducks)
            } else {
                None
            }
        })
        .sum::<u64>();

    phase_1_rounds + phase_2_rounds
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let flock = data
        .lines()
        .map(|line| line.parse::<u64>().expect("invalid ducks' number"))
        .collect::<Vec<_>>();

    assert!(
        (0..flock.len() - 1)
            .zip(1..flock.len())
            .all(|(i, j)| flock[i] <= flock[j])
    );

    let target = flock.iter().sum::<u64>() / flock.len() as u64;
    flock
        .into_iter()
        .filter_map(|ducks| {
            if ducks < target {
                Some(target - ducks)
            } else {
                None
            }
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
                r"9
1
1
4
9
6"
            ),
            109,
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            part_2(
                r"9
1
1
4
9
6"
            ),
            11,
        );
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(
            part_2(
                r"805
706
179
48
158
150
232
885
598
524
423"
            ),
            1579,
        );
    }
}
