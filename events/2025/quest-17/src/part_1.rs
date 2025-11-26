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
                        if a * a + b * b <= 10 * 10 {
                            Some(u64::from(cell - b'0'))
                        } else {
                            None
                        }
                    }
                    b'@' => Some(0),
                    _ => unreachable!(),
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                r"189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131"
            ),
            1573
        );
    }
}
