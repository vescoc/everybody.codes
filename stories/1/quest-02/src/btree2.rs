use std::collections::HashMap;

use crate::{Command, parse_line};

#[derive(Debug, Default)]
struct BTree<ID, R, S> {
    nodes: Vec<BNode<ID, R, S>>,
    left_root: Option<usize>,
    right_root: Option<usize>,
}

#[derive(Debug)]
struct BNode<ID, R, S> {
    id: ID,
    rank: R,
    symbol: S,
    left: Option<usize>,
    right: Option<usize>,
}

impl<ID, R, S> BTree<ID, R, S>
where
    ID: PartialEq + Copy,
    R: PartialOrd + Copy,
{
    fn visit_left(&self, mut f: impl FnMut(ID, R, &S, usize)) {
        let mut stack = vec![];
        if let Some(left_root) = self.left_root {
            self.visit_node(&mut stack, left_root, 0);
        }

        for (index, level) in stack {
            if let Some(BNode {
                id, rank, symbol, ..
            }) = self.nodes.get(index)
            {
                f(*id, *rank, symbol, level);
            }
        }
    }

    fn visit_right(&self, mut f: impl FnMut(ID, R, &S, usize)) {
        let mut stack = vec![];
        if let Some(right_root) = self.right_root {
            self.visit_node(&mut stack, right_root, 0);
        }

        for (index, level) in stack {
            if let Some(BNode {
                id, rank, symbol, ..
            }) = self.nodes.get(index)
            {
                f(*id, *rank, symbol, level);
            }
        }
    }

    fn visit_node(&self, stack: &mut Vec<(usize, usize)>, index: usize, level: usize) {
        if let Some(current_node) = self.nodes.get(index) {
            if let Some(left_index) = current_node.left {
                self.visit_node(stack, left_index, level + 1);
            }
            stack.push((index, level));
            if let Some(right_index) = current_node.right {
                self.visit_node(stack, right_index, level + 1);
            }
        }
    }

    fn add_node(&mut self, current_index: usize, index: usize, rank: R) {
        let mut current_node = self.nodes.get_mut(current_index).unwrap();
        loop {
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
    }

    fn add_left_node(&mut self, (id, rank, symbol): (ID, R, S)) {
        self.nodes.push(BNode {
            id,
            rank,
            symbol,
            left: None,
            right: None,
        });
        let index = self.nodes.len() - 1;

        if let Some(root_index) = self.left_root {
            self.add_node(root_index, index, rank);
        } else {
            self.left_root = Some(index);
        }
    }

    fn add_right_node(&mut self, (id, rank, symbol): (ID, R, S)) {
        self.nodes.push(BNode {
            id,
            rank,
            symbol,
            left: None,
            right: None,
        });
        let index = self.nodes.len() - 1;

        if let Some(root_index) = self.right_root {
            self.add_node(root_index, index, rank);
        } else {
            self.right_root = Some(index);
        }
    }

    fn swap(&mut self, target_id: ID) {
        let mut nodes = self
            .nodes
            .iter()
            .enumerate()
            .filter_map(
                |(index, BNode { id, .. })| if *id == target_id { Some(index) } else { None },
            );
        let first_node = nodes.next().unwrap();
        let second_node = nodes.next().unwrap();

        self.nodes.swap(first_node, second_node);
    }
}

impl<ID, R, S> FromIterator<Command<ID, R, S>> for BTree<ID, R, S>
where
    ID: PartialEq + Copy + Default,
    R: PartialOrd + Copy + Default,
    S: Default,
{
    fn from_iter<II: IntoIterator<Item = Command<ID, R, S>>>(ii: II) -> Self {
        let mut tree = Self::default();
        for command in ii {
            match command {
                Command::Add(id, (l_r, l_s), (r_r, r_s)) => {
                    tree.add_left_node((id, l_r, l_s));
                    tree.add_right_node((id, r_r, r_s));
                }
                Command::Swap(id) => {
                    tree.swap(id);
                }
            }
        }
        tree
    }
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> String {
    fn add_node(nodes: &mut HashMap<usize, String>, symbol: char, level: usize) {
        nodes.entry(level).or_default().push(symbol);
    }

    let tree = data
        .lines()
        .map(parse_line)
        .collect::<BTree<usize, usize, char>>();

    let mut left_nodes = HashMap::with_capacity(1024);
    tree.visit_left(|_, _, symbol, level| add_node(&mut left_nodes, *symbol, level));

    let mut right_nodes = HashMap::with_capacity(1024);
    tree.visit_right(|_, _, symbol, level| add_node(&mut right_nodes, *symbol, level));

    left_nodes
        .iter()
        .max_by_key(|(level, s)| (s.len(), std::cmp::Reverse(*level)))
        .unwrap()
        .1
        .to_string()
        + right_nodes
            .iter()
            .max_by_key(|(level, s)| (s.len(), std::cmp::Reverse(*level)))
            .unwrap()
            .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_3_1() {
        let data = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2";

        assert_eq!(part_3(data), "DJMGL");
    }

    #[test]
    fn test_part_3_2() {
        let data = r"ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5";

        assert_eq!(part_3(data), "DJCGL");
    }
}
