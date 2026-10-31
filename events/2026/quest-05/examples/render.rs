use std::fs;

use clap::{Parser, ValueEnum};

use imageproc::{drawing::Canvas, image};

use event_2026_05::{calc_land, calc_sand, calc_trees, parse_map};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Land,
    Sand,
    Tree,
}

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

    /// Render land, land + sand, land + sand + trees
    #[arg(value_enum, short, long, default_value_t = Mode::Tree)]
    mode: Mode,
}

#[allow(clippy::cast_possible_truncation)]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let data = fs::read_to_string(&args.input)?;

    let (mut map, (width, height)) = parse_map(&data);
    match args.mode {
        Mode::Land => {
            calc_land(&mut map, (width, height));
        }
        Mode::Sand => {
            calc_land(&mut map, (width, height));
            calc_sand(&mut map, (width, height));
        }
        Mode::Tree => {
            calc_land(&mut map, (width, height));
            calc_sand(&mut map, (width, height));
            calc_trees(&mut map, (width, height));
        }
    }

    let dark_blue = image::Rgb::from([0, 0, 139]);
    let light_blue = image::Rgb::from([0x32, 0x48, 0x70]);
    let sand = image::Rgb::from([236, 217, 11]);
    let land = image::Rgb::from([0, 219, 67]);
    let tree = image::Rgb::from([0, 122, 38]);

    let mut image = image::RgbImage::new(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            let color = match map[y * (width + 1) + x] {
                b'0' => dark_blue,
                b'1' | b'.' => light_blue,
                b'=' => sand,
                b'#' => land,
                b'@' => tree,
                _ => unreachable!(),
            };

            image.draw_pixel(x as u32, y as u32, color);
        }
    }

    image.save(args.output)?;

    Ok(())
}
