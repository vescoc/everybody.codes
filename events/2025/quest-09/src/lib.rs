use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

struct MergeSets {
    id: usize,
    nodes: HashMap<usize, usize>,
    sets: HashMap<usize, HashSet<usize>>,
}

impl MergeSets {
    fn new(nodes: usize) -> Self {
        Self {
            id: nodes,
            nodes: (0..nodes).map(|node| (node, node)).collect(),
            sets: (0..nodes)
                .map(|node| {
                    let mut set = HashSet::with_capacity(nodes);
                    set.insert(node);
                    (node, set)
                })
                .collect(),
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let set_a_id = self.nodes[&a];
        let set_b_id = self.nodes[&b];
        if set_a_id == set_b_id {
            return;
        }

        let mut union = self.sets.remove(&set_a_id).unwrap();
        for element in self.sets.remove(&set_b_id).unwrap() {
            union.insert(element);
        }

        let id = self.new_id();
        for element in &union {
            self.nodes.insert(*element, id);
        }

        self.sets.insert(id, union);
    }

    fn new_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }

    fn max(&self) -> &HashSet<usize> {
        self.sets.iter().max_by_key(|(_, set)| set.len()).unwrap().1
    }
}

fn similarity(first: &[u8], second: &[u8]) -> u128 {
    let mut result = 0u128;
    for (i, (first, second)) in first.iter().zip(second).enumerate() {
        if first == second {
            result |= 1 << i;
        }
    }

    result
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> usize {
    let dnas = data
        .lines()
        .map(|line| {
            let (_, dna) = line.split_once(':').expect("invalid dna");
            dna.as_bytes()
        })
        .collect::<Vec<_>>();

    let mut table = HashMap::with_capacity(1024);
    for first in 0..dnas.len() - 1 {
        for second in first + 1..dnas.len() {
            let similarity = similarity(dnas[first], dnas[second]);
            table.insert((first, second), similarity);
            table.insert((second, first), similarity);
        }
    }

    let mut mask = 0u128;
    for i in 0..dnas[0].len() {
        mask |= 1 << i;
    }

    let check_candidate = |candidate| {
        for first_parent in 0..dnas.len() - 1 {
            if first_parent == candidate {
                continue;
            }

            for second_parent in first_parent + 1..dnas.len() {
                if second_parent == candidate {
                    continue;
                }

                if table[&(candidate, first_parent)] | table[&(candidate, second_parent)] == mask {
                    return table[&(candidate, first_parent)].count_ones() as usize
                        * table[&(candidate, second_parent)].count_ones() as usize;
                }
            }
        }

        0
    };

    if dnas.len() > 3 {
        (0..dnas.len()).into_par_iter().map(check_candidate).sum()
    } else {
        (0..dnas.len()).map(check_candidate).sum()
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    solve(data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    solve(data)
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let dnas = data
        .lines()
        .map(|line| {
            let (_, dna) = line.split_once(':').expect("invalid dna");
            dna.as_bytes()
        })
        .collect::<Vec<_>>();

    let mut table = HashMap::with_capacity(1024);
    for first in 0..dnas.len() - 1 {
        for second in first + 1..dnas.len() {
            let similarity = similarity(dnas[first], dnas[second]);
            table.insert((first, second), similarity);
            table.insert((second, first), similarity);
        }
    }

    let mut mask = 0u128;
    for i in 0..dnas[0].len() {
        mask |= 1 << i;
    }

    let merge_sets = if dnas.len() > 3 {
        let merge_sets = std::sync::Mutex::new(MergeSets::new(dnas.len()));
        (0..dnas.len()).into_par_iter().for_each(|candidate| {
            for first_parent in 0..dnas.len() - 1 {
                if first_parent == candidate {
                    continue;
                }

                for second_parent in first_parent + 1..dnas.len() {
                    if second_parent == candidate {
                        continue;
                    }

                    if table[&(candidate, first_parent)] | table[&(candidate, second_parent)]
                        == mask
                    {
                        merge_sets.lock().unwrap().merge(candidate, first_parent);
                        merge_sets.lock().unwrap().merge(candidate, second_parent);
                        return;
                    }
                }
            }
        });

        merge_sets.into_inner().unwrap()
    } else {
        let mut merge_sets = MergeSets::new(dnas.len());
        for candidate in 0..dnas.len() {
            'outher: for first_parent in 0..dnas.len() - 1 {
                if first_parent == candidate {
                    continue;
                }

                for second_parent in first_parent + 1..dnas.len() {
                    if second_parent == candidate {
                        continue;
                    }

                    if table[&(candidate, first_parent)] | table[&(candidate, second_parent)]
                        == mask
                    {
                        merge_sets.merge(candidate, first_parent);
                        merge_sets.merge(candidate, second_parent);
                        break 'outher;
                    }
                }
            }
        }
        merge_sets
    };

    merge_sets.max().iter().map(|index| index + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG"
            ),
            414,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG"
            ),
            1245,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG"
            ),
            12,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG"
            ),
            36,
        );
    }

    #[test]
    fn merge_test() {
        let mut merge_set = MergeSets::new(500);

        merge_set.merge(1, 2);
        merge_set.merge(3, 4);
        merge_set.merge(1, 3);
        merge_set.merge(10, 11);
        merge_set.merge(11, 12);
        merge_set.merge(4, 11);

        let mut set = merge_set.max().iter().copied().collect::<Vec<_>>();
        set.sort_unstable();

        assert_eq!(&set, &[1, 2, 3, 4, 10, 11, 12]);
    }
}
