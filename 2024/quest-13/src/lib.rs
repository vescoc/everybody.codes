use std::collections::{HashSet, HashMap};

type Node = (usize, usize);

fn neighbours<'a>(map: &'a [&'a [u8]], visited: &'a HashSet<Node>, nodes: &'a HashSet<Node>, node: &'a Node) -> impl Iterator<Item = (Node, usize)> + use<'a> {
    let a = map[node.0][node.1];
    
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(move |(dr, dc)| {
            let r = node.0.checked_add_signed(*dr);
            let c = node.1.checked_add_signed(*dc);

            match (r, c) {
                (Some(r), Some(c)) if nodes.contains(&(r, c)) || !visited.contains(&(r, c)) => {
                    map
                        .get(r)
                        .and_then(|row| row.get(c))
                        .and_then(|&b| if b == b'#' || b == b' ' { None } else { Some(b) })
                        .map(|b| ((r, c), distance(a, b)))
                }
                _ => None,
            }
        })
}

fn parse(value: u8) -> usize {
    match value {
        b'S' | b'E' => 0,
        value => value as usize - b'0' as usize,
    }
}

fn distance(a: u8, b: u8) -> usize {
    if a == b {
        return 1;
    }

    let a = parse(a);
    let b = parse(b);

    let diff = a.abs_diff(b);

    diff.min(10 - a + b).min(10 - b + a) + 1
}

fn find_minimum(nodes: &HashSet<Node>, dist: &HashMap<Node, usize>) -> Option<Node> {
    nodes
        .iter()
        .filter_map(|node| dist.get(node).map(|dist| (node, dist)))
        .min_by_key(|(_, dist)| **dist)
        .map(|(node, _)| *node)
}

/// # Panics
#[must_use]
pub fn solve<const FROM: u8, const TO: u8>(data: &[u8]) -> usize {
    let map = data.split(|&c| c == b'\n').collect::<Vec<_>>();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|tile| *tile == FROM).map(|c| (r, c)))
        .unwrap();

    let mut visited = HashSet::with_capacity(data.len());
    let mut dist = HashMap::with_capacity(data.len());
    let mut nodes = HashSet::with_capacity(data.len());

    nodes.insert(start);
    dist.insert(start, 0);

    while let Some(node) = find_minimum(&nodes, &dist) {
        if map[node.0][node.1] == TO {
            return dist[&node];
        }
        
        nodes.remove(&node);
        visited.insert(node);

        let current_distance = dist[&node];
        
        let mut added_nodes = vec![];
        for (next, distance) in neighbours(&map, &visited, &nodes, &node) {
            assert!(!visited.contains(&next));
            
            added_nodes.push(next);
            
            let alt = current_distance + distance;
            if let Some(current_distance) = dist.get_mut(&next) {
                if alt < *current_distance {
                    *current_distance = alt;
                }
            } else {
                dist.insert(next, alt);
            }
        }

        for node in added_nodes {
            nodes.insert(node);
        }
    }
    
    unreachable!()
}

pub fn part_1(data: &[u8]) -> usize {
    solve::<b'S', b'E'>(data)
}

pub fn part_2(data: &[u8]) -> usize {
    solve::<b'S', b'E'>(data)
}

pub fn part_3(data: &[u8]) -> usize {
    solve::<b'E', b'S'>(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(28, part_1(br"#######
#6769##
S50505E
#97434#
#######"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(28, part_2(br"#######
#6769##
S50505E
#97434#
#######"));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(14, part_3(br"SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS"));
    }
}
