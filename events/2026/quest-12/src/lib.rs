use std::collections::VecDeque;

fn rect((min_x, min_y): (usize, usize), (max_x, max_y): (usize, usize)) -> usize {
    if min_x > max_x || min_y > max_y {
        0
    } else {
        (max_x - min_x + 1).min(max_y - min_y + 1)
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let mut magma_stacks = data.as_bytes().to_vec();
    let width = magma_stacks
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid input data");
    let height = (magma_stacks.len() + 1) / width;

    let mut sum = 0;
    let (mut x, mut y) = (0, 0);
    loop {
        if magma_stacks[y * (width + 1) + x] == b'#' {
            // find (w, h)
            let mut w = 0;
            while x + w < width && magma_stacks[y * (width + 1) + x + w] == b'#' {
                w += 1;
            }

            let mut h = 0;
            while y + h < height && magma_stacks[(y + h) * (width + 1) + x] == b'#' {
                h += 1;
            }

            // erase magma
            for x in x..x + w {
                for y in y..y + h {
                    magma_stacks[y * (width + 1) + x] = b'o';
                }
            }

            sum += w.min(h);

            // move after
            x += w;
        }

        x += 1;
        if x >= width {
            x = 0;
            y += 1;
        }

        if y >= height {
            break;
        }
    }

    sum
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let mut magma_stacks = data.as_bytes().to_vec();
    let width = magma_stacks
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid input data");
    let height = (magma_stacks.len() + 1) / width;

    let mut sum = 0;
    let (mut x, mut y) = (0, 0);
    loop {
        if magma_stacks[y * (width + 1) + x] == b'#' {
            let mut area = 0;
            let (mut min_x, mut min_y) = (x, y);
            let (mut max_x, mut max_y) = (x, y);
            let mut queue = VecDeque::from([(x, y)]);
            while let Some((x, y)) = queue.pop_front() {
                let cell = &mut magma_stacks[y * (width + 1) + x];
                if *cell == b'#' {
                    *cell = b'o';

                    area += 1;

                    min_x = min_x.min(x);
                    min_y = min_y.min(y); // doesn't change: magmas above are removed

                    max_x = max_x.max(x);
                    max_y = max_y.max(y);

                    if x + 1 < width {
                        queue.push_back((x + 1, y));
                    }
                    if let Some(x) = x.checked_sub(1) {
                        queue.push_back((x, y));
                    }
                    if y + 1 < height {
                        queue.push_back((x, y + 1));
                    }
                }
            }

            // check if it is a simple rectangle
            let (w, h) = (max_x - min_x + 1, max_y - min_y + 1);
            sum += if w * h == area {
                w.min(h)
            } else {
                // handle L shape
                // find inner hollow rectangle
                let (mut inner_min_x, mut inner_min_y) = (max_x, max_y);
                let (mut inner_max_x, mut inner_max_y) = (min_x, min_y);
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        if magma_stacks[y * (width + 1) + x] != b'o' {
                            inner_min_x = inner_min_x.min(x);
                            inner_min_y = inner_min_y.min(y);

                            inner_max_x = inner_max_x.max(x);
                            inner_max_y = inner_max_y.max(y);
                        }
                    }
                }

                let (inner_w, inner_h) =
                    (inner_max_x - inner_min_x + 1, inner_max_y - inner_min_y + 1);

                w.min(h).min((w - inner_w).min(h) + w.min(h - inner_h))
            };
        }

        x += 1;
        if x >= width {
            x = 0;
            y += 1;
        }

        if y >= height {
            break;
        }
    }

    sum
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let mut magma_stacks = data.as_bytes().to_vec();
    let width = magma_stacks
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid input data");
    let height = (magma_stacks.len() + 1) / width;

    let mut sum = 0;
    let (mut x, mut y) = (0, 0);
    loop {
        if magma_stacks[y * (width + 1) + x] == b'#' {
            let mut area = 0;
            let (mut min_x, mut min_y) = (x, y);
            let (mut max_x, mut max_y) = (x, y);
            let mut queue = VecDeque::from([(x, y)]);
            while let Some((x, y)) = queue.pop_front() {
                let cell = &mut magma_stacks[y * (width + 1) + x];
                if *cell == b'#' {
                    *cell = b'o';

                    area += 1;

                    min_x = min_x.min(x);
                    min_y = min_y.min(y); // doesn't change: magmas above are removed

                    max_x = max_x.max(x);
                    max_y = max_y.max(y);

                    if x + 1 < width {
                        queue.push_back((x + 1, y));
                    }
                    if let Some(x) = x.checked_sub(1) {
                        queue.push_back((x, y));
                    }
                    if y + 1 < height {
                        queue.push_back((x, y + 1));
                    }
                }
            }

            // check if it is a simple rectangle
            let (w, h) = (max_x - min_x + 1, max_y - min_y + 1);
            let min = if w * h == area {
                w.min(h)
            } else {
                // find inner hollow rectangle
                let (mut inner_min_x, mut inner_min_y) = (max_x, max_y);
                let (mut inner_max_x, mut inner_max_y) = (min_x, min_y);
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        if magma_stacks[y * (width + 1) + x] != b'o' {
                            inner_min_x = inner_min_x.min(x);
                            inner_min_y = inner_min_y.min(y);

                            inner_max_x = inner_max_x.max(x);
                            inner_max_y = inner_max_y.max(y);
                        }
                    }
                }

                let vertical = rect((min_x, min_y), (max_x, inner_min_y - 1)) // up
                    + rect((min_x, inner_min_y), (inner_min_x - 1, inner_max_y)) // left
                    + rect((inner_max_x + 1, inner_min_y), (max_x, inner_max_y)) // right
                    + rect((min_x, inner_max_y + 1), (max_x, max_y)); // down

                let horizontal = rect((min_x, min_y), (inner_min_x - 1, max_y)) // left
                    + rect((inner_min_x, min_y), (inner_max_x, inner_min_y - 1)) // up
                    + rect((inner_min_x, inner_max_y + 1), (inner_max_x, max_y)) // down
                    + rect((inner_max_x + 1, min_y), (max_x, max_y)); // right

                vertical.min(horizontal)
            };

            sum += min;
        }

        x += 1;
        if x >= width {
            x = 0;
            y += 1;
        }

        if y >= height {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"........#.
.##.......
.##..####.
.##..####.
.....####.
.###......"
            ),
            7,
        );
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(
            part_2(
                r"........#.
.##.......
.##..####.
.#...#....
.....#....
.####....."
            ),
            6,
        );
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(
            part_2(
                r"...............................
.####....#####..#######........
.####....#####..#######........
.####....#####..#######........
.........#####.......##........
.........########....##........
.........########....##........
...............................
.######....#.....#.....###.....
...........#...........###.....
...........#......########.....
...........#......########.....
...........#......########.....
..................########.....
..............................."
            ),
            23,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"........#.
.##.......
.##..####.
.#...#..#.
.....####.
.###......"
            ),
            8,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"..............................
.##########....##.....#######.
.##########.##.##..#..#######.
.####..####.##.##.....#.#####.
.##########.##.#####..#.#####.
.##########.##........#######.
..............................
.##########..################.
.##########..#..............#.
.#######..#..#..............#.
.#######..#..################.
.##########..################.
.............................."
            ),
            29,
        );
    }

    #[test]
    fn test_part_3_3() {
        assert_eq!(
            part_3(
                r"..............................
.############################.
.##.......................###.
.##..################.###.###.
.##..######.........#.###.###.
.##..################.###.###.
.##...................###.###.
.##..####################.###.
.##..####################.###.
.##.......................###.
.############################.
.............................."
            ),
            16,
        );
    }

    #[test]
    fn test_same_part_2_vs_3() {
        let input = include_str!("../data/part_2");

        assert_eq!(part_2(input), part_3(input));
    }
}
