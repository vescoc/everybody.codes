#![allow(clippy::cast_possible_truncation)]

use rayon::prelude::*;

struct Part(u64);

fn join(mut result: u64, mut cur: u64, mut mul: u64) -> (u64, u64) {
    if cur == 0 {
        (result, mul * 10)
    } else {
        while cur > 0 {
            let d = cur % 10;
            cur /= 10;
            result += d * mul;
            mul *= 10;
        }
        (result, mul)
    }
}

fn mod_pow(mut n: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    n %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * n) % modulus;
        }
        exp >>= 1;
        n = (n * n) % modulus;
    }
    result
}

fn eni_1(n: u64, exp: u64, modulus: u64) -> u64 {
    let mut cur = 1;
    let mut result = 0;
    let mut mul = 1;
    for _ in 0..exp {
        cur = (cur * n) % modulus;
        (result, mul) = join(result, cur, mul);
    }
    result
}

fn eni_2(n: u64, exp: u64, modulus: u64) -> u64 {
    if exp < 5 {
        eni_1(n, exp, modulus)
    } else {
        let mut cur = mod_pow(n, exp - 5, modulus);
        let mut result = 0;
        let mut mul = 1;
        for _ in 0..5 {
            cur = (cur * n) % modulus;
            (result, mul) = join(result, cur, mul);
        }
        result
    }
}

fn eni_3(n: u64, exp: u64, modulus: u64) -> u64 {
    let mut sum = 0;
    let mut seen_at = vec![None; modulus as usize];
    let mut partial_sums = vec![sum];
    let mut cur = 1;
    while partial_sums.len() < exp as usize {
        cur = (cur * n) % modulus;
        if let Some(cycle_start) = seen_at[cur as usize] {
            let cycle_length = partial_sums.len() - cycle_start;
            let cycle_sum = sum + cur - partial_sums[cycle_start];
            let remaining = exp - partial_sums.len() as u64 + 1;
            let cycles = remaining / cycle_length as u64;

            sum += cycle_sum * cycles
                + partial_sums[cycle_start + remaining as usize % cycle_length - 1]
                - partial_sums[cycle_start - 1];

            break;
        }

        sum += cur;
        seen_at[cur as usize] = Some(partial_sums.len());
        partial_sums.push(sum);
    }
    sum
}

impl<'a> FromIterator<&'a str> for Part {
    fn from_iter<II: IntoIterator<Item = &'a str>>(ii: II) -> Self {
        Self(
            ii.into_iter()
                .nth(1)
                .expect("Invalid input")
                .parse()
                .expect("Invalid number"),
        )
    }
}

fn solve(data: &str, eni: impl Fn(u64, u64, u64) -> u64) -> u64 {
    data.lines()
        .map(|line| {
            line.split(' ')
                .map(|part| {
                    let Part(n) = part.split('=').collect::<Part>();
                    n
                })
                .collect::<Vec<_>>()
        })
        .map(|input| {
            let modulus = input[6];
            eni(input[0], input[3], modulus)
                + eni(input[1], input[4], modulus)
                + eni(input[2], input[5], modulus)
        })
        .max()
        .unwrap()
}

fn solve_rayon(data: &str, eni: impl Fn(u64, u64, u64) -> u64 + Sync) -> u64 {
    data.par_lines()
        .map(|line| {
            line.split(' ')
                .map(|part| {
                    let Part(n) = part.split('=').collect::<Part>();
                    n
                })
                .collect::<Vec<_>>()
        })
        .map(|input| {
            let modulus = input[6];
            eni(input[0], input[3], modulus)
                + eni(input[1], input[4], modulus)
                + eni(input[2], input[5], modulus)
        })
        .max()
        .unwrap()
}

#[must_use]
pub fn part_1(data: &str) -> u64 {
    solve(data, eni_1)
}

#[must_use]
pub fn part_2(data: &str) -> u64 {
    solve_rayon(data, eni_2)
}

#[must_use]
pub fn part_3(data: &str) -> u64 {
    solve_rayon(data, eni_3)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal)]

    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"A=4 B=4 C=6 X=3 Y=4 Z=5 M=11
A=8 B=4 C=7 X=8 Y=4 Z=6 M=12
A=2 B=8 C=6 X=2 Y=4 Z=5 M=13
A=5 B=9 C=6 X=8 Y=6 Z=8 M=14
A=5 B=9 C=7 X=6 Y=6 Z=8 M=15
A=8 B=8 C=8 X=6 Y=9 Z=6 M=16";

        assert_eq!(part_1(data), 11611972920);
    }

    #[test]
    fn test_eni_2() {
        assert_eq!(eni_2(2, 7, 5), 34213);
        assert_eq!(eni_2(3, 8, 16), 111931);
    }

    #[test]
    fn test_eni_3() {
        assert_eq!(eni_3(2, 7, 5), 19);
        assert_eq!(eni_3(3, 8, 16), 48);
    }

    #[test]
    fn test_part_2_1() {
        let data = r"A=4 B=4 C=6 X=3 Y=14 Z=15 M=11
A=8 B=4 C=7 X=8 Y=14 Z=16 M=12
A=2 B=8 C=6 X=2 Y=14 Z=15 M=13
A=5 B=9 C=6 X=8 Y=16 Z=18 M=14
A=5 B=9 C=7 X=6 Y=16 Z=18 M=15
A=8 B=8 C=8 X=6 Y=19 Z=16 M=16";

        assert_eq!(part_2(data), 11051340);
    }

    #[test]
    fn test_part_2_2() {
        let data = r"A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";

        assert_eq!(part_2(data), 1507702060886);
    }

    #[test]
    fn test_part_3_1() {
        let data = r"A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110
A=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120
A=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130
A=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140
A=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150
A=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160";

        assert_eq!(part_3(data), 3279640);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";

        assert_eq!(part_3(data), 7276515438396);
    }
}
