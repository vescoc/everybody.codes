trait Solve {
    fn table_size(notes: impl Iterator<Item = usize>) -> usize {
        notes.max().unwrap() + 1
    }

    fn beetles(idx: usize, table: &[usize]) -> usize {
        table[idx]
    }

    fn solve(data: &[u8], factors: &[usize]) -> usize {
        let notes = data
            .split(|&c| c == b'\n')
            .map(|line| {
                line.iter().fold(0, |acc, &digit| {
                    acc * 10 + usize::from(digit) - usize::from(b'0')
                })
            })
            .collect::<Vec<_>>();

        let size = Self::table_size(notes.iter().copied());

        let mut table = vec![usize::MAX; size];

        for &dots in factors {
            table[dots] = 1;
        }

        for i in 2..size {
            for &dots in factors {
                if i > dots {
                    table[i] = table[i].min(table[i - dots] + 1);
                }
            }
        }

        notes.iter().map(|&i| Self::beetles(i, &table)).sum()
    }
}

struct Part1;
impl Solve for Part1 {}

use Part1 as Part2;

struct Part3;
impl Solve for Part3 {
    fn table_size(notes: impl Iterator<Item = usize>) -> usize {
        (notes.max().unwrap() + 1) / 2 + 100
    }

    fn beetles(idx: usize, table: &[usize]) -> usize {
        let mut min = usize::MAX;

        let mut first = idx / 2;
        let mut second = idx - first;

        while second - first <= 100 {
            min = min.min(table[first] + table[second]);

            first -= 1;
            second += 1;
        }

        min
    }
}

#[must_use]
pub fn part_1(data: &[u8]) -> usize {
    Part1::solve(data, &[1, 3, 5, 10])
}

#[must_use]
pub fn part_2(data: &[u8]) -> usize {
    Part2::solve(data, &[1, 3, 5, 10, 15, 16, 20, 24, 25, 30])
}

#[must_use]
pub fn part_3(data: &[u8]) -> usize {
    Part3::solve(
        data,
        &[
            1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            10,
            part_1(
                br"2
4
7
16"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            10,
            part_2(
                br"33
41
55
99"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            10449,
            part_3(
                br"156488
352486
546212"
            )
        );
    }
}
