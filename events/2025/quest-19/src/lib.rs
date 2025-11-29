use std::collections::HashMap;

type Coord = (i64, i64);

fn flaps((from_x, from_y): Coord, (to_x, to_y): Coord) -> Option<i64> {
    let distance = to_x - from_x;
    let high = to_y - from_y;

    if (to_x + to_y) % 2 != 0 {
        return None;
    }

    if high.abs() > distance {
        return None;
    }

    Some(i64::midpoint(to_y - from_y, distance))
}

type Cache = HashMap<(Coord, *const Vec<(i64, i64, i64)>), Option<i64>>;
fn min_flaps(memoize: &mut Cache, data: &[Vec<(i64, i64, i64)>], from: Coord) -> Option<i64> {
    if let Some(flaps) = memoize.get(&(from, data.as_ptr())) {
        return *flaps;
    }

    let flaps = if let Some((wall, data)) = data.split_first() {
        let mut min = None;
        'out: for &(to_x, start, segments) in wall {
            for to_y in start..start + segments {
                if let Some(to_flaps) = flaps(from, (to_x, to_y))
                    && let Some(flaps) = min_flaps(memoize, data, (to_x, to_y))
                {
                    min.replace(to_flaps + flaps);
                    break 'out;
                }
            }
        }
        min
    } else {
        Some(0)
    };

    memoize.insert((from, data.as_ptr()), flaps);

    flaps
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> i64 {
    let walls = data
        .lines()
        .map(|line| {
            let mut values = line
                .split(',')
                .map(|value| value.parse::<i64>().expect("invalid number"));
            vec![(
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            )]
        })
        .collect::<Vec<_>>();

    let mut memoize = HashMap::with_capacity(1024);
    min_flaps(&mut memoize, &walls[0..], (0, 0)).unwrap()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> i64 {
    let mut walls = Vec::<Vec<(i64, i64, i64)>>::with_capacity(1024);
    for line in data.lines() {
        let mut values = line
            .split(',')
            .map(|value| value.parse::<i64>().expect("invalid number"));

        let (x, start, segments) = (
            values.next().unwrap(),
            values.next().unwrap(),
            values.next().unwrap(),
        );

        if let Some(wall) = walls.last_mut()
            && wall[0].0 == x
        {
            wall.push((x, start, segments));
            continue;
        }

        walls.push(vec![(x, start, segments)]);
    }

    let mut memoize = HashMap::with_capacity(1024);
    min_flaps(&mut memoize, &walls[0..], (0, 0)).unwrap()
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> i64 {
    part_2(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2"
            ),
            24
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2"
            ),
            22
        );
    }
}
