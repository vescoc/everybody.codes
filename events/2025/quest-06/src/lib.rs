use rayon::prelude::*;

#[must_use]
pub fn part_1(data: &str) -> usize {
    data.bytes()
        .fold((0, 0), |(mentors, pairs), c| {
            if c == b'a' {
                (mentors, pairs + mentors)
            } else if c == b'A' {
                (mentors + 1, pairs)
            } else {
                (mentors, pairs)
            }
        })
        .1
}

#[must_use]
pub fn part_2(data: &str) -> usize {
    let mut mentors = [0; 3];
    let mut pairs = [0; 3];
    data.bytes()
        .fold((&mut mentors, &mut pairs), |(mentors, pairs), c| {
            if c.is_ascii_lowercase() {
                let index = (c - b'a') as usize;
                pairs[index] += mentors[index];
            } else {
                let index = (c - b'A') as usize;
                mentors[index] += 1;
            }

            (mentors, pairs)
        })
        .1
        .iter()
        .sum()
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
#[must_use]
pub fn part_3<const REPEAT: usize, const DISTANCE: usize>(data: &str) -> usize {
    let data = data.as_bytes();
    let size = data.len() as isize;

    data.par_iter()
        .enumerate()
        .map(|(i, b)| {
            let mut result = 0;
            if b.is_ascii_lowercase() {
                let mentor = b.to_ascii_uppercase();
                for j in -(DISTANCE as isize)..=(DISTANCE as isize) {
                    let offset = i as isize + j;

                    let wrapped = offset.rem_euclid(size) as usize;

                    if data[wrapped] == mentor {
                        let adjust = if offset < 0 {
                            size - 1 - offset
                        } else {
                            offset
                        };
                        result += REPEAT - (adjust / size) as usize;
                    }
                }
            }
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part_3_slow<const REPEAT: usize, const DISTANCE: usize>(data: &str) -> usize {
        let data = data.as_bytes().repeat(REPEAT);
        let len = data.len();
        data.par_iter()
            .enumerate()
            .map(|(i, c)| {
                if c.is_ascii_lowercase() {
                    let mentor = c.to_ascii_uppercase();

                    let lower = i.saturating_sub(DISTANCE);
                    let higher = i.saturating_add(DISTANCE + 1).min(len);

                    bytecount::count(&data[lower..higher], mentor)
                } else {
                    0
                }
            })
            .sum()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("ABabACacBCbca"), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("ABabACacBCbca"), 11);
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(part_3::<1, 10>("AABCBABCABCabcabcABCCBAACBCa"), 34);
    }

    #[test]
    fn test_part_3_1_slow() {
        let data = "AABCBABCABCabcabcABCCBAACBCa";
        assert_eq!(part_3::<1, 10>(data), part_3_slow::<1, 10>(data));
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(part_3::<2, 10>("AABCBABCABCabcabcABCCBAACBCa"), 72);
    }

    #[test]
    fn test_part_3_2_slow() {
        let data = "AABCBABCABCabcabcABCCBAACBCa";
        assert_eq!(part_3::<2, 10>(data), part_3_slow::<2, 10>(data));
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_3_3() {
        assert_eq!(
            part_3::<1000, 1000>("AABCBABCABCabcabcABCCBAACBCa"),
            3442321
        );
    }

    #[test]
    fn test_part_3_3_slow() {
        let data = "AABCBABCABCabcabcABCCBAACBCa";
        assert_eq!(part_3::<1000, 1000>(data), part_3_slow::<1000, 1000>(data));
    }

    #[test]
    fn test_part_3_3_large_slow() {
        let data = "AABCBABCABCabcabcABCCBAACBCa".repeat(1000);
        assert_eq!(
            part_3::<1000, 1000>(&data),
            part_3_slow::<1000, 1000>(&data),
        );
    }

    #[test]
    fn test_part_3_same() {
        let data = include_str!("../data/part_3");
        assert_eq!(part_3::<1000, 1000>(data), part_3_slow::<1000, 1000>(data));
    }
}
