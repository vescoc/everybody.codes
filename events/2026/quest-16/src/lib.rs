use std::collections::VecDeque;

fn skip_front(target: u8, shades: &mut VecDeque<u8>, new_shades: &mut VecDeque<u8>) {
    while shades.pop_front_if(|value| *value == target).is_some() {}
    while let Some(value) = shades.pop_back_if(|value| *value != target) {
        new_shades.push_front(value);
    }
}

fn skip_back(target: u8, shades: &mut VecDeque<u8>, new_shades: &mut VecDeque<u8>) {
    while shades.pop_back_if(|value| *value == target).is_some() {}
    while let Some(value) = shades.pop_front_if(|value| *value != target) {
        new_shades.push_front(value);
    }
}

fn arrange(data: &str, repeat: usize) -> (u64, u64) {
    let mut shades = std::iter::repeat_n(data.as_bytes(), repeat)
        .flatten()
        .copied()
        .collect::<VecDeque<_>>();

    let (mut swaps, mut flips) = (0, 0);
    while let Some(target) = shades.front().copied() {
        let mut new_shades = VecDeque::with_capacity(shades.len());

        skip_front(target, &mut shades, &mut new_shades);

        let mut reverse = true;
        while !shades.is_empty() {
            swaps += shades.len() as u64;
            flips += 1;

            if reverse {
                skip_back(target, &mut shades, &mut new_shades);
            } else {
                skip_front(target, &mut shades, &mut new_shades);
            }

            reverse = !reverse;
        }

        shades = new_shades;
    }

    (swaps, flips)
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    arrange(data, 1).1
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    arrange(data, 10).0
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    arrange(data, 100).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        assert_eq!(part_1(r"RRGGBBGRRGGBBG"), 3,);
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(
            part_1(r"BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGGBBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG"),
            26,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(r"RRYGYBYG"), 1053,);
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3() {
        assert_eq!(part_3(r"RRYGGYBBYG"), 135154,);
    }
}
