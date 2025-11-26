struct Destruction(Vec<u64>);

impl FromIterator<(usize, u64)> for Destruction {
    fn from_iter<II: IntoIterator<Item = (usize, u64)>>(ii: II) -> Self {
        let mut result = [0; 128];
        for (distance, value) in ii {
            result[distance] += value;
        }
        Destruction(result.to_vec())
    }
}

impl Destruction {
    fn into_iter(self) -> impl Iterator<Item = (usize, u64)> {
        self.0.into_iter().enumerate().skip(1)
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> u64 {
    let map = data.as_bytes();
    let columns = map
        .iter()
        .position(|cell| *cell == b'\n')
        .expect("Invalid data");
    let rows = (map.len() + 1) / (columns + 1);

    let (x_v, y_v) = (columns / 2, rows / 2);
    assert!(map[y_v * (columns + 1) + x_v] == b'@');

    map.chunks(columns + 1)
        .enumerate()
        .flat_map(|(y_c, row)| {
            row.iter()
                .take(columns)
                .enumerate()
                .filter_map(move |(x_c, cell)| match cell {
                    b'1'..=b'9' => {
                        let a = x_v.abs_diff(x_c);
                        let b = y_v.abs_diff(y_c);
                        let r_2 = a * a + b * b;

                        let mut r = r_2.isqrt();
                        while r * r < r_2 {
                            r += 1;
                        }
                        Some((r, u64::from(cell - b'0')))
                    }
                    b'@' => None,
                    _ => unreachable!(),
                })
        })
        .collect::<Destruction>()
        .into_iter()
        // .inspect(|p| println!("{p:?}"))
        .max_by_key(|(_, s)| *s)
        .map(|(r, s)| r as u64 * s)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                r"4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449"
            ),
            1090
        );
    }
}
