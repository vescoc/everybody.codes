use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, hash_map::Entry};
use std::{mem, ops};

use rayon::prelude::*;

trait Set {
    fn is_set(&self, p: (usize, usize)) -> bool;
    fn set(&mut self, p: (usize, usize), value: bool) -> bool;
}

trait Num
where
    Self: Copy,
    Self: ops::Shl<usize, Output = Self>
        + ops::BitAnd<Output = Self>
        + ops::BitOrAssign
        + ops::BitAndAssign
        + ops::Not<Output = Self>,
    Self: PartialEq,
{
    const BITS: usize;
    const ZERO: Self;
    const ONE: Self;
}

impl Num for u128 {
    const BITS: usize = const { mem::size_of::<u128>() * 8 };
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl<const ROWS: usize, T> Set for [[T; 2]; ROWS]
where
    T: Num,
{
    fn is_set(&self, (x, y): (usize, usize)) -> bool {
        let (v, mask) = if x >= T::BITS {
            (&self[y][1], { T::ONE } << (x - T::BITS))
        } else {
            (&self[y][0], { T::ONE } << x)
        };

        *v & mask != T::ZERO
    }

    fn set(&mut self, (x, y): (usize, usize), value: bool) -> bool {
        let (v, mask) = if x >= T::BITS {
            (&mut self[y][1], { T::ONE } << (x - T::BITS))
        } else {
            (&mut self[y][0], { T::ONE } << x)
        };

        let r = *v & mask != T::ZERO;
        if value {
            *v |= mask;
        } else {
            *v &= !mask;
        }
        r
    }
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> usize {
    let map = data.as_bytes();
    let columns = map
        .iter()
        .position(|cell| *cell == b'\n')
        .expect("Invalid data");
    let rows = (map.len() + 1) / (columns + 1);

    let (x_v, y_v) = (columns / 2, rows / 2);
    assert!(map[y_v * (columns + 1) + x_v] == b'@');

    let start = map
        .chunks(columns + 1)
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .take(columns)
                .map(move |(x, tile)| ((x, y), *tile))
        })
        .find_map(|(p, cell)| if cell == b'S' { Some(p) } else { None })
        .expect("Cannot find S");

    let mut lava = [[0u128; 2]; 151];
    lava.set(start, true);
    for dy in 0..rows / 2 {
        lava.set((x_v, y_v + dy), true);
    }

    let lava = &lava;
    (1..rows / 2)
        .into_par_iter()
        .filter_map(|step| {
            let mut lava = *lava;
            for y_c in 0..rows {
                for x_c in 0..columns {
                    let a = x_v.abs_diff(x_c);
                    let b = y_v.abs_diff(y_c);
                    let r_2 = a * a + b * b;

                    let mut r = r_2.isqrt();
                    while r * r < r_2 {
                        r += 1;
                    }

                    if r <= step {
                        lava.set((x_c, y_c), true);
                    }
                }
            }

            let target = (x_v, y_v + step + 1);
            let go_left = (target.0 - 1, target.1);
            let go_right = (target.0 + 1, target.1);

            let limit = 30 * (step + 1);

            let left_cost = dijkstra(&lava, map, (columns, rows), go_left, start, limit)?;
            let right_cost = dijkstra(&lava, map, (columns, rows), go_right, start, limit)?;

            let cost = usize::from(map[target.1 * (columns + 1) + target.0] - b'0')
                + usize::from(map[go_left.1 * (columns + 1) + go_left.0] - b'0')
                + usize::from(map[go_right.1 * (columns + 1) + go_right.0] - b'0')
                + left_cost
                + right_cost;
            if cost >= limit {
                return None;
            }

            Some((cost, step))
        })
        .min_by_key(|(cost, _)| *cost)
        .map(|(cost, step)| cost * step)
        .unwrap()
}

fn dijkstra(
    lava: &impl Set,
    map: &[u8],
    (columns, rows): (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
    limit: usize,
) -> Option<usize> {
    let mut heap = BinaryHeap::with_capacity(1024);
    let mut visited = HashMap::with_capacity(rows * columns);

    heap.push(Reverse((0, start)));
    visited.insert(start, 0);
    while let Some(Reverse((cost, (x, y)))) = heap.pop() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y)) if x < columns && y < rows => {
                    if (x, y) == end {
                        return Some(cost);
                    }

                    if lava.is_set((x, y)) {
                        continue;
                    }

                    let tile = map[y * (columns + 1) + x];
                    let cost = cost + usize::from(tile - b'0');
                    if cost > limit {
                        continue;
                    }

                    match visited.entry((x, y)) {
                        Entry::Vacant(e) => {
                            e.insert_entry(cost);
                            heap.push(Reverse((cost, (x, y))));
                        }
                        Entry::Occupied(mut e) => {
                            let v = e.get_mut();
                            if cost < *v {
                                *v = cost;
                                heap.push(Reverse((cost, (x, y))));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(
                r"2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464"
            ),
            592
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve(
                r"545233443422255434324
5222533434S2322342222
523444354223232542432
553522225435232255242
232343243532432452524
245245322252324442542
252533232225244224355
523533554454232553332
522332223232242523223
524523432425432244432
3532242243@4323422334
542524223994422443222
252343244322522222332
253355425454255523242
344324325233443552555
423523225325255345522
244333345244325322335
242244352245522323422
443332352222535334325
323532222353523253542
553545434425235223552"
            ),
            330
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                r"5441525241225111112253553251553
133522122534119S911411222155114
3445445533355599933443455544333
3345333555434334535435433335533
5353333345335554434535533555354
3533533435355443543433453355553
3553353435335554334453355435433
5435355533533355533535335345335
4353545353545354555534334453353
4454543553533544443353355553453
5334554534533355333355543533454
4433333345445354553533554555533
5554454343455334355445533453453
4435554534445553335434455334353
3533435453433535345355533545555
534433533533535@353533355553345
4453545555435334544453344455554
4353333535535354535353353535355
4345444453554554535355345343354
3534544535533355333333445433555
3535333335335334333534553543535
5433355333553344355555344553435
5355535355535334555435534555344
3355433335553553535334544544333
3554333535553335343555345553535
3554433545353554334554345343343
5533353435533535333355343333555
5355555353355553535354333535355
4344534353535455333455353335333
5444333535533453535335454535553
3534343355355355553543545553345"
            ),
            3180
        );
    }
}
