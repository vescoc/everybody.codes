fn creature_to_potions(creature: u8) -> Option<u64> {
    match creature {
        b'A' => Some(0),
        b'B' => Some(1),
        b'C' => Some(3),
        b'D' => Some(5),
        b'x' => None,
        _ => unreachable!(),
    }
}

#[must_use]
pub fn part_1(data: &[u8]) -> u64 {
    data.iter().copied().filter_map(creature_to_potions).sum()
}

#[must_use]
pub fn part_2(data: &[u8]) -> u64 {
    data
        .chunks_exact(2)
        .map(|pair| {
            let (sum, count) = pair
                .iter()
                .copied()
                .map(creature_to_potions)
                .fold((0, 0), |(sum, count), v| v.map_or((sum, count), |v| (sum + v, count + 1)));
            sum + if count == 2 { 2 } else { 0 }
        })
        .sum()
}

#[must_use]
pub fn part_3(data: &[u8]) -> u64 {
    data
        .chunks_exact(3)
        .map(|pair| {
            let (sum, count) = pair
                .iter()
                .copied()
                .map(creature_to_potions)
                .fold((0, 0), |(sum, count), v| v.map_or((sum, count), |v| (sum + v, count + 1)));
            sum + match count {
                2 => 2,
                3 => 6,
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(5, part_1(b"ABBAC"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(28, part_2(b"AxBCDDCAxD"));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(30, part_3(b"xBxAAABCDxCC"));
    }
}
