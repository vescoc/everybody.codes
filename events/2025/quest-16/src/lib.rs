#[allow(clippy::cast_possible_truncation)]
fn spell(data: &str) -> Vec<u64> {
    let mut columns = data
        .split(',')
        .map(|value| value.parse::<u64>().expect("invalid number"))
        .collect::<Vec<_>>();

    let mut spell = Vec::<u64>::with_capacity(columns.len());
    let mut current = 1;
    let mut start = 0;
    while start < columns.len() {
        if columns[start] == 1 {
            spell.push(current);
            let mut i = start;
            while i < columns.len() {
                columns[i] -= 1;
                i += current as usize;
            }
        }
        current += 1;
        start += 1;
    }

    spell
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    data.split(',')
        .map(|value| value.parse::<u64>().expect("invalid number"))
        .map(|value| 90 / value)
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    spell(data).into_iter().product()
}

/// # Panics
#[must_use]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn part_3(data: &str) -> u64 {
    #[allow(clippy::unreadable_literal)]
    const BLOCKS: u64 = 202520252025000;

    let spell = spell(data);

    // sum (columns / spell_i) <= BLOCKS -> columns <= BLOCKS / sum (1 / spell_i)
    let sum = spell.iter().map(|value| 1.0 / *value as f64).sum::<f64>();

    let mut mid = (BLOCKS as f64 / sum) as u64;
    let mut low = mid - 100;
    let mut high = mid + 100;
    while low + 1 < high {
        mid = (low + high) / 2;
        if spell.iter().map(|value| mid / value).sum::<u64>() <= BLOCKS {
            low = mid;
        } else {
            high = mid;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("1,2,3,5,9"), 193);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2"),
            270
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2"),
            94439495762954
        );
    }
}
