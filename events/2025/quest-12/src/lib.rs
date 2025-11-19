use std::collections::VecDeque;

use rayon::prelude::*;

trait Set {
    fn is_set(&self, coordinate: (usize, usize)) -> bool;
    fn set(&mut self, coordinate: (usize, usize));
}

struct EmptySet;

impl Set for EmptySet {
    fn is_set(&self, _: (usize, usize)) -> bool {
        false
    }
    fn set(&mut self, _: (usize, usize)) {}
}

impl<const SIZE: usize> Set for [u32; SIZE] {
    fn is_set(&self, (row, column): (usize, usize)) -> bool {
        self[row] & (1 << column) != 0
    }

    fn set(&mut self, (row, column): (usize, usize)) {
        self[row] |= 1 << column;
    }
}

impl<const SIZE: usize> Set for [[u128; 2]; SIZE] {
    fn is_set(&self, (row, column): (usize, usize)) -> bool {
        if column >= 128 {
            self[row][1] & (1 << (column - 128)) != 0
        } else {
            self[row][0] & (1 << column) != 0
        }
    }

    fn set(&mut self, (row, column): (usize, usize)) {
        if column >= 128 {
            self[row][1] |= 1 << (column - 128);
        } else {
            self[row][0] |= 1 << column;
        }
    }
}

#[allow(clippy::struct_field_names)]
struct Barrels<'a> {
    barrels: &'a [u8],
    rows: usize,
    columns: usize,
}

impl<'a> Barrels<'a> {
    fn new(barrels: &'a [u8]) -> Self {
        let columns = barrels
            .iter()
            .position(|tile| *tile == b'\n')
            .expect("invalid input data");
        let rows = (barrels.len() + 1) / (columns + 1);

        Self {
            barrels,
            rows,
            columns,
        }
    }

    fn bfs(&self, visited: &mut impl Set, start: &[(usize, usize)]) -> usize {
        let Self {
            barrels,
            rows,
            columns,
        } = *self;

        let mut count = 0;

        let mut queue = VecDeque::with_capacity(1024);
        for position in start {
            queue.push_back(*position);
        }
        while let Some((row, column)) = queue.pop_front() {
            if visited.is_set((row, column)) {
                continue;
            }
            visited.set((row, column));

            count += 1;

            let current_barrel = barrels[row * (columns + 1) + column];
            for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                match (row.checked_add_signed(dr), column.checked_add_signed(dc)) {
                    (Some(new_row), Some(new_column))
                        if new_row < rows
                            && new_column < columns
                            && !visited.is_set((new_row, new_column))
                            && current_barrel >= barrels[new_row * (columns + 1) + new_column] =>
                    {
                        queue.push_back((new_row, new_column));
                    }
                    _ => {}
                }
            }
        }

        count
    }

    fn must_evaluate(&self, visited: &impl Set, (row, column): (usize, usize)) -> bool {
        let Self {
            barrels,
            rows,
            columns,
        } = *self;

        let current_barrel = barrels[row * (columns + 1) + column];

        ![(-1, 0), (0, -1)]
	    .iter()
	    .all(|&(dr, dc)| {
		matches!(
		    (row.checked_add_signed(dr), column.checked_add_signed(dc)),
		    (Some(new_row), Some(new_column)) if new_row < rows && new_column < columns && !visited.is_set((new_row, new_column)) && current_barrel < barrels[new_row * (columns + 1) + new_column],
		)
	    })
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let barrels = Barrels::new(data.as_bytes());

    let mut visited = [0u32; 32];
    barrels.bfs(&mut visited, &[(0, 0)])
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let barrels = Barrels::new(data.as_bytes());

    let mut visited = [[0u128; 2]; 72];
    barrels.bfs(
        &mut visited,
        &[(0, 0), (barrels.rows - 1, barrels.columns - 1)],
    )
}

