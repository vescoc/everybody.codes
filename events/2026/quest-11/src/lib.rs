use std::collections::VecDeque;

fn teleports(
    portals: usize,
    sides: usize,
    shortcuts: impl Iterator<Item = (usize, usize)>,
) -> usize {
    let mut paths = vec![vec![false; portals]; portals];
    for n in 0..portals {
        for side in 0..sides {
            paths[n][(n + portals + side + 1).rem_euclid(portals)] = true;
            paths[n][(n + portals - side - 1).rem_euclid(portals)] = true;
        }
    }
    for (from, to) in shortcuts {
        paths[from][to] = true;
        paths[to][from] = true;
    }

    (0..portals)
        .map(|start| {
            let mut sum = 0;
            let mut queue = VecDeque::with_capacity(portals * portals);
            let mut visited = vec![false; portals];
            queue.push_back((start, 0));
            while let Some((current, distance)) = queue.pop_front() {
                if visited[current] {
                    continue;
                }
                visited[current] = true;
                sum += distance;
                for (next, _) in paths[current]
                    .iter()
                    .enumerate()
                    .filter(|(_, connected)| **connected)
                {
                    queue.push_back((next, distance + 1));
                }
            }
            sum
        })
        .sum()
}

fn teleports_crystals_2(
    portals: usize,
    sides: usize,
    shortcuts: impl Iterator<Item = (usize, usize)>,
) -> usize {
    let mut paths = vec![vec![false; portals]; portals];
    for n in 0..portals {
        for side in 0..sides {
            paths[n][(n + portals + side + 1).rem_euclid(portals)] = true;
            paths[n][(n + portals - side - 1).rem_euclid(portals)] = true;
        }
    }
    for (from, to) in shortcuts {
        paths[from][to] = true;
        paths[to][from] = true;
    }

    let mut min = usize::MAX;
    for crystal_1 in 1..portals - 1 {
        for crystal_2 in crystal_1..portals {
            if !paths[crystal_1][crystal_2] && !paths[crystal_2][crystal_1] {
                paths[crystal_1][crystal_2] = true;
                paths[crystal_2][crystal_1] = true;
                let candidate = (0..portals)
                    .map(|start| {
                        let mut sum = 0;
                        let mut queue = VecDeque::with_capacity(portals * portals);
                        let mut visited = vec![false; portals];
                        queue.push_back((start, 0));
                        while let Some((current, distance)) = queue.pop_front() {
                            if visited[current] {
                                continue;
                            }
                            visited[current] = true;
                            sum += distance;
                            for (next, _) in paths[current]
                                .iter()
                                .enumerate()
                                .filter(|(_, connected)| **connected)
                            {
                                queue.push_back((next, distance + 1));
                            }
                        }
                        sum
                    })
                    .sum();
                min = min.min(candidate);

                paths[crystal_1][crystal_2] = false;
                paths[crystal_2][crystal_1] = false;
            }
        }
    }

    min
}

fn pairs_set(values: &mut [usize]) -> Vec<Vec<(usize, usize)>> {
    fn pairs_set_r(values: &mut [usize], result: &mut Vec<Vec<(usize, usize)>>) {
        let Some((first, rest)) = values.split_first_mut() else {
            return;
        };

        let len = rest.len();
        for i in 0..len {
            let second = rest[i];
            rest[i..].rotate_left(1);

            let mut pairs_set = vec![];
            pairs_set_r(&mut rest[..len - 1], &mut pairs_set);

            if pairs_set.is_empty() {
                let mut v = Vec::with_capacity(4);
                v.push((*first, second));
                result.push(v);
            } else {
                for mut pairs in pairs_set {
                    pairs.push((*first, second));
                    result.push(pairs);
                }
            }

            rest[i..].rotate_right(1);
        }
    }

    let mut result = Vec::with_capacity(1024);

    pairs_set_r(values, &mut result);

    result
}

