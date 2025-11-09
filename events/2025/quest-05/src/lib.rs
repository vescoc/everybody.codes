trait MinMax<T>: Iterator<Item = T>
where
    Self: Sized,
    T: Copy + PartialOrd,
{
    fn minmax(self) -> Option<(T, T)> {
        let mut min = None;
        let mut max = None;

        for value in self {
            if let Some(ref mut min) = min {
                if value < *min {
                    *min = value;
                }
            } else {
                min.replace(value);
            }

            if let Some(ref mut max) = max {
                if value > *max {
                    *max = value;
                }
            } else {
                max.replace(value);
            }
        }

        Some((min?, max?))
    }
}

impl<T, I> MinMax<T> for I
where
    I: Iterator<Item = T>,
    T: Copy + PartialOrd,
{
}

#[derive(Debug)]
struct Level {
    left: Option<u8>,
    center: u8,
    right: Option<u8>,
}

fn make_level(a: Option<u8>, b: u8, c: Option<u8>) -> u64 {
    (u64::from(a.unwrap_or_default()) * (if b < 10 { 10 } else { 100 }) + u64::from(b))
        * (if let Some(c) = c {
            if c < 10 { 10 } else { 100 }
        } else {
            1
        })
        + u64::from(c.unwrap_or_default())
}

fn fishbone(data: &str) -> (&str, Vec<Level>) {
    let (id, numbers) = data.split_once(':').expect("invalid data");

    let mut fishbone = Vec::with_capacity(64);
    for number in numbers
        .split(',')
        .map(|number| number.parse().expect("invalid number"))
    {
        let mut found = false;
        for Level {
            left,
            center,
            right,
        } in &mut fishbone
        {
            if left.is_none() && number < *center {
                *left = Some(number);
                found = true;
                break;
            } else if right.is_none() && number > *center {
                *right = Some(number);
                found = true;
                break;
            }
        }

        if !found {
            fishbone.push(Level {
                left: None,
                center: number,
                right: None,
            });
        }
    }

    (id, fishbone)
}

/// # Panics
#[must_use]
fn sword(data: &str) -> (u64, u64, Vec<u64>) {
    let (id, fishbone) = fishbone(data);

    let mut levels = Vec::with_capacity(8);
    let mut quality = 0;
    for Level {
        left,
        center,
        right,
    } in fishbone
    {
        levels.push(make_level(left, center, right));

        quality = quality * (if center < 10 { 10 } else { 100 }) + u64::from(center);
    }

    (id.parse().expect("invalid id"), quality, levels)
}

/// # Panics
#[must_use]
fn quality_value(data: &str) -> u64 {
    let (_, fishbone) = fishbone(data);

    fishbone.into_iter().fold(
        0,
        |acc,
         Level {
             left: _,
             center,
             right: _,
         }| { acc * (if center < 10 { 10 } else { 100 }) + u64::from(center) },
    )
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    quality_value(data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    data.lines()
        .map(quality_value)
        .minmax()
        .map(|(min, max)| max - min)
        .expect("invalid data")
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let mut swords = data.lines().map(sword).collect::<Vec<_>>();

    swords.sort_unstable_by(|(id_a, quality_a, levels_a), (id_b, quality_b, levels_b)| {
        quality_b
            .cmp(quality_a)
            .then_with(|| levels_b.cmp(levels_a))
            .then_with(|| id_b.cmp(id_a))
    });

    swords
        .into_iter()
        .enumerate()
        .map(|(i, (id, ..))| (i + 1) as u64 * id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_1() {
        assert_eq!(part_1("58:5,3,7,8,9,10,4,5,7,8,8"), 581078);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5"
            ),
            77053,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7"
            ),
            260,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"1:7,1,9,1,6,9,8,3,7,2
2:7,1,9,1,6,9,8,3,7,2"
            ),
            4,
        );
    }
}
