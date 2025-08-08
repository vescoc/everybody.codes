fn parse_coordinate(value: &str) -> i64 {
    value
        .split_once('=')
        .expect("Invalid coordinate")
        .1
        .parse()
        .expect("Invalid number")
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> i64 {
    data.lines()
        .map(|line| {
            let (x, y) = line.split_once(' ').expect("Invalid line");
            (parse_coordinate(x), parse_coordinate(y))
        })
        .map(|(x, y)| {
            let m = x + y - 1;
            (
                (x - 1 + 100).rem_euclid(m) + 1,
                (y - 1 - 100).rem_euclid(m) + 1,
            )
        })
        .map(|(x, y)| x + y * 100)
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> i64 {
    data.lines()
        .map(|line| {
            let (x, y) = line.split_once(' ').expect("Invalid line");
            (parse_coordinate(x), parse_coordinate(y))
        })
        .map(|(x, y)| (y - 1, x + y - 1))
        .reduce(|(mut x1, m1), (x2, m2)| {
            while x1 % m2 != x2 {
                x1 += m1;
            }
            (x1, num::integer::lcm(m1, m2))
        })
        .unwrap()
        .0
}

#[cfg(not(feature = "rayon"))]
pub use part_2 as part_3;

/// # Panics
#[must_use]
#[cfg(feature = "rayon")]
pub fn part_3(data: &str) -> i64 {
    use rayon::prelude::*;

    data.par_lines()
        .map(|line| {
            let (x, y) = line.split_once(' ').expect("Invalid line");
            (parse_coordinate(x), parse_coordinate(y))
        })
        .map(|(x, y)| (y - 1, x + y - 1))
        .reduce(
            || (0, 1),
            |(mut x1, m1), (x2, m2)| {
                while x1 % m2 != x2 {
                    x1 += m1;
                }
                (x1, num::integer::lcm(m1, m2))
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4";

        assert_eq!(part_1(data), 1310);
    }

    #[test]
    fn test_part_2_1() {
        let data = r"x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3";

        assert_eq!(part_2(data), 14);
    }

    #[test]
    fn test_part_2_2() {
        let data = r"x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3";

        assert_eq!(part_2(data), 13659);
    }

    #[test]
    fn test_part_3_1() {
        let data = r"x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3";

        assert_eq!(part_3(data), 14);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3";

        assert_eq!(part_3(data), 13659);
    }
}
