use itertools::Itertools;
use rayon::prelude::*;

/// # Panics
#[must_use]
pub fn part_1<const NAILS: u8>(data: &str) -> usize {
    data.split(',')
        .map(|nail| nail.parse::<u8>().expect("invalid nail number"))
        .tuple_windows()
        .filter(|(start, end)| start.abs_diff(*end) == NAILS / 2)
        .count()
}

fn overlaps((s1, e1): (u32, u32), (s2, e2): (u32, u32)) -> bool {
    (s1 == s2 && e1 == e2) || (s2 > s1 && s2 < e1 && e2 > e1) || (e2 > s1 && e2 < e1 && s2 < s1)
}

/// # Panics
#[must_use]
pub fn part_2<const NAILS: u32>(data: &str) -> usize {
    let threads = data
        .split(',')
        .map(|nail| nail.parse::<u32>().expect("invalid nail") - 1)
        .tuple_windows()
        .map(|(start, end)| (start.min(end), start.max(end)))
        .collect::<Vec<_>>();

    threads
        .par_iter()
        .enumerate()
        .map(|(i, thread_1)| {
            threads[0..i]
                .iter()
                .filter(|thread_2| overlaps(*thread_1, **thread_2))
                .count()
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_3<const NAILS: u32>(data: &str) -> usize {
    let threads = &data
        .split(',')
        .map(|nail| nail.parse::<u32>().expect("invalid nail number") - 1)
        .tuple_windows()
        .map(|(start, end)| (start.min(end), start.max(end)))
        .collect::<Vec<(_, _)>>();

    (0..NAILS - 1)
        .into_par_iter()
        .filter_map(|start| {
            (start + 1..NAILS)
                .map(move |end| {
                    threads
                        .iter()
                        .filter(|thread| overlaps((start, end), **thread))
                        .count()
                })
                .max()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1::<8>("1,5,2,6,8,4,1,7,3"), 4);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2::<8>("1,5,2,6,8,4,1,7,3,5,7,8,2"), 21);
    }

    #[test]
    fn test_part_3() {
        assert_eq!(part_3::<8>("1,5,2,6,8,4,1,7,3,6"), 7);
    }
}
