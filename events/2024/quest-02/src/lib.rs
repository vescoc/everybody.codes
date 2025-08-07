use rayon::prelude::*;

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> usize {
    let mut parts = data.split(|c| *c == b'\n');

    let words = parts.next().unwrap()["WORDS:".len()..].split(|c| *c == b',');

    let inscription = parts.nth(1).unwrap();

    words
        .map(|word| {
            inscription
                .windows(word.len())
                .filter(|part| part == &word)
                .count()
        })
        .sum()
}

fn match_horizontally(words: &[&[u8]], inscription: &[u8], i: usize, c: u8) -> bool {
    let len = inscription.len();
    words.iter().any(|word| {
        for (j, &wc) in word.iter().enumerate() {
            if wc == c {
                let forward = word.iter().enumerate().all(|(jj, &wc)| {
                    if i + jj < j || i + jj - j >= len {
                        false
                    } else {
                        wc == inscription[i + jj - j]
                    }
                });
                if forward {
                    return true;
                }

                let w_len = word.len() - 1;
                let backward = word.iter().rev().enumerate().all(|(jj, &wc)| {
                    if i + j + jj < w_len || i + j + jj - w_len >= len {
                        false
                    } else {
                        wc == inscription[i + j + jj - w_len]
                    }
                });
                if backward {
                    return true;
                }
            }
        }
        false
    })
}

fn match_hv(
    words: &[&[u8]],
    inscriptions: &[&[u8]],
    inscription: &[u8],
    x: usize,
    y: usize,
    c: u8,
) -> bool {
    let height = inscriptions.len();
    let width = inscription.len();
    words.iter().any(|word| {
        for (j, &wc) in word.iter().enumerate() {
            if wc == c {
                let forward = word
                    .iter()
                    .enumerate()
                    .all(|(jj, &wc)| wc == inscription[(width + x + jj - j) % width]);
                if forward {
                    return true;
                }

                let w_len = word.len() - 1;
                let backward = word
                    .iter()
                    .rev()
                    .enumerate()
                    .all(|(jj, &wc)| wc == inscription[(width + x + j + jj - w_len) % width]);
                if backward {
                    return true;
                }

                let up = word.iter().enumerate().all(|(jj, &wc)| {
                    if y + jj < j || y + jj - j >= height {
                        return false;
                    }
                    wc == inscriptions[y + jj - j][x]
                });
                if up {
                    return true;
                }

                let down = word.iter().rev().enumerate().all(|(jj, &wc)| {
                    if y + j + jj < w_len || y + j + jj - w_len >= height {
                        return false;
                    }
                    wc == inscriptions[y + j + jj - w_len][x]
                });
                if down {
                    return true;
                }
            }
        }
        false
    })
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> usize {
    let mut parts = data.split(|c| *c == b'\n');

    let words = parts.next().unwrap()["WORDS:".len()..]
        .split(|c| *c == b',')
        .collect::<Vec<_>>();

    parts
        .skip(1)
        .par_bridge()
        .flat_map_iter(|inscription| {
            inscription
                .iter()
                .enumerate()
                .filter(|(i, c)| match_horizontally(&words, inscription, *i, **c))
        })
        .count()
}

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> usize {
    let mut parts = data.split(|c| *c == b'\n');

    let words = parts.next().unwrap()["WORDS:".len()..]
        .split(|c| *c == b',')
        .collect::<Vec<_>>();

    let inscriptions = parts.skip(1).collect::<Vec<_>>();

    inscriptions
        .par_iter()
        .enumerate()
        .flat_map_iter(|(y, inscription)| {
            let words = &words;
            let inscriptions = &inscriptions;
            inscription
                .iter()
                .enumerate()
                .filter(move |(x, c)| match_hv(words, inscriptions, inscription, *x, y, **c))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            4,
            part_1(
                br"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            37,
            part_2(
                br"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END"
            )
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            15,
            part_2(
                br"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE"
            )
        );
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(
            9,
            part_2(
                br"WORDS:THE,OWE,MES,ROD,HER

THE FLAME SHIELDED THE HEART OF THE KINGS"
            )
        );
    }

    #[test]
    fn test_part_2_3() {
        assert_eq!(
            6,
            part_2(
                br"WORDS:THE,OWE,MES,ROD,HER

POWE PO WER P OWE R"
            )
        );
    }

    #[test]
    fn test_part_2_4() {
        assert_eq!(
            7,
            part_2(
                br"WORDS:THE,OWE,MES,ROD,HER

THERE IS THE END"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            10,
            part_3(
                br"WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"
            )
        );
    }
}
