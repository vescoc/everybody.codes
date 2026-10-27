const SQUARES_LEN: usize = 4096;

static FILLED_SQUARES: &[u64] = &const {
    let mut squares = [0; SQUARES_LEN];
    let mut side = 0;
    let mut index = 0;
    while index < SQUARES_LEN {
        squares[index] = side * side;

        index += 1;
        side += 1;
    }

    squares
};

static FRAMED_SQUARES: &[u64] = &const {
    let mut squares = [0; SQUARES_LEN];
    let mut side = 3;
    let mut index = 0;
    while index < SQUARES_LEN {
        squares[index] = side * side - (side - 2) * (side - 2);

        index += 1;
        side += 1;
    }

    squares
};

static FILLED_WITH_HOLE_SQUARES: &[u64] = &const {
    let mut squares = [0; SQUARES_LEN];
    let mut side = 3;
    let mut index = 0;
    while index < SQUARES_LEN {
        squares[index] = side * side - 1;

        index += 1;
        side += 2;
    }

    squares
};

/// # Panics
#[must_use]
fn solve(targets: &[u64], data: &str) -> u64 {
    data.split(',')
        .map(|cones| {
            let cones = cones.parse::<u64>().expect("invalid cones");
            let target_cones = if let Err(index) = targets.binary_search(&cones) {
                targets[index]
            } else {
                cones
            };
            target_cones - cones
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(FILLED_SQUARES, data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve(FRAMED_SQUARES, data)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    solve(FILLED_WITH_HOLE_SQUARES, data)
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
