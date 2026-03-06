#![no_std]

type Socket<'a> = (&'a str, &'a str);

struct Node<'a> {
    id: u64,
    #[allow(dead_code)]
    plug: Socket<'a>,
    left: Socket<'a>,
    right: Socket<'a>,
    left_index: usize,
    left_strong: bool,
    right_index: usize,
    right_strong: bool,
}

impl Node<'_> {}

struct Nodes<'a, const SIZE: usize> {
    nodes: [core::mem::MaybeUninit<Node<'a>>; SIZE],
    next: usize,
}

impl<'a, const SIZE: usize> Nodes<'a, SIZE> {
    const fn new() -> Self {
        Self {
            nodes: [const { core::mem::MaybeUninit::uninit() }; SIZE],
            next: 0,
        }
    }

    fn insert(
        &mut self,
        compatible_bond: impl Fn(Socket<'a>, Socket<'a>) -> bool,
        id: u64,
        plug: Socket<'a>,
        left: Socket<'a>,
        right: Socket<'a>,
    ) {
        self.nodes[self.next].write(Node {
            id,
            plug,
            left,
            right,
            left_index: 0,
            left_strong: false,
            right_index: 0,
            right_strong: false,
        });

        if self.next > 0 {
            // SAFETY: root exists
            if !unsafe { self.unsafe_insert(&compatible_bond, 0, plug) } {
                unreachable!("cannot insert {id}");
            }
        }

        self.next += 1;
    }

    fn insert_break(&mut self, id: u64, plug: Socket<'a>, left: Socket<'a>, right: Socket<'a>) {
        self.nodes[self.next].write(Node {
            id,
            plug,
            left,
            right,
            left_index: 0,
            left_strong: false,
            right_index: 0,
            right_strong: false,
        });

        if self.next > 0 {
            let mut new = self.next;
            let mut plug = plug;
            loop {
                // SAFETY: root exists
                let Some((n, p)) = (unsafe { self.unsafe_insert_break(0, new, plug) }) else {
                    break;
                };

                assert!(n != new && p != plug);

                new = n;
                plug = p;
            }
        }

        self.next += 1;
    }

    fn visit(&self, mut visitor: impl FnMut(&Node<'a>)) {
        if self.next > 0 {
            // SAFETY: root exists
            unsafe {
                self.unsafe_visit(0, &mut visitor);
            }
        }
    }

    /// # Safety: internal function
    unsafe fn unsafe_insert(
        &mut self,
        compatible_bond: &impl Fn(Socket<'a>, Socket<'a>) -> bool,
        current: usize,
        plug: Socket<'a>,
    ) -> bool {
        // SAFETY: Internal function
        unsafe {
            {
                let node = self.nodes[current].assume_init_mut();
                let left_index = node.left_index;
                if left_index == 0 {
                    if compatible_bond(plug, node.left) {
                        node.left_index = self.next;
                        return true;
                    }
                } else if self.unsafe_insert(compatible_bond, left_index, plug) {
                    return true;
                }
            }

            {
                let node = self.nodes[current].assume_init_mut();
                let right_index = node.right_index;
                if right_index == 0 {
                    if compatible_bond(plug, node.right) {
                        node.right_index = self.next;
                        return true;
                    }
                } else if self.unsafe_insert(compatible_bond, right_index, plug) {
                    return true;
                }
            }
        }

        false
    }

    /// # Safety: internal function
    unsafe fn unsafe_insert_break(
        &mut self,
        current: usize,
        mut new: usize,
        mut plug: Socket<'a>,
    ) -> Option<(usize, Socket<'a>)> {
        // SAFETY: Internal function
        unsafe {
            {
                let node = self.nodes[current].assume_init_mut();
                let left_index = node.left_index;
                let left_strong = node.left_strong;
                if left_index == 0 {
                    if weak_bond(plug, node.left) {
                        node.left_index = new;
                        node.left_strong = strong_bond(plug, node.left);
                        return None;
                    }
                } else if !left_strong && strong_bond(plug, node.left) {
                    node.left_index = new;
                    node.left_strong = true;

                    new = left_index;
                    plug = self.nodes[new].assume_init_ref().plug;
                } else if let Some((n, p)) = self.unsafe_insert_break(left_index, new, plug) {
                    new = n;
                    plug = p;
                } else {
                    return None;
                }
            }

            {
                let node = self.nodes[current].assume_init_mut();
                let right_index = node.right_index;
                let right_strong = node.right_strong;
                if right_index == 0 {
                    if weak_bond(plug, node.right) {
                        node.right_index = new;
                        node.right_strong = strong_bond(plug, node.right);
                        return None;
                    }
                } else if !right_strong && strong_bond(plug, node.right) {
                    node.right_index = new;
                    node.right_strong = true;

                    new = right_index;
                    plug = self.nodes[new].assume_init_ref().plug;
                } else if let Some((n, p)) = self.unsafe_insert_break(right_index, new, plug) {
                    new = n;
                    plug = p;
                } else {
                    return None;
                }
            }
        }

        Some((new, plug))
    }

    /// # Safety: internal function
    unsafe fn unsafe_visit(&self, current: usize, visitor: &mut impl FnMut(&Node<'a>)) {
        unsafe {
            let node = self.nodes[current].assume_init_ref();
            if node.left_index > 0 {
                self.unsafe_visit(node.left_index, visitor);
            }
            visitor(node);
            if node.right_index > 0 {
                self.unsafe_visit(node.right_index, visitor);
            }
        }
    }
}

fn weak_bond<'a>(plug: Socket<'a>, socket: Socket<'a>) -> bool {
    plug.0 == socket.0 || plug.1 == socket.1
}

fn strong_bond<'a>(plug: Socket<'a>, socket: Socket<'a>) -> bool {
    plug.0 == socket.0 && plug.1 == socket.1
}

