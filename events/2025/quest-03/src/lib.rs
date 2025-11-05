use std::collections::HashMap;

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let mut crates = data
        .split(',')
        .map(|c| c.parse::<u64>().expect("invalid number"))
        .collect::<Vec<_>>();

    crates.sort_unstable();
    crates.dedup();

    crates.iter().sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let mut crates = data
        .split(',')
        .map(|c| c.parse::<u64>().expect("invalid number"))
        .collect::<Vec<_>>();

    crates.sort_unstable();
    crates.dedup();

    crates.iter().take(20).sum()
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let mut map = HashMap::with_capacity(1024);
    for c in data
        .split(',')
        .map(|c| c.parse::<u64>().expect("invalid number"))
    {
        *map.entry(c).or_default() += 1;
    }

    *map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("10,5,1,10,3,8,5,2,2"), 29);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77"
            ),
            781
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77"
            ),
            3
        );
    }
}
