use rayon::prelude::*;

use itertools::Itertools;

fn token_fall(pattern: &[&str], sequence: &str, toss_slot: usize) -> u64 {
    let mut behaviour = sequence.chars();
    let mut current_row = 0;
    let mut current_column = toss_slot * 2;

    while let Some(row) = pattern.get(current_row) {
        let row = row.as_bytes();
        if row[current_column] == b'*' {
            match behaviour.next() {
                Some('L') => {
                    if current_column > 0 {
                        current_column -= 1;
                    } else {
                        current_column += 1;
                    }
                    current_row += 1;
                }
                Some('R') => {
                    if current_column + 1 < row.len() {
                        current_column += 1;
                    } else {
                        current_column -= 1;
                    }
                    current_row += 1;
                }
                _ => unreachable!(),
            }
        } else {
            current_row += 1;
        }
    }

    let final_slot = current_column / 2;

    ((final_slot + 1) * 2).saturating_sub(toss_slot + 1) as u64
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let (pattern, tokens) = data.split_once("\n\n").unwrap();
    let pattern = pattern.lines().collect::<Vec<_>>();

    tokens
        .lines()
        .enumerate()
        .map(|(toss_slot, sequence)| token_fall(&pattern, sequence, toss_slot))
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let (pattern, tokens) = data.split_once("\n\n").unwrap();
    let pattern = pattern.lines().collect::<Vec<_>>();

    let slots = pattern[0].len() / 2;

    tokens
        .lines()
        .map(|sequence| {
            (0..=slots)
                .map(|toss_slot| token_fall(&pattern, sequence, toss_slot))
                .max()
                .unwrap()
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> String {
    let (pattern, tokens) = data.split_once("\n\n").unwrap();
    let pattern = pattern.lines().collect::<Vec<_>>();

    let slots = pattern[0].len() / 2;

    let games = tokens
        .par_lines()
        .map(|sequence| {
            (0..=slots)
                .map(|toss_slot| token_fall(&pattern, sequence, toss_slot))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (min, max) = (0..=slots)
        .permutations(6)
        .map(|slots| {
            slots
                .iter()
                .enumerate()
                .map(|(i, slot)| games[i][*slot])
                .sum::<u64>()
        })
        .minmax()
        .into_option()
        .unwrap();

    format!("{min} {max}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL
LRLLLLLLRLLL
RRLLLRLLRLRR
RLLLLLRLLLRL";

        assert_eq!(part_1(data), 26);
    }

    #[test]
    fn test_part_2() {
        let data = r"*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL
LRLLRRLRLLLLLLLRLRRL
LRLLRRLLLRRRRRLRRLRR
LRLLRRLRLLRLRRLLLRLL
RLLRRRRLRLRLRLRLLRRL";

        assert_eq!(part_2(data), 115);
    }

    #[test]
    fn test_part_3_1() {
        let data = r"*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL";

        assert_eq!(&part_3(data), "13 43");
    }

    #[test]
    fn test_part_3_2() {
        let data = r"*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL";

        assert_eq!(&part_3(data), "25 66");
    }

    #[test]
    fn test_part_3_3() {
        let data = r"*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*.*.*.........*.*.*.*.....*.*.*
.*.*...*.*.*.*.*.*.*.*.*.*.*...*.*.*.*.
*.*.*.*...*.*.*.*.*.....*.*.*.*...*.*..
.*...*.*...*.*.*.*.*.*.*.....*.*.*.*.*.
*.*.*.*.*.....*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*...*.*.*.*.....*.*.*.*...*.
*.*...*.*.*.*.*.*.*.*...*.*.*...*.*.*.*
.*...*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.
*.*.*.*.*.*...*.....*.*...*...*.*.*.*.*
.*...*.*.*.*.*...*.*.*.*.*...*.*...*.*.
*.*.*.*.*...*.*.*.*.*.*.*.*...*.*.*.*.*
.*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.*.*.
....*.*.*.*...*.*.*.*.*.*.*...*.*.*...*
.*.*.*...*.*.*.*.*...*.*.*.*.*.*.*.*...
*.*.*.*.*.*.*.....*...*...*.*.*.*.*.*.*
.*.*...*.....*.*.*.*.*.*.*...*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.

RRRRLLRRLLLLLLLRLLRL
RRRRRRRLRRLRRLRRRLRR
RRRLLRRRRRLRRRRRLRRR
LLLLRRLLRRLLLLLRRLLL
LRRRRLRRLRLLRLLRRLRR
RRRRRRRRLRRRRLLRRRLR";

        assert_eq!(&part_3(data), "39 122");
    }
}
