use std::cmp::Reverse;
use std::ops;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum Color {
    K = 0b111,
    C = 0b001,
    M = 0b010,
    Y = 0b100,
    R = 0b110,
    G = 0b101,
    B = 0b011,
}

impl TryFrom<u8> for Color {
    type Error = &'static str;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            b'K' => Ok(Color::K),
            b'C' => Ok(Color::C),
            b'M' => Ok(Color::M),
            b'Y' => Ok(Color::Y),
            b'R' => Ok(Color::R),
            b'G' => Ok(Color::G),
            b'B' => Ok(Color::B),
            _ => Err("invalid color"),
        }
    }
}

impl From<usize> for Color {
    fn from(value: usize) -> Self {
        match value {
            0b111 => Color::K,
            0b001 => Color::C,
            0b010 => Color::M,
            0b100 => Color::Y,
            0b110 => Color::R,
            0b101 => Color::G,
            0b011 => Color::B,
            _ => panic!("invalid value"),
        }
    }
}

impl ops::BitOr for Color {
    type Output = Color;

    fn bitor(self, other: Color) -> Self::Output {
        Color::from(self as usize | other as usize)
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let (first_grid, second_grid) = data.split_once("\n\n").expect("invalid input data");

    let mut colors = [0; const { Color::K as usize + 1 }];
    for (c1, c2) in first_grid
        .as_bytes()
        .iter()
        .filter(|c| **c != b'\n')
        .zip(second_grid.as_bytes().iter().filter(|c| **c != b'\n'))
    {
        colors[(Color::try_from(*c1).expect("invalid color")
            | Color::try_from(*c2).expect("invalid color")) as usize] += 1;
    }

    colors.sort_unstable_by_key(|k| Reverse(*k));

    colors.iter().take(3).product()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    fn find_max(
        perm: &mut [Color],
        candidate: &mut Vec<Color>,
        first_grid: &[u8],
        second_grid: &[u8],
    ) -> u64 {
        let len = perm.len();
        if len > 0 {
            (0..len)
                .map(|index| {
                    candidate.push(perm[index]);
                    perm[index..].rotate_left(1);

                    let result = find_max(&mut perm[..len - 1], candidate, first_grid, second_grid);

                    perm[index..].rotate_right(1);
                    candidate.pop();

                    result
                })
                .max()
                .unwrap()
        } else {
            let mut colors = [0; const { Color::K as usize + 1 }];
            for (c1, c2) in first_grid
                .iter()
                .filter(|c| **c != b'\n')
                .zip(second_grid.iter().filter(|c| **c != b'\n'))
            {
                colors[(Color::try_from(*c1).expect("invalid color")
                    | candidate[(*c2 - b'0' - 1) as usize]) as usize] += 1;
            }

            colors.sort_unstable_by_key(|k| Reverse(*k));

            colors.iter().take(3).product()
        }
    }

    let (first_grid, second_grid) = data.split_once("\n\n").expect("invalid input data");

    let mut perm = [Color::K, Color::C, Color::M, Color::Y];
    let mut candidate = Vec::with_capacity(4);
    find_max(
        &mut perm,
        &mut candidate,
        first_grid.as_bytes(),
        second_grid.as_bytes(),
    )
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let grid = data.as_bytes();

    let mut colors = [0; const { Color::K as usize + 1 }];
    for c in grid.iter().filter(|c| **c != b'\n') {
        colors[Color::try_from(*c).expect("invalid color") as usize] += 1;
    }

    let k = colors[Color::K as usize];
    let sum = [Color::C, Color::M, Color::Y]
        .into_iter()
        .map(|color| colors[color as usize])
        .sum::<u64>();
    if [Color::C, Color::M, Color::Y]
        .into_iter()
        .all(|color| k < colors[color as usize])
    {
        let remainder = sum % 3;
        let sum3 = sum / 3;
        (sum3 + u64::from(remainder > 1)) * (sum3 + u64::from(remainder > 0)) * sum3
    } else {
        let remainder = sum % 2;
        let sum2 = sum / 2;
        (sum2 + remainder) * sum2 * k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"CCCCC
MMMMM
YYYYY
KKKKK
CCMYY

CMYKC
CMYKC
CMYKC
CMYKC
CMYKC"
            ),
            144,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"CCCCC
MMMMM
YYYYY
KKKKK
CCMYY

14231
14231
14231
14231
14231"
            ),
            180,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"CCMM
YYKK"
            ),
            18,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"CCCMMM
CCMMYY
CCCYKK
CMMYYK"
            ),
            343,
        );
    }

    #[test]
    fn test_part_3_3() {
        assert_eq!(
            part_3(
                r"CCCCYMMM
CCMMMYYY
CCCYYYMM
MMCCCYYY
YYYMMMCC
CCYYMMKK
MMYYCCKK
YYYCCMMM"
            ),
            8000,
        );
    }
}
