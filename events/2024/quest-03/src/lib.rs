use std::collections::HashSet;

type Map = HashSet<(usize, usize)>;

fn mine_simple(blocks: &Map) -> Map {
    let mut result = HashSet::with_capacity(blocks.len());
    for (r, c) in blocks {
        let mine = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .all(|(dr, dc)| {
                match (r.checked_add_signed(*dr), c.checked_add_signed(*dc)) {
                    (Some(r), Some(c)) => blocks.contains(&(r, c)),
                    _ => false,
                }
            });
        if mine {
            result.insert((*r, *c));
        }
    }

    result
}

fn mine(blocks: &Map) -> Map {
    let mut result = HashSet::with_capacity(blocks.len());
    for (r, c) in blocks {
        let mine = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (-1, -1), (1, -1), (1, 1)]
            .iter()
            .all(|(dr, dc)| {
                match (r.checked_add_signed(*dr), c.checked_add_signed(*dc)) {
                    (Some(r), Some(c)) => blocks.contains(&(r, c)),
                    _ => false,
                }
            });
        if mine {
            result.insert((*r, *c));
        }
    }

    result
}

#[must_use]
fn part(data: &[u8], mine: impl Fn(&Map) -> Map) -> usize {
    let mut blocks = HashSet::with_capacity(data.len());
    for (r, row) in data.split(|&c| c == b'\n').enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if tile == b'#' {
                blocks.insert((r, c));
            }
        }
    }

    let mut size = blocks.len();
    let mut blocks = mine(&blocks);
    loop {
        let current_size = blocks.len();
        if current_size == 0 {
            break;
        }
        size += current_size;
        blocks = mine(&blocks);
    }
    
    size
}

#[must_use]
pub fn part_1(data: &[u8]) -> usize {
    part(data, mine_simple)
}

#[must_use]
pub fn part_2(data: &[u8]) -> usize {
    part(data, mine_simple)
}

#[must_use]
pub fn part_3(data: &[u8]) -> usize {
    part(data, mine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_common() {
        assert_eq!(35, part(br"..........
..###.##..
...####...
..######..
..######..
...####...
..........", mine_simple));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(29, part_3(br"..........
..###.##..
...####...
..######..
..######..
...####...
.........."));
    }
}
