/// # Panics
#[must_use]
fn solve(data: &str, f: impl Fn(u64) -> u64) -> u64 {
    data.split(',')
        .map(|cones| {
            let cones = cones.parse::<u64>().expect("invalid cones");
            let target_cones = f(cones);
            target_cones - cones
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(data, |cones| {
        let side = (cones - 1).isqrt() + 1;
        side * side
    })
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve(data, |cones| {
        let side = cones.div_ceil(4).max(2);
        side * 4
    })
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    solve(data, |cones| {
        let side = (cones.isqrt() + 1) | 1;
        side * side - 1
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(r"3,11,12,16,24,25"), 11,);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(r"3,11,12,16,24,25"), 9,);
    }

    #[test]
    fn test_part_3() {
        assert_eq!(part_3(r"3,11,12,16,24,25"), 61,);
    }
}
