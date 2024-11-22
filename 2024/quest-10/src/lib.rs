use std::collections::{HashMap, HashSet, VecDeque};

/// # Panics
#[allow(clippy::match_on_vec_items)]
#[must_use]
pub fn part_1(data: &[u8]) -> String {
    let mut data = data.to_vec();

    let _ = decode_runes::<8>(&mut data, (0, 0), 8);

    runes::<8>(&data, (0, 0), 8)
        .collect::<Result<String, _>>()
        .unwrap()
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> u32 {
    let width = data.iter().position(|&c| c == b'\n').unwrap();

    data.chunks((width + 1) * 8 + 1)
        .map(|data| {
            let mut data = data.to_vec();

            (0..(width + 1) / 9)
                .map(|n| {
                    let _ = decode_runes::<9>(&mut data, (n, 0), width);

                    power::<9>(&data, (n, 0), width).unwrap()
                })
                .sum::<u32>()
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> u32 {
    let width = data.iter().position(|&c| c == b'\n').unwrap();
    let height = (data.len() + 1) / width;

    let mut data = data.to_vec();
    let mut check: Option<VecDeque<(usize, usize)>> = None;
    let mut result = 0;
    std::iter::from_fn(|| {
        if let Some(current_check) = check.take() {
            let mut new_check = VecDeque::with_capacity(current_check.capacity());
            result += current_check
                .into_iter()
                .map(|(nc, nr)| {
                    let check = decode_runes::<6>(&mut data, (nc, nr), width);
                    if decode_unknown::<6>(&mut data, check, (nc, nr), width) {
                        power::<6>(&data, (nc, nr), width).unwrap_or_default()
                    } else {
                        new_check.push_back((nc, nr));

                        0
                    }
                })
                .sum::<u32>();

            check = Some(new_check);

            Some(result)
        } else {
            let mut new_check = VecDeque::with_capacity(width / 6 * height / 6);
            result = (0..(height - 2) / 6)
                .map(|nr| {
                    (0..(width - 2) / 6)
                        .map(|nc| {
                            let check = decode_runes::<6>(&mut data, (nc, nr), width);
                            if decode_unknown::<6>(&mut data, check, (nc, nr), width) {
                                power::<6>(&data, (nc, nr), width).unwrap_or_default()
                            } else {
                                new_check.push_back((nc, nr));

                                0
                            }
                        })
                        .sum::<u32>()
                })
                .sum::<u32>();

            check = Some(new_check);

            Some(result)
        }
    })
    .nth(1)
    .unwrap()
}

fn runes<const THUMB: usize>(
    data: &[u8],
    (nc, nr): (usize, usize),
    width: usize,
) -> impl Iterator<Item = Result<char, &'static str>> + use<'_, THUMB> {
    (2..=5).flat_map(move |r| {
        (2..=5).map(
            move |c| match data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] {
                b'.' => Err("unresolved"),
                b'*' | b'?' => panic!("invalid"),
                l => Ok(char::from(l)),
            },
        )
    })
}

#[allow(clippy::cast_possible_truncation)]
fn power<const THUMB: usize>(
    data: &[u8],
    (nc, nr): (usize, usize),
    width: usize,
) -> Result<u32, &'static str> {
    runes::<THUMB>(data, (nc, nr), width)
        .enumerate()
        .try_fold(0_u32, |acc, (i, rune)| {
            rune.map(|rune| acc + (i as u32 + 1) * (rune as u32 - 'A' as u32 + 1))
        })
}

fn decode_runes<const THUMB: usize>(
    data: &mut [u8],
    (nc, nr): (usize, usize),
    width: usize,
) -> VecDeque<(usize, usize)> {
    let mut check = VecDeque::new();
    for r in 2..=5 {
        for c in 2..=5 {
            if data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] == b'.' {
                let mut mark = false;

                let row = (0..8)
                    .filter_map(
                        |c| match data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] {
                            b'?' => {
                                mark = true;
                                None
                            }
                            b'*' | b'.' => None,
                            l => Some(l),
                        },
                    )
                    .collect::<HashSet<_>>();

                let col = (0..8)
                    .filter_map(
                        |r| match data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] {
                            b'?' => {
                                mark = true;
                                None
                            }
                            b'*' | b'.' => None,
                            l => Some(l),
                        },
                    )
                    .collect::<HashSet<_>>();

                let mut intersection = row.intersection(&col);
                if let Some(l) = intersection.next() {
                    if intersection.next().is_none() {
                        assert!(*l != b'.');
                        data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] = *l;
                    } else if mark {
                        check.push_back((c, r));
                    }
                } else if mark {
                    check.push_back((c, r));
                }
            }
        }
    }

    check
}

#[allow(clippy::match_on_vec_items)]
fn decode_unknown<const THUMB: usize>(
    data: &mut [u8],
    mut check: VecDeque<(usize, usize)>,
    (nc, nr): (usize, usize),
    width: usize,
) -> bool {
    let mut resolved = check.is_empty();
    while let Some((c, r)) = check.pop_front() {
        let mut mark = vec![];

        let mut map: HashMap<u8, usize> = HashMap::with_capacity(16);
        for c in 0..8 {
            match data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] {
                b'?' => mark.push((c, r)),
                b'.' => *map.entry(b'.').or_default() += 1,
                b'*' => {}
                l => *map.entry(l).or_default() += 1,
            }
        }
        for r in 0..8 {
            match data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] {
                b'?' => mark.push((c, r)),
                b'.' => *map.entry(b'.').or_default() += 1,
                b'*' => {}
                l => *map.entry(l).or_default() += 1,
            }
        }

        if mark.len() > 1 {
            resolved = false;
            continue;
        }

        if matches!(map.get(&b'.'), Some(&count) if count > 2) {
            resolved = false;
            continue;
        }

        if let Some((mark_c, mark_r)) = mark.pop() {
            let mut singles = map.iter().filter(|(_, &v)| v == 1);
            if let Some((&l, _)) = singles.next() {
                if singles.next().is_none() {
                    assert!(l != b'.');
                    data[nc * THUMB + (nr * THUMB + r) * (width + 1) + c] = l;
                    data[nc * THUMB + (nr * THUMB + mark_r) * (width + 1) + mark_c] = l;
                } else {
                    resolved = false;
                }
            } else {
                resolved = false;
            }
        }
    }

    resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "PTBVRCZHFLJWGMNS".to_string(),
            part_1(
                br"**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            1851,
            part_2(
                br"**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            3889,
            part_3(
                br"**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**"
            )
        );
    }
}
