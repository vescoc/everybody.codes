use std::collections::HashMap;

use crate::{Command, parse_line};

#[derive(Debug, Default)]
struct BTree<ID, R, S> {
    nodes: Vec<BNode<ID, R, S>>,
    levels: HashMap<usize, usize>,
}

#[derive(Debug)]
struct BNode<ID, R, S> {
    id: ID,
    rank: R,
    symbol: S,
    left: Option<usize>,
    right: Option<usize>,
    level: usize,
}

impl<ID, R, S> BTree<ID, R, S>
where
    ID: PartialEq + Copy,
    R: PartialOrd + Copy,
{
    fn get_mut(&mut self, id: ID) -> Option<&mut BNode<ID, R, S>> {
        self.nodes.iter_mut().find(|node| node.id == id)
    }

    fn visit(&self, mut f: impl FnMut(ID, R, &S, usize)) {
        let mut stack = vec![];

        self.visit_node(0, &mut stack);

        for index in stack {
            if let Some(BNode {
                id,
                rank,
                symbol,
                level,
                ..
            }) = self.nodes.get(index)
            {
                f(*id, *rank, symbol, *level);
            }
        }
    }

    fn visit_node(&self, index: usize, stack: &mut Vec<usize>) {
        if let Some(current_node) = self.nodes.get(index) {
            if let Some(left_index) = current_node.left {
                self.visit_node(left_index, stack);
            }
            stack.push(index);
            if let Some(right_index) = current_node.right {
                self.visit_node(right_index, stack);
            }
        }
    }

    fn add(&mut self, (id, rank, symbol): (ID, R, S)) {
        self.nodes.push(BNode {
            id,
            rank,
            symbol,
            left: None,
            right: None,
            level: 0,
        });
        let index = self.nodes.len() - 1;
        if index == 0 {
            self.levels.insert(0, 1);
            return;
        }

        let mut level = 0;
        let mut current_node = self.nodes.get_mut(0).unwrap();
        loop {
            level += 1;
            if rank > current_node.rank {
                if let Some(current_index) = current_node.right {
                    current_node = self.nodes.get_mut(current_index).unwrap();
                } else {
                    current_node.right = Some(index);
                    break;
                }
            } else if rank < current_node.rank {
                if let Some(current_index) = current_node.left {
                    current_node = self.nodes.get_mut(current_index).unwrap();
                } else {
                    current_node.left = Some(index);
                    break;
                }
            } else {
                unreachable!("rank equals!");
            }
        }

        self.nodes.get_mut(index).unwrap().level = level;
        *self.levels.entry(level).or_default() += 1;
    }
}

impl<ID, R, S> Extend<(ID, R, S)> for BTree<ID, R, S>
where
    ID: PartialEq + Copy,
    R: PartialOrd + Copy,
{
    fn extend<II: IntoIterator<Item = (ID, R, S)>>(&mut self, ii: II) {
        for node in ii {
            self.add(node);
        }
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> String {
    let (left_tree, right_tree) = data
        .lines()
        .map(|line| match parse_line(line) {
            Command::Add(id, (l_r, l_s), (r_r, r_s)) => ((id, l_r, l_s), (id, r_r, r_s)),
            Command::Swap(_) => unreachable!("invalid input"),
        })
        .collect::<(BTree<_, _, _>, BTree<_, _, _>)>();

    let left_level = *left_tree
        .levels
        .iter()
        .max_by_key(|(_, v)| *v)
        .expect("cannot find left level")
        .0;
    let right_level = *right_tree
        .levels
        .iter()
        .max_by_key(|(_, v)| *v)
        .expect("cannot find left level")
        .0;

    let mut result = String::with_capacity(1024);
    left_tree.visit(|_, _, symbol, level| {
        if level == left_level {
            result.push(*symbol);
        }
    });
    right_tree.visit(|_, _, symbol, level| {
        if level == right_level {
            result.push(*symbol);
        }
    });

    result
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> String {
    let (mut left_tree, mut right_tree) = (BTree::default(), BTree::default());
    for command in data.lines().map(parse_line) {
        match command {
            Command::Add(id, (l_r, l_s), (r_r, r_s)) => {
                left_tree.add((id, l_r, l_s));
                right_tree.add((id, r_r, r_s));
            }
            Command::Swap(id) => {
                let left_node = left_tree.get_mut(id).expect("cannot find left node");
                let right_node = right_tree.get_mut(id).expect("cannot find right node");
                std::mem::swap(&mut left_node.rank, &mut right_node.rank);
                std::mem::swap(&mut left_node.symbol, &mut right_node.symbol);
            }
        }
    }

    let left_level = *left_tree
        .levels
        .iter()
        .max_by_key(|(_, v)| *v)
        .expect("cannot find left level")
        .0;
    let right_level = *right_tree
        .levels
        .iter()
        .max_by_key(|(_, v)| *v)
        .expect("cannot find left level")
        .0;

    let mut result = String::with_capacity(1024);
    left_tree.visit(|_, _, symbol, level| {
        if level == left_level {
            result.push(*symbol);
        }
    });
    right_tree.visit(|_, _, symbol, level| {
        if level == right_level {
            result.push(*symbol);
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let data = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";

        assert_eq!(part_1(data), "CFGNLK");
    }

    #[test]
    fn test_part_1_2() {
        let data = r"ADD id=1 left=[160,E] right=[175,S]
ADD id=2 left=[140,W] right=[224,D]
ADD id=3 left=[122,U] right=[203,F]
ADD id=4 left=[204,N] right=[114,G]
ADD id=5 left=[136,V] right=[256,H]
ADD id=6 left=[147,G] right=[192,O]
ADD id=7 left=[232,I] right=[154,K]
ADD id=8 left=[118,E] right=[125,Y]
ADD id=9 left=[102,A] right=[210,D]
ADD id=10 left=[183,Q] right=[254,E]
ADD id=11 left=[146,E] right=[148,C]
ADD id=12 left=[173,Y] right=[299,S]
ADD id=13 left=[190,B] right=[277,B]
ADD id=14 left=[124,T] right=[142,N]
ADD id=15 left=[153,R] right=[133,M]
ADD id=16 left=[252,D] right=[276,M]
ADD id=17 left=[258,I] right=[245,P]
ADD id=18 left=[117,O] right=[283,!]
ADD id=19 left=[212,O] right=[127,R]
ADD id=20 left=[278,A] right=[169,C]";

        assert_eq!(part_1(data), "EVERYBODYCODES");
    }

    #[test]
    fn test_part_2() {
        let data = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";

        assert_eq!(part_2(data), "MGFLNK");
    }
}
