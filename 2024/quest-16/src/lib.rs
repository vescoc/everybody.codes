use std::collections::HashMap;

use rayon::prelude::*;

use num::Integer;

fn parse(data: &[u8]) -> (Vec<usize>, Vec<Vec<&[u8]>>) {
    let mut parts = data.split(|&c| c == b'\n');

    let positions = parts
        .next()
        .unwrap()
        .split(|&c| c == b',')
        .map(|value| {
            value
                .iter()
                .fold(0, |acc, &digit| acc * 10 + usize::from(digit - b'0'))
        })
        .collect::<Vec<_>>();

    let mut sequences: Vec<Vec<&[u8]>> = vec![Vec::new(); positions.len()];
    for line in parts.skip(1) {
        for (i, arr) in line.chunks(4).enumerate() {
            if arr[0] != b' ' {
                sequences[i].push(&arr[0..3]);
            }
        }
    }

    (positions, sequences)
}

fn solve_1(data: &[u8], count: usize) -> String {
    let (positions, sequences) = parse(data);

    let mut result = String::new();
    for (figure, position) in sequences.iter().zip(positions.iter()) {
        result = format!(
            "{result}{} ",
            std::str::from_utf8(figure[(position * count) % figure.len()]).unwrap()
        );
    }

    result.trim().to_string()
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> String {
    solve_1(data, 100)
}

fn score_eyes(
    positions: &[usize],
    sequences: &[Vec<&[u8]>],
    n: usize,
    push: usize,
    pull: usize,
) -> usize {
    let mut result = HashMap::<u8, usize>::with_capacity(128);
    for (figure, position) in sequences.iter().zip(positions.iter()) {
        let size = figure.len();
        let figure = figure[(position * n + pull + (size - push % size)) % size];
        *result.entry(figure[0]).or_default() += 1;
        *result.entry(figure[2]).or_default() += 1;
    }

    result
        .values()
        .filter_map(|&count| count.checked_sub(2))
        .sum()
}

fn solve_2(data: &[u8], count: usize) -> usize {
    let (positions, sequences) = parse(data);

    let period = sequences
        .iter()
        .zip(positions.iter())
        .map(|(figure, position)| {
            assert!(figure.len().gcd(position) == 1);
            figure.len()
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap();

    let n = count / period;

    let b = || {
        (1..=count % period)
            .map(|n| score_eyes(&positions, &sequences, n, 0, 0))
            .sum::<usize>()
    };

    let (a, b) = if n > 0 {
        std::thread::scope(|s| {
            let a = s.spawn(|| {
                (count % period + 1..=period)
                    .map(|n| score_eyes(&positions, &sequences, n, 0, 0))
                    .sum::<usize>()
            });
            let b = b();
            (a.join().unwrap(), b)
        })
    } else {
        (0, b())
    };

    (a + b) * n + b
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_2(data: &[u8]) -> usize {
    const LOOPS: usize = 202420242024;

    solve_2(data, LOOPS)
}

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> String {
    const STEPS: usize = 256;

    let (positions, sequences) = parse(data);

    let middle = (STEPS * 2).div_ceil(2) + 1;

    let mut current_max = [usize::MIN; STEPS * 2 + 3];
    let mut current_min = [usize::MAX; STEPS * 2 + 3];

    current_max[middle] = 0;
    current_min[middle] = 0;

    for i in 1..=STEPS {
        let (mut next_max, mut next_min) = (current_max, current_min);
        (middle - i..=middle + i)
            .zip(
                next_max[middle - i..=middle + i]
                    .iter_mut()
                    .zip(next_min[middle - i..=middle + i].iter_mut()),
            )
            .par_bridge()
            .for_each(|(j, (next_max, next_min))| {
                let push = middle.saturating_sub(j);
                let pull = j.saturating_sub(middle);

                let score = score_eyes(&positions, &sequences, i, pull, push);

                *next_max = score
                    + current_max[j - 1]
                        .max(current_max[j])
                        .max(current_max[j + 1]);
                *next_min = score
                    + current_min[j - 1]
                        .min(current_min[j])
                        .min(current_min[j + 1]);
            });

        (current_max, current_min) = (next_max, next_min);
    }

    format!(
        "{} {}",
        current_max.iter().max().unwrap(),
        current_min.iter().min().unwrap()
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal)]
    
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            ">.- -.- ^,-",
            &part_1(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>"
            )
        );
    }

    #[test]
    fn test_part_1_1() {
        assert_eq!(
            ">.- -.- ^_^",
            &solve_1(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                1
            )
        );
    }

    #[test]
    fn test_part_1_21() {
        assert_eq!(
            "^_^ -.- ^_^",
            &solve_1(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                21
            )
        );
    }

    #[test]
    fn test_part_1_33() {
        assert_eq!(
            "^_^ ^_^ ^_^",
            &solve_1(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                33
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            280014668134,
            part_2(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>"
            )
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            1,
            solve_2(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                1
            )
        );
    }

    #[test]
    fn test_part_2_10() {
        assert_eq!(
            15,
            solve_2(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                10
            )
        );
    }

    #[test]
    fn test_part_2_100() {
        assert_eq!(
            138,
            solve_2(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
                100
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "627 128",
            part_3(
                br"1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>"
            )
        );
    }
}
