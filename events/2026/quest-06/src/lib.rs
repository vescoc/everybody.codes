use std::collections::{HashMap, HashSet, VecDeque};

use rayon::prelude::*;

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let records = data
        .lines()
        .map(|line| {
            let (head, rest) = line.split_once(':').expect("invalid line");
            let rest = rest.split(',').collect::<Vec<_>>();
            (head, rest)
        })
        .collect::<HashMap<_, _>>();

    visit(&records, "NULL").len().saturating_sub(1)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let mut records = HashMap::with_capacity(1024);
    for (head, next) in data.lines().flat_map(|line| {
        let (head, rest) = line.split_once(':').expect("invalid line");
        rest.split(',').map(move |next| (next, head))
    }) {
        records.entry(head).or_insert_with(Vec::new).push(next);
    }

    visit(&records, "NULL").len().saturating_sub(1)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let mut nodes = HashSet::with_capacity(1024);
    let mut direct_records = HashMap::with_capacity(1024);
    let mut indirect_records = HashMap::with_capacity(1024);
    for (head, neighbors) in data.lines().map(|line| {
        let (head, rest) = line.split_once(':').expect("invalid line");
        (head, rest.split(',').collect::<Vec<_>>())
    }) {
        if !head.starts_with("NULL") {
            nodes.insert(head);
        }

        direct_records.insert(head, neighbors.clone());

        for next in neighbors {
            indirect_records
                .entry(next)
                .or_insert_with(Vec::new)
                .push(head);

            if !next.starts_with("NULL") {
                nodes.insert(next);
            }
        }
    }

    let (null_v1_set, (null_v2_set, null_v3_set)) = rayon::join(
        || visit(&indirect_records, "NULLv1"),
        || {
            rayon::join(
                || visit(&indirect_records, "NULLv2"),
                || visit(&indirect_records, "NULLv3"),
            )
        },
    );

    nodes
        .par_iter()
        .filter(|node| {
            let count = u64::from(null_v1_set.contains(*node))
                + u64::from(null_v2_set.contains(*node))
                + u64::from(null_v3_set.contains(*node));
            if count > 1 {
                return true;
            }

            let visited = visit(&direct_records, node);

            let count = u64::from(visited.contains("NULLv1"))
                + u64::from(visited.contains("NULLv2"))
                + u64::from(visited.contains("NULLv3"));

            count > 1
        })
        .count()
}

fn visit<'a>(records: &HashMap<&'a str, Vec<&'a str>>, start: &'a str) -> HashSet<&'a str> {
    let mut visited = HashSet::with_capacity(1024);
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }

        if let Some(neighbors) = records.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"BRDS:FLWR,SLTT,YSTS
FLWR:SLTT,YSTS
SLTT:YSTS,MLKK,BTTR
NULL:BRDS,FLWR,SLTT
YSTS:BRDS,FLWR
MLKK:SPCS,NULL
HNYN:BRDS,FLWR,SLTT,YSTS"
            ),
            7,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"BRDS:FLWR,SLTT,YSTS
FLWR:SLTT,YSTS
SLTT:YSTS,MLKK,BTTR
NULL:BRDS,FLWR,SLTT
YSTS:BRDS,FLWR
MLKK:SPCS,NULL
HNYN:BRDS,FLWR,SLTT,YSTS"
            ),
            6,
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"MEAL:BRDS,CAKE,DOUG
BRDS:DOUG,FLWR
CAKE:DOUG,FLWR,NULLv2,NULLv3
DOUG:FLWR,YSTS,SLTT,NULLv1
FLWR:BRDS,DOUG
YSTS:SPCS
SOUP:BRDS,CAKE,DOUG,FLWR"
            ),
            3,
        );
    }
}
