use std::collections::HashMap;
use std::iter::Peekable;

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

pub use part_3_fast as part_3;

/// # Panics
#[must_use]
pub fn part_3_bf(data: &str) -> String {
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

/// # Panics
#[must_use]
pub fn part_3_fast(data: &str) -> String {
    let (pattern, tokens) = data.split_once("\n\n").unwrap();
    let pattern = pattern.lines().collect::<Vec<_>>();

    let slots = pattern[0].len() / 2;

    let games = tokens
        .par_lines()
        .map(|sequence| {
            let mut throws = (0..=slots)
                .map(|toss_slot| token_fall(&pattern, sequence, toss_slot))
                .enumerate()
                .collect::<Vec<_>>();
            throws.sort_unstable_by_key(|(_, v)| *v);
            throws
        })
        .collect::<Vec<_>>();

    let min = min_coins(&games);
    let max = max_coins(&games);

    format!("{min} {max}")
}

fn min_coins(games: &[Vec<(usize, u64)>]) -> u64 {
    calc_coins::<{ u64::MAX }, _, _, _, _>(|a, b| a > b, std::cmp::Ord::min, |ii| ii, games)
}

fn max_coins(games: &[Vec<(usize, u64)>]) -> u64 {
    calc_coins::<{ u64::MIN }, _, _, _, _>(
        |a, b| a < b,
        std::cmp::Ord::max,
        std::iter::Iterator::rev,
        games,
    )
}

fn calc_coins<const INIT: u64, IOUT, CUT, MINMAX, REV>(
    cut: CUT,
    minmax: MINMAX,
    rev: REV,
    games: &[Vec<(usize, u64)>],
) -> u64
where
    IOUT: Iterator<Item = (usize, u64)> + Clone,
    CUT: Fn(u64, u64) -> bool + Copy,
    MINMAX: Fn(u64, u64) -> u64 + Copy,
    REV: Fn(std::vec::IntoIter<(usize, u64)>) -> IOUT,
{
    let mut current = INIT;

    let mut current_coins = 0;
    let mut set = Box::new([0u8; 20]);
    let mut candidate = HashMap::<usize, Vec<_>>::with_capacity(6);
    for (token_id, token_throws) in games.iter().enumerate() {
        let mut token_throws = rev(token_throws.clone().into_iter()).peekable();
        let (slot, coins) = *token_throws.peek().unwrap();
        current_coins += coins;
        candidate
            .entry(slot)
            .or_default()
            .push((token_id, token_throws));
        set[slot] |= 1 << token_id;
    }

    let mut seen = HashMap::new();

    calc_coins_r::<INIT, _, _, _>(
        cut,
        minmax,
        &mut current,
        &mut seen,
        current_coins,
        set,
        &candidate,
    )
}

fn calc_coins_r<const INIT: u64, I, CUT, MINMAX>(
    cut: CUT,
    minmax: MINMAX,
    current: &mut u64,
    seen: &mut HashMap<Box<[u8; 20]>, u64>,
    coins: u64,
    set: Box<[u8; 20]>,
    candidate: &HashMap<usize, Vec<(usize, Peekable<I>)>>,
) -> u64
where
    I: Iterator<Item = (usize, u64)> + Clone,
    CUT: Fn(u64, u64) -> bool + Copy,
    MINMAX: Fn(u64, u64) -> u64 + Copy,
{
    if cut(coins, *current) {
        return coins;
    }

    if seen.contains_key(&set) {
        return seen[&set];
    }

    if candidate.len() == 6 {
        seen.insert(set, coins);
        *current = minmax(*current, coins);
        return coins;
    }

    let mut new_current = INIT;
    for (slot, tokens) in candidate {
        if tokens.len() > 1 {
            let old_slot_coins = tokens
                .iter()
                .filter_map(|(_, token_throws)| {
                    token_throws.clone().peek().map(|(_, coins)| *coins)
                })
                .sum::<u64>();
            for (token_id, token_throws) in tokens {
                let mut token_throws = token_throws.clone();
                let mut new_coins = token_throws.peek().unwrap().1;

                let mut new_candidate = candidate.clone();
                new_candidate.insert(*slot, vec![(*token_id, token_throws)]);

                let mut new_set = set.clone();
                new_set[*slot] = 1 << token_id;

                for (other_token_id, other_token_throws) in tokens {
                    if other_token_id != token_id {
                        let mut other_token_throws = other_token_throws.clone();
                        let _ = other_token_throws.next();
                        let (new_token_slot, new_token_coins) = *other_token_throws.peek().unwrap();
                        new_candidate
                            .entry(new_token_slot)
                            .or_default()
                            .push((*other_token_id, other_token_throws));
                        new_coins += new_token_coins;
                        new_set[new_token_slot] |= 1 << other_token_id;
                    }
                }

                new_current = minmax(
                    new_current,
                    calc_coins_r::<INIT, I, _, _>(
                        cut,
                        minmax,
                        current,
                        seen,
                        coins + new_coins - old_slot_coins,
                        new_set,
                        &new_candidate,
                    ),
                );
            }
        }
    }

    seen.insert(set, new_current);

    new_current
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
    fn test_part_3_1_bf() {
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

        assert_eq!(&part_3_bf(data), "13 43");
    }

    #[test]
    fn test_part_3_2_bf() {
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
    fn test_part_3_3_bf() {
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

    #[test]
    fn test_part_3_1_fast() {
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

        assert_eq!(&part_3_fast(data), "13 43");
    }

    #[test]
    fn test_part_3_2_fast() {
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

        assert_eq!(&part_3_fast(data), "25 66");
    }

    #[test]
    fn test_part_3_3_fast() {
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

        assert_eq!(&part_3_fast(data), "39 122");
    }
}
