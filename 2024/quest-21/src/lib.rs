fn multiply(a: &[usize], indexes: &[usize]) -> Vec<usize> {
    a.iter().map(|&idx| indexes[idx]).collect::<Vec<usize>>()
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> String {
    let data = unsafe { std::str::from_utf8_unchecked(data) };

    let mut steps = data.chars().take(4).fold(0, |acc, value| {
        acc * 10 + (u32::from(value) - u32::from('0')) as usize
    });

    let operations = data
        .chars()
        .skip(5)
        .take_while(|&c| c != '\n')
        .collect::<Vec<char>>();

    let data = data
        .lines()
        .skip(2)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = data[0].len();
    let height = data.len();

    let mut y = (0..width * height).collect::<Vec<_>>();
    let mut x = y.clone();
    for ((r, c), operation) in (0..(width - 2) * (height - 2))
        .map(|i| (1 + i / (width - 2), 1 + i % (width - 2)))
        .zip(operations.iter().cycle())
    {
        match operation {
            'L' => {
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
            'R' => {
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
        .map(|idx| data[idx / width][idx % width])
        .collect::<Vec<_>>();

    message
        .chunks_exact(width)
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect::<String>()
}
