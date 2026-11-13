use std::collections::BTreeSet;

use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Info(usize, usize, u128);

impl Ord for Info {
    fn cmp(&self, Info(bindings, n, _): &Info) -> Ordering {
        n.cmp(&self.1).then(self.0.cmp(bindings))
    }
}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Info) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let map = data.as_bytes();
    let width = map
        .iter()
        .position(|&c| c == b'\n')
        .expect("invalid input data");
    let height = (map.len() + 1) / (width + 1);

    let mut bindings = width * 2 + height * 2;

    // vertical bindings
    bindings += map
        .chunks(width + 1)
        .map(|row| {
            row[..width]
                .windows(2)
                .filter(|pair| pair[0] != pair[1])
                .count()
        })
        .sum::<usize>();

    // horizontal bindings
    bindings += (0..width)
        .map(|x| {
            (0..height - 1)
                .filter(|y| map[y * (width + 1) + x] != map[(y + 1) * (width + 1) + x])
                .count()
        })
        .sum::<usize>();

    bindings
}

fn parse_connections(data: &str) -> (u128, Box<[[usize; 128]]>) {
    let map = data.as_bytes();
    let width = map
        .iter()
        .position(|&c| c == b'\n')
        .expect("invalid input data");
    let height = (map.len() + 1) / (width + 1);

    let mut all_mask = 1u128; // first bit is the frame
    let mut connections = vec![[0; 128]; 128].into_boxed_slice();
    for row in map.chunks(width + 1) {
        let row = &row[..width];
        for pair in row.windows(2) {
            let a = pair[0] as usize;
            let b = pair[1] as usize;
            connections[a][b] += 1;
            connections[b][a] += 1;

            all_mask |= 1 << a;
            all_mask |= 1 << b;
        }

        let first = row[0] as usize;
        connections[0][first] += 1;
        connections[first][0] += 1;

        let last = row[row.len() - 1] as usize;
        connections[0][last] += 1;
        connections[last][0] += 1;
    }

    for x in 0..width {
        for y in 0..height - 1 {
            let a = map[y * (width + 1) + x] as usize;
            let b = map[(y + 1) * (width + 1) + x] as usize;
            connections[a][b] += 1;
            connections[b][a] += 1;

            all_mask |= 1 << a;
            all_mask |= 1 << b;
        }

        let first = map[x] as usize;
        connections[0][first] += 1;
        connections[first][0] += 1;

        let last = map[(height - 1) * (width + 1) + x] as usize;
        connections[0][last] += 1;
        connections[last][0] += 1;
    }

    (all_mask, connections)
}

fn find_bindings(all_mask: u128, connections: &[[usize; 128]], mut sets: BTreeSet<Info>) -> usize {
    while let Some(Info(count, n, set)) = sets.pop_first() {
        if set == all_mask {
            return count;
        }

        for (other_index, connections) in connections.iter().enumerate() {
            let other = 1u128 << other_index;
            if other & all_mask != 0 && set & other == 0 {
                for (index, &bindings) in connections.iter().enumerate() {
                    if bindings > 0 && set & (1u128 << index) != 0 {
                        sets.insert(Info(count + bindings, n + 1, set | other));
                    }
                }
            }
        }
    }

    unreachable!("not connected???")
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let (all_mask, connections) = parse_connections(data);

    let sets = (0..128)
        .filter_map(|index| {
            let set = 1u128 << index;
            if all_mask & set != 0 {
                Some(Info(0, 1, set))
            } else {
                None
            }
        })
        .take(1)
        .collect::<BTreeSet<_>>();

    find_bindings(all_mask, &connections, sets)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let (all_mask, connections) = parse_connections(data);

    let mut bindings = usize::MAX;

    // ignore the frame
    for first in 1..128 - 1 {
        let set = 1u128 << first;
        if all_mask & set == 0 {
            continue;
        }

        let first_set =
            connections[first]
                .iter()
                .enumerate()
                .fold(set, |set, (index, &connections)| {
                    if connections > 0 {
                        set | (1 << index)
                    } else {
                        set
                    }
                });

        for second in first + 1..128 {
            let set = 1u128 << second;
            if all_mask & set == 0 || first_set & set != 0 {
                continue;
            }

            let second_set =
                connections[second]
                    .iter()
                    .enumerate()
                    .fold(set, |set, (index, &connections)| {
                        if connections > 0 {
                            set | (1 << index)
                        } else {
                            set
                        }
                    });

            let sets = BTreeSet::from([Info(0, 2, first_set | second_set)]);

            let result = find_bindings(all_mask, &connections, sets);

            bindings = bindings.min(result);
        }
    }

    bindings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"AAAEEEFFFF
AADDDDDGFF
BACDDIGGFF
BABBIIIGGH
BBBBIIIIHH"
            ),
            67,
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            part_2(
                r"AAAEEEFFFF
AADDDDDGFF
BACDDIGGFF
BABBIIIGGH
BBBBIIIIHH"
            ),
            12,
        );
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(
            part_2(
                r"IIIIIILOHHHHHHHHGGGGGQSRRRRKKK
IIIIIILOOHHHHHHGGXGGGQSRRRRKKK
PPPPILLLOOOODDHBGGGGGQSSRRRKKK
PPZPLLLLFFOODDBBBBGGQQQSSRKKKK
PPPPLLLLFFFONDDDBBMGGQQSSKKJJJ
PPPCCCCCCFFONNDDMMMGQQQSSSKJJJ
PCCCCCCCCFFNNDDDDMAAQQQSSSKKJJ
PPCCECCCFFNNNNDADMAATQTTSSSJJJ
PPEEEEECCFFNNAAAMMATTTTTSTSSSJ
PPEEEEEEEFFNNNAAAAAATTTTTTSSJJ"
            ),
            63,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"AAAEEEFFFF
AADDDDDGFF
BACDDIGGFF
BABBIIIGGH
BBBBIIIIHH"
            ),
            0,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"IIIIIILOHHHHHHHHGGGGGQSRRRRKKK
IIIIIILOOHHHHHHGGXGGGQSRRRRKKK
PPPPILLLOOOODDHBGGGGGQSSRRRKKK
PPZPLLLLFFOODDBBBBGGQQQSSRKKKK
PPPPLLLLFFFONDDDBBMGGQQSSKKJJJ
PPPCCCCCCFFONNDDMMMGQQQSSSKJJJ
PCCCCCCCCFFNNDDDDMAAQQQSSSKKJJ
PPCCECCCFFNNNNDADMAATQTTSSSJJJ
PPEEEEECCFFNNAAAMMATTTTTSTSSSJ
PPEEEEEEEFFNNNAAAAAATTTTTTSSJJ"
            ),
            21,
        );
    }
}
