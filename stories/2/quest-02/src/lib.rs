use std::collections::VecDeque;

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let mut balloons = data.chars();

    for (count, fluffbolt) in "RGB".chars().cycle().enumerate() {
        let mut hit = false;
        loop {
            match balloons.next() {
                None => return count + usize::from(hit),
                Some(balloon) if balloon == fluffbolt => {
                    // passthrough
                    hit = true;
                }
                Some(_) => {
                    break;
                }
            }
        }
    }

    unreachable!()
}

/// # Panics
#[allow(clippy::maybe_infinite_iter)]
#[must_use]
pub fn part_2<const REPEAT: usize>(data: &str) -> usize {
    let mut left = std::iter::repeat_n(data, REPEAT)
        .flat_map(|data| data.chars())
        .take(data.len() * REPEAT / 2)
        .collect::<VecDeque<_>>();
    let mut right = std::iter::repeat_n(data, REPEAT)
        .flat_map(|data| data.chars())
        .skip(data.len() * REPEAT / 2)
        .collect::<VecDeque<_>>();

    "RGB"
        .chars()
        .cycle()
        .take_while(move |&fluffbolt| {
            if left.is_empty() && right.is_empty() {
                false
            } else {
                if left.len() > right.len() {
                    left.pop_front();
                } else if right.len() > left.len() {
                    left.push_back(right.pop_front().unwrap());
                    left.pop_front();
                } else if let Some(balloon) = left.pop_front()
                    && balloon == fluffbolt
                {
                    right.pop_front();
                }

                true
            }
        })
        .count()
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    part_2::<100_000>(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"GRBGGGBBBRRRRRRRR";

        assert_eq!(part_1(data), 7);
    }

    #[test]
    fn test_part_2_1() {
        let data = r"GGBR";

        assert_eq!(part_2::<5>(data), 14);
    }

    #[test]
    fn test_part_2_2() {
        let data = r"BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";

        assert_eq!(part_2::<10>(data), 304);
    }

    #[test]
    fn test_part_2_3() {
        let data = r"BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";

        assert_eq!(part_2::<50>(data), 1464);
    }

    #[test]
    fn test_part_2_4() {
        let data = r"BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";

        assert_eq!(part_2::<100>(data), 2955);
    }
}
