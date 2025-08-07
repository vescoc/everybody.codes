use std::collections::{HashMap, VecDeque};

const START_NODE: &[u8] = b"RR";

fn solve(data: &[u8], encode_node: impl Fn(&[u8]) -> &str) -> String {
    let mut tree = HashMap::with_capacity(4 * 1_024);
    for line in data.split(|&c| c == b'\n') {
        let mut parts = line.split(|&c| c == b':');
        let node = parts.next().unwrap();
        for branch in parts.next().unwrap().split(|&c| c == b',') {
            tree.entry(node)
                .or_insert(Vec::with_capacity(8))
                .push(branch);
        }
    }

    let mut candidate: Option<(String, _, _)> = None;
    let mut queue = VecDeque::with_capacity(10 * 1_024);
    queue.push_back((START_NODE, encode_node(START_NODE).to_string(), 0));
    while let Some((current, path, length)) = queue.pop_front() {
        if let Some(branches) = tree.get(current) {
            for node in branches {
                if node == b"@" {
                    if let Some((current_path, current_length, count)) = candidate.as_mut() {
                        if *current_length < length {
                            if *count == 1 {
                                return format!("{current_path}@");
                            }
                            current_path.clone_from(&path);
                            *current_length = length;
                            *count = 1;
                        } else {
                            *count += 1;
                        }
                    } else {
                        candidate = Some((path.clone(), length, 1));
                    }
                } else {
                    queue.push_back((node, path.to_string() + encode_node(node), length + 1));
                }
            }
        }
    }

    unreachable!()
}

#[must_use]
pub fn part_1(data: &[u8]) -> String {
    solve(data, |node| unsafe { std::str::from_utf8_unchecked(node) })
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> String {
    solve(data, |node| unsafe {
        std::str::from_utf8_unchecked(&node[0..1])
    })
}

pub use part_2 as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "RRB@",
            part_1(
                br"RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "RB@",
            part_2(
                br"RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            "RB@",
            part_2(
                br"RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"
            )
        );
    }
}
