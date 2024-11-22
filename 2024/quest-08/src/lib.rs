#[must_use]
fn parse(data: &[u8]) -> u64 {
    data.iter().fold(0, |acc, value| {
        acc * 10 + u64::from(*value) - u64::from(b'0')
    })
}

#[must_use]
pub fn part_1(data: &[u8]) -> u64 {
    let blocks = parse(data);

    let mut last = 0;
    let mut sum = 0;
    let mut idx = 0;
    for i in (0..).map(|i| i * 2 + 1) {
        sum += i;
        if blocks > last && blocks <= sum {
            idx = i;
            break;
        }
        last = sum;
    }

    (sum - blocks) * idx
}

#[must_use]
pub fn part_2<const ACOLYTES: u64, const MARBLE: u64>(data: &[u8]) -> u64 {
    let priests = parse(data);

    let mut thickness = 1;
    let mut sum = 1;
    let mut width = 1;
    loop {
        thickness = thickness * priests % ACOLYTES;
        width += 2;
        sum += width * thickness;
        if sum >= MARBLE {
            break;
        }
    }

    (sum - MARBLE) * width
}

#[must_use]
pub fn part_3<const ACOLYTES: u64, const PLATINUM: u64>(data: &[u8]) -> u64 {
    let priests = parse(data);

    let mut thickness: u64 = 1;
    
    let mut columns = Vec::with_capacity(4096);
    columns.push(1);
    
    let mut width = 1;
    loop {
        width += 2;

        thickness = (thickness * priests) % ACOLYTES + ACOLYTES;

        for column in &mut columns {
            *column += thickness;
        }
        columns.push(thickness);

        let a = width * priests;

        let mut total = columns[0] - (a * columns[0]) % ACOLYTES + columns[columns.len() - 1] * 2;
        for height in columns.iter().skip(1).take(columns.len() - 2) {
            total += (height - (a * height) % ACOLYTES) * 2;
        }

        if total > PLATINUM {
            return total - PLATINUM;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(21, part_1(br"13"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(27, part_2::<5, 50>(br"3"));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(2, part_3::<5, 160>(br"2"));
    }
}
