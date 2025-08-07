/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> String {
    let mut parts = data.split(|&c| c == b'\n');

    let operations = parts.next().unwrap();

    let mut message = parts.skip(1).map(<[u8]>::to_vec).collect::<Vec<_>>();
    let width = message[0].len();

    let r = 1;
    for (c, operation) in (1..width - 1).zip(operations.iter().cycle()) {
        match operation {
            b'L' => {
                let tmp = message[r - 1][c - 1];
                message[r - 1][c - 1] = message[r - 1][c];
                message[r - 1][c] = message[r - 1][c + 1];
                message[r - 1][c + 1] = message[r][c + 1];
                message[r][c + 1] = message[r + 1][c + 1];
                message[r + 1][c + 1] = message[r + 1][c];
                message[r + 1][c] = message[r + 1][c - 1];
                message[r + 1][c - 1] = message[r][c - 1];
                message[r][c - 1] = tmp;
            }
            b'R' => {
                let tmp = message[r - 1][c - 1];
                message[r - 1][c - 1] = message[r][c - 1];
                message[r][c - 1] = message[r + 1][c - 1];
                message[r + 1][c - 1] = message[r + 1][c];
                message[r + 1][c] = message[r + 1][c + 1];
                message[r + 1][c + 1] = message[r][c + 1];
                message[r][c + 1] = message[r - 1][c + 1];
                message[r - 1][c + 1] = message[r - 1][c];
                message[r - 1][c] = tmp;
            }
            _ => unreachable!(),
        }
    }

    message
        .iter()
        .find_map(|row| {
            if let Some((s, e)) = row
                .iter()
                .position(|&c| c == b'>')
                .and_then(|s| row.iter().position(|&c| c == b'<').map(|e| (s, e)))
            {
                Some(String::from_utf8_lossy(&row[s + 1..e]).into_owned())
            } else {
                None
            }
        })
        .unwrap()
}

fn multiply(a: &[usize], indexes: &[usize]) -> Vec<usize> {
    a.iter().map(|&idx| indexes[idx]).collect::<Vec<usize>>()
}

/// # Panics
#[must_use]
fn solve(data: &[u8], mut steps: usize) -> String {
    let mut newlines = data
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == b'\n' { Some(i) } else { None });

    let operations = &data[0..newlines.next().unwrap()];

    let start = newlines.next().unwrap() + 1;
    let width = newlines.next().unwrap() - start;

    let data = &data[start..];

    let height = (data.len() + 1) / (width + 1);

    let mut y = (0..width * height).collect::<Vec<_>>();
    let mut x = y.clone();
    for ((r, c), operation) in (0..(width - 2) * (height - 2))
        .map(|i| (1 + i / (width - 2), 1 + i % (width - 2)))
        .zip(operations.iter().cycle())
    {
        match operation {
            b'L' => {
                let tmp = x[(r - 1) * width + c - 1];
                x[(r - 1) * width + c - 1] = x[(r - 1) * width + c];
                x[(r - 1) * width + c] = x[(r - 1) * width + c + 1];
                x[(r - 1) * width + c + 1] = x[(r) * width + c + 1];
                x[r * width + c + 1] = x[(r + 1) * width + c + 1];
                x[(r + 1) * width + c + 1] = x[(r + 1) * width + c];
                x[(r + 1) * width + c] = x[(r + 1) * width + c - 1];
                x[(r + 1) * width + c - 1] = x[(r) * width + c - 1];
                x[r * width + c - 1] = tmp;
            }
            b'R' => {
                let tmp = x[(r - 1) * width + c - 1];
                x[(r - 1) * width + c - 1] = x[(r) * width + c - 1];
                x[(r) * width + c - 1] = x[(r + 1) * width + c - 1];
                x[(r + 1) * width + c - 1] = x[(r + 1) * width + c];
                x[(r + 1) * width + c] = x[(r + 1) * width + c + 1];
                x[(r + 1) * width + c + 1] = x[(r) * width + c + 1];
                x[(r) * width + c + 1] = x[(r - 1) * width + c + 1];
                x[(r - 1) * width + c + 1] = x[(r - 1) * width + c];
                x[(r - 1) * width + c] = tmp;
            }
            _ => unreachable!(),
        }
    }

    while steps > 1 {
        if steps % 2 == 1 {
            y = multiply(&x, &y);
            steps -= 1;
        }
        x = multiply(&x, &x);
        steps /= 2;
    }
    let indexes = multiply(&x, &y);

    let message = indexes
        .into_iter()
        .map(|idx| data[idx + idx / width])
        .collect::<Vec<_>>();

    message
        .chunks_exact(width)
        .find_map(|row| {
            if let Some((s, e)) = row
                .iter()
                .position(|&c| c == b'>')
                .and_then(|s| row.iter().position(|&c| c == b'<').map(|e| (s, e)))
            {
                Some(String::from_utf8_lossy(&row[s + 1..e]).into_owned())
            } else {
                None
            }
        })
        .unwrap()
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> String {
    solve(data, 100)
}

/// # Panics
#[allow(clippy::unreadable_literal)]
#[must_use]
pub fn part_3(data: &[u8]) -> String {
    solve(data, 1048576000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "WIN",
            &part_1(
                br"LR

>-IN-
-----
W---<"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "VICTORY",
            &part_2(
                br"RRLL

A.VI..>...T
.CC...<...O
.....EIB.R.
.DHB...YF..
.....F..G..
D.H........"
            )
        );
    }
}