/// # Panics
#[must_use]
pub fn part_3_par(data: &str) -> usize {
    const MAX_ROWS: usize = 105;

    let barrels = &Barrels::new(data.as_bytes());

    let (_, fire_1, visited_1) = (0..barrels.rows)
        .into_par_iter()
        .flat_map(|row| {
            (0..barrels.columns)
                .into_par_iter()
                .filter_map(move |column| {
                    if barrels.must_evaluate(&EmptySet, (row, column)) {
                        let mut set = [[0u128; 2]; MAX_ROWS];
                        Some((
                            barrels.bfs(&mut set, &[(row, column)]),
                            (row, column),
                            Box::new(set),
                        ))
                    } else {
                        None
                    }
                })
        })
        .max_by_key(|(count, _, _)| *count)
        .unwrap();

    let visited_1 = &visited_1;
    let (_, fire_2, visited_2) = (0..barrels.rows)
        .into_par_iter()
        .flat_map(|row| {
            (0..barrels.columns)
                .into_par_iter()
                .filter_map(move |column| {
                    if barrels.must_evaluate(visited_1.as_ref(), (row, column)) {
                        let mut set = **visited_1;
                        Some((
                            barrels.bfs(&mut set, &[(row, column)]),
                            (row, column),
                            Box::new(set),
                        ))
                    } else {
                        None
                    }
                })
        })
        .max_by_key(|(count, _, _)| *count)
        .unwrap();

    let visited_2 = &visited_2;
    let (_, fire_3) = (0..barrels.rows)
        .into_par_iter()
        .flat_map(|row| {
            (0..barrels.columns)
                .into_par_iter()
                .filter_map(move |column| {
                    if barrels.must_evaluate(visited_2.as_ref(), (row, column)) {
                        let mut set = **visited_2;
                        Some((barrels.bfs(&mut set, &[(row, column)]), (row, column)))
                    } else {
                        None
                    }
                })
        })
        .max_by_key(|(count, _)| *count)
        .unwrap();

    let mut visited = [[0u128; 2]; MAX_ROWS];
    barrels.bfs(&mut visited, &[fire_1, fire_2, fire_3])
}

/// # Panics
#[must_use]
pub fn part_3_nopar(data: &str) -> usize {
    use std::rc::Rc;

    let barrels = Barrels::new(data.as_bytes());

    let mut visited_1 = None;
    let mut fire_1 = None;
    {
        let mut list: Vec<Rc<Box<[[u128; 2]; 128]>>> = vec![];
        let mut max = usize::MIN;
        for row in 0..barrels.rows {
            for column in 0..barrels.columns {
                if list.iter().all(|set| !set.is_set((row, column))) {
                    let mut set = [[0u128; 2]; 128];
                    let count = barrels.bfs(&mut set, &[(row, column)]);
                    if count > max {
                        max = count;
                        fire_1.replace((row, column));

                        let set = Rc::new(Box::new(set));
                        list.push(Rc::clone(&set));

                        visited_1.replace(set);
                    }
                }
            }
        }
    }
    let visited_1 = visited_1.unwrap();
    let fire_1 = fire_1.unwrap();

    let mut visited_2 = None;
    let mut fire_2 = None;
    {
        let mut list = vec![Rc::clone(&visited_1)];
        let mut max = usize::MIN;
        for row in 0..barrels.rows {
            for column in 0..barrels.columns {
                if list.iter().all(|set| !set.is_set((row, column))) {
                    let mut set = **visited_1;
                    let count = barrels.bfs(&mut set, &[(row, column)]);
                    if count > max {
                        max = count;
                        fire_2.replace((row, column));

                        let set = Rc::new(Box::new(set));
                        list.push(Rc::clone(&set));

                        visited_2.replace(set);
                    }
                }
            }
        }
    }
    let visited_2 = visited_2.unwrap();
    let fire_2 = fire_2.unwrap();

    let mut fire_3 = None;
    {
        let mut list = vec![Rc::clone(&visited_2)];
        let mut max = usize::MIN;
        for row in 0..barrels.rows {
            for column in 0..barrels.columns {
                if list.iter().all(|set| !set.is_set((row, column))) {
                    let mut set = **visited_2;
                    let count = barrels.bfs(&mut set, &[(row, column)]);
                    if count > max {
                        max = count;
                        fire_3.replace((row, column));

                        let set = Rc::new(Box::new(set));
                        list.push(set);
                    }
                }
            }
        }
    }
    let fire_3 = fire_3.unwrap();

    let mut visited = [[0u128; 2]; 128];
    barrels.bfs(&mut visited, &[fire_1, fire_2, fire_3])
}

pub use part_3_par as part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"989611
857782
746543
766789"
            ),
            16,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"9589233445
9679121695
8469121876
8352919876
7342914327
7234193437
6789193538
6781219648
5691219769
5443329859"
            ),
            58
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"5411
3362
5235
3112"
            ),
            14
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517"
            ),
            136
        );
    }

    #[test]
    fn test_part_alt_3_1() {
        assert_eq!(
            part_3_nopar(
                r"5411
3362
5235
3112"
            ),
            14
        );
    }

    #[test]
    fn test_part_alt_3_2() {
        assert_eq!(
            part_3_nopar(
                r"41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517"
            ),
            136
        );
    }
}
