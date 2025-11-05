use rayon::prelude::*;

use crate::Complex;

#[allow(clippy::cast_possible_wrap, unused)]
#[must_use]
pub fn mandelbrot<const SIZE: i64, const STEP: usize>(a: Complex) -> usize {
    (0..=const { SIZE / STEP as i64 })
        .into_par_iter()
        .flat_map_iter(|x| {
            (0..=const { SIZE / STEP as i64 }).filter(move |y| {
                let current = a + [x * STEP as i64, y * STEP as i64].into();
                let mut r = Complex::from([0, 0]);

                let mut i = 0;
                loop {
                    if i == 100 {
                        break true;
                    }

                    i += 1;

                    r = r * r;
                    r = r / [100_000, 100_000].into();
                    r = r + current;
                    if !(-1_000_000..=1_000_000).contains(&r.x)
                        || !(-1_000_000..=1_000_000).contains(&r.y)
                    {
                        break false;
                    }
                }
            })
        })
        .count()
}
