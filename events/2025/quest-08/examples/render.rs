use clap::Parser;

use itertools::Itertools;

use imageproc::{
    image,
    drawing,
    rect::Rect,
};

use std::fs;

/// Render input data
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input data to render
    #[arg(short, long)]
    input: String,

    /// File to store image
    #[arg(short, long)]
    output: String,

    /// Dimension
    #[arg(short, long, default_value_t = 2048)]
    size: usize,

    /// Margin
    #[arg(short, long, default_value_t = 24)]
    margin: usize,

    /// Number of nails
    #[arg(short, long, default_value_t = 256)]
    nails: usize,
}

fn main() {
    let args = Args::parse();

    let size = args.size as f32;
    let margin = args.margin as f32;
    let nails = args.nails as f32;
    
    let (x, y) = (size / 2.0, size / 2.0);
    let delta = 2.0 * std::f32::consts::PI / nails;
    let radius = size / 2.0 - margin;
    
    let black = image::Luma::from([0]);
    let white = image::Luma::from([255]);

    let point = |nail: f32| {
        let rad = nail * delta - std::f32::consts::FRAC_PI_2;
        (
            x + radius * rad.cos(),
            y + radius * rad.sin(),
        )
    };
    
    let mut image = image::GrayImage::new(size as _, size as _);

    drawing::draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(size as _, size as _), white);
    drawing::draw_hollow_circle_mut(&mut image, ((size / 2.0) as _, (size / 2.0) as _), radius as _, black);    
    for (start, end) in fs::read_to_string(args.input)
        .expect("cannot open input")
        .split(',')
        .map(|nail| nail.parse::<f32>().expect("invalid nail") - 1.0)
        .tuple_windows()
        .map(|(start, end)| (start.min(end), start.max(end)))
    {
        drawing::draw_line_segment_mut(&mut image, point(start), point(end), black);
    }

    image.save(args.output).unwrap();
}