fn teleports_crystals_8(
    portals: usize,
    sides: usize,
    shortcuts: impl Iterator<Item = (usize, usize)>,
) -> usize {
    let mut paths = vec![vec![false; portals]; portals];
    for n in 0..portals {
        for side in 0..sides {
            paths[n][(n + portals + side + 1).rem_euclid(portals)] = true;
            paths[n][(n + portals - side - 1).rem_euclid(portals)] = true;
        }
    }
    for (from, to) in shortcuts {
        paths[from][to] = true;
        paths[to][from] = true;
    }

    let pairs_set = pairs_set(&mut [0, 1, 2, 3, 4, 5, 6, 7]);

    let mut min = usize::MAX;
    let steps = portals / 8;
    for start in 0..steps {
        for pairs in &pairs_set {
            if pairs.iter().any(|(a, b)| {
                paths[a * steps + start][b * steps + start]
                    || paths[b * steps + start][a * steps + start]
            }) {
                continue;
            }

            for (a, b) in pairs {
                paths[a * steps + start][b * steps + start] = true;
                paths[b * steps + start][a * steps + start] = true;
            }

            let candidate = (0..portals)
                .map(|start| {
                    let mut sum = 0;
                    let mut queue = VecDeque::with_capacity(portals * portals);
                    let mut visited = vec![false; portals];
                    queue.push_back((start, 0));
                    while let Some((current, distance)) = queue.pop_front() {
                        if visited[current] {
                            continue;
                        }
                        visited[current] = true;
                        sum += distance;
                        for (next, _) in paths[current]
                            .iter()
                            .enumerate()
                            .filter(|(_, connected)| **connected)
                        {
                            queue.push_back((next, distance + 1));
                        }
                    }
                    sum
                })
                .sum();
            min = min.min(candidate);

            for (a, b) in pairs {
                paths[a * steps + start][b * steps + start] = false;
                paths[b * steps + start][a * steps + start] = false;
            }
        }
    }

    min
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let (portals_part, shortcuts_part) = data.split_once("\n\n").expect("invalid input data");
    let portals = portals_part
        .strip_prefix("portals: ")
        .expect("invalid portals part")
        .parse::<usize>()
        .expect("invalid portals");

    let portal_strain = {
        let half = portals / 2;
        (half * (half + 1) + (half - 1) * half) * portals / 2
    };

    let portal_teleports = teleports(
        portals,
        1,
        shortcuts_part.lines().map(|line| {
            let (from, to) = line.split_once(' ').expect("invalid shortcuts part");
            let from = from.parse::<usize>().expect("invalid shortcut from");
            let to = to.parse::<usize>().expect("invalid shortcut to");

            (from - 1, to - 1)
        }),
    );

    portal_strain - portal_teleports
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let (portals_part, shortcuts_part) = data.split_once("\n\n").expect("invalid input data");
    let portals = portals_part
        .strip_prefix("portals: ")
        .expect("invalid portals part")
        .parse::<usize>()
        .expect("invalid portals");

    let portal_strain = teleports(portals, 3, std::iter::empty());

    let portal_teleports = teleports_crystals_2(
        portals,
        3,
        shortcuts_part.lines().map(|line| {
            let (from, to) = line.split_once(' ').expect("invalid shortcuts part");
            let from = from.parse::<usize>().expect("invalid shortcut from");
            let to = to.parse::<usize>().expect("invalid shortcut to");

            (from - 1, to - 1)
        }),
    );

    portal_strain - portal_teleports
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let (portals_part, shortcuts_part) = data.split_once("\n\n").expect("invalid input data");
    let portals = portals_part
        .strip_prefix("portals: ")
        .expect("invalid portals part")
        .parse::<usize>()
        .expect("invalid portals");

    let portal_strain = teleports(portals, 3, std::iter::empty());

    let portal_teleports = teleports_crystals_8(
        portals,
        3,
        shortcuts_part.lines().map(|line| {
            let (from, to) = line.split_once(' ').expect("invalid shortcuts part");
            let from = from.parse::<usize>().expect("invalid shortcut from");
            let to = to.parse::<usize>().expect("invalid shortcut to");

            (from - 1, to - 1)
        }),
    );

    portal_strain - portal_teleports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"portals: 10

5 10
2 7"
            ),
            60,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"portals: 10

5 10
2 7"
            ),
            6,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"portals: 32

1 10
20 30
15 29
13 10"
            ),
            854,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"portals: 64

1 40
3 35
5 42
7 50"
            ),
            9634,
        );
    }
}
