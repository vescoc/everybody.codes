use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;

use clap::Parser;

use imageproc::{drawing::Canvas, image};

use event_2026_19::Color;

/// Render input data
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input data to render
    #[arg(short, long)]
    input: String,

    /// Image output file
    #[arg(short, long)]
    output: String,
}

#[allow(clippy::cast_possible_truncation)]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let data = fs::read_to_string(&args.input)?;

    let colors = [
        (Color::K, image::Rgb::from([0x00, 0x00, 0x00])),
        (Color::C, image::Rgb::from([0x00, 0xb7, 0xeb])),
        (Color::Y, image::Rgb::from([0xff, 0xff, 0x00])),
        (Color::M, image::Rgb::from([0xff, 0x00, 0xff])),
        (Color::R, image::Rgb::from([0xff, 0x00, 0x00])),
        (Color::G, image::Rgb::from([0x00, 0xff, 0x00])),
        (Color::B, image::Rgb::from([0x00, 0x00, 0xff])),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let image = if let Some((first_grid, second_grid)) = data.split_once("\n\n") {
        let (first_grid, second_grid) = (first_grid.as_bytes(), second_grid.as_bytes());

        let ((width, height), color_map) = color_map(first_grid, second_grid);

        let mut image = image::RgbImage::new(width as u32, height as u32);
        for (y, (row1, row2)) in first_grid
            .chunks(width + 1)
            .zip(second_grid.chunks(width + 1))
            .enumerate()
        {
            for (x, (c1, c2)) in row1
                .iter()
                .take(width)
                .zip(row2.iter().take(width))
                .enumerate()
            {
                image.draw_pixel(x as u32, y as u32, colors[&(color_map[c1] | color_map[c2])]);
            }
        }

        image
    } else {
        let grid = data.as_bytes();
        let width = grid
            .iter()
            .position(|tile| *tile == b'\n')
            .expect("invalid input data");
        let height = (grid.len() + 1) / (width + 1);

        let mut image = image::RgbImage::new(width as u32, height as u32);
        for (y, row) in grid.chunks(width + 1).enumerate() {
            for (x, &c) in row.iter().take(width).take(width).enumerate() {
                image.draw_pixel(x as u32, y as u32, colors[&Color::try_from(c).unwrap()]);
            }
        }

        image
    };

    image.save(args.output)?;

    Ok(())
}

fn color_map(first_grid: &[u8], second_grid: &[u8]) -> ((usize, usize), HashMap<u8, Color>) {
    fn find_max(
        perm: &mut [Color],
        candidate: &mut Vec<Color>,
        first_grid: &[u8],
        second_grid: &[u8],
    ) -> (u64, Vec<Color>) {
        let len = perm.len();
        if len > 0 {
            (0..len)
                .map(|index| {
                    candidate.push(perm[index]);
                    perm[index..].rotate_left(1);

                    let result = find_max(&mut perm[..len - 1], candidate, first_grid, second_grid);

                    perm[index..].rotate_right(1);
                    candidate.pop();

                    result
                })
                .max_by_key(|(k, _)| *k)
                .unwrap()
        } else {
            let mut colors = [0; const { Color::K as usize + 1 }];
            for (c1, c2) in first_grid
                .iter()
                .filter(|c| **c != b'\n')
                .zip(second_grid.iter().filter(|c| **c != b'\n'))
            {
                colors[(Color::try_from(*c1).expect("invalid color")
                    | candidate[(*c2 - b'0' - 1) as usize]) as usize] += 1;
            }

            colors.sort_unstable_by_key(|k| Reverse(*k));

            (colors.iter().take(3).product(), candidate.clone())
        }
    }

    let mut perm = [Color::K, Color::C, Color::M, Color::Y];
    let mut candidate = Vec::with_capacity(4);
    let colors = find_max(&mut perm, &mut candidate, first_grid, second_grid).1;

    let width = first_grid
        .iter()
        .position(|c| *c == b'\n')
        .expect("invalid grid");
    let height = (first_grid.len() + 1) / (width + 1);

    (
        (width, height),
        HashMap::from([
            (b'K', Color::K),
            (b'C', Color::C),
            (b'Y', Color::Y),
            (b'M', Color::M),
            (b'R', Color::R),
            (b'G', Color::G),
            (b'B', Color::B),
            (b'1', colors[0]),
            (b'2', colors[1]),
            (b'3', colors[2]),
            (b'4', colors[3]),
        ]),
    )
}