fn parse(line: &str) -> (u64, Socket<'_>, Socket<'_>, Socket<'_>, &'_ str) {
    let mut parts = line.split(", ");
    (
        parts
            .next()
            .expect("cannot find id")
            .split_once("id=")
            .expect("invalid id")
            .1
            .parse()
            .expect("invalid id"),
        parts
            .next()
            .expect("cannot find plug")
            .split_once("plug=")
            .expect("invalid plug")
            .1
            .split_once(' ')
            .expect("plug malformed"),
        parts
            .next()
            .expect("cannot find leftSocket")
            .split_once("leftSocket=")
            .expect("invalid leftSocket")
            .1
            .split_once(' ')
            .expect("leftSocket malformed"),
        parts
            .next()
            .expect("cannot find rightSocket")
            .split_once("rightSocket=")
            .expect("invalid rightSocket")
            .1
            .split_once(' ')
            .expect("leftSocket malformed"),
        parts
            .next()
            .expect("cannot find data")
            .split_once("data=")
            .expect("invalid data")
            .1,
    )
}

/// # Panics
#[must_use]
fn solve<'a, const SIZE: usize>(data: &'a str, insert: impl Fn(&mut Nodes<'a, SIZE>, u64, Socket<'a>, Socket<'a>, Socket<'a>)) -> u64 {
    let mut nodes = Nodes::<'a, SIZE>::new();
    for line in data.lines() {
        let (id, plug, left, right, _) = parse(line);

        insert(&mut nodes, id, plug, left, right);
    }

    let mut checksum = 0;
    let mut index = 1;
    nodes.visit(|node| {
        checksum += node.id * index;
        index += 1;
    });

    checksum
}

/// # Panics
#[must_use]
pub fn part_1<'a>(data: &'a str) -> u64 {
    solve::<'a, 32>(
        data,
        |nodes, id, plug, left, right| {
            nodes.insert(strong_bond, id, plug, left, right);
        },
    )
}

/// # Panics
#[must_use]
pub fn part_2<'a>(data: &'a str) -> u64 {
    solve::<'a, 128>(
        data,
        |nodes, id, plug, left, right| {
            nodes.insert(weak_bond, id, plug, left, right);
        },
    )
}

/// # Panics
#[must_use]
pub fn part_3<'a>(data: &'a str) -> u64 {
    solve::<'a, 256>(
        data,
        |nodes, id, plug, left, right| {
            nodes.insert_break(id, plug, left, right);
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"id=1, plug=BLUE HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=BLUE PENTAGON, data=?
id=2, plug=GREEN CIRCLE, leftSocket=BLUE HEXAGON, rightSocket=BLUE CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=BLUE CIRCLE, data=?
id=4, plug=BLUE CIRCLE, leftSocket=RED HEXAGON, rightSocket=BLUE HEXAGON, data=?
id=5, plug=RED HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=RED HEXAGON, data=?";
        assert_eq!(part_1(data), 43);
    }

    #[test]
    fn test_part_2() {
        let data = r"id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?";
        assert_eq!(part_2(data), 50);
    }

    #[test]
    fn test_part_3_1() {
        let data = r"id=1, plug=RED TRIANGLE, leftSocket=RED TRIANGLE, rightSocket=RED TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=RED PENTAGON, leftSocket=GREEN CIRCLE, rightSocket=GREEN CIRCLE, data=?";
        assert_eq!(part_3(data), 38);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"id=1, plug=RED TRIANGLE, leftSocket=BLUE TRIANGLE, rightSocket=GREEN TRIANGLE, data=?
id=2, plug=GREEN TRIANGLE, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=3, plug=BLUE PENTAGON, leftSocket=BLUE CIRCLE, rightSocket=GREEN CIRCLE, data=?
id=4, plug=RED TRIANGLE, leftSocket=BLUE PENTAGON, rightSocket=GREEN PENTAGON, data=?
id=5, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?
id=6, plug=BLUE TRIANGLE, leftSocket=GREEN CIRCLE, rightSocket=RED CIRCLE, data=?";
        assert_eq!(part_3(data), 60);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                "id=1, plug=BLUE HEXAGON, leftSocket=GREEN CIRCLE, rightSocket=BLUE PENTAGON, data=?"
            ),
            (
                1,
                ("BLUE", "HEXAGON"),
                ("GREEN", "CIRCLE"),
                ("BLUE", "PENTAGON"),
                "?"
            ),
        );
    }
}
