use std::simd::{LaneCount, SupportedLaneCount, prelude::*};

use rayon::prelude::*;

use crate::Complex;

#[allow(
    clippy::similar_names,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn mandelbrot_x<const N: usize, const STEP: usize>(
    size: usize,
    a_x: &Simd<i64, N>,
    a_y: &Simd<i64, N>,
) -> usize
where
    LaneCount<N>: SupportedLaneCount,
{
    if size == 0 {
        return 0;
    }

    let lower_threshold = Simd::<i64, N>::splat(-1_000_000);
    let higher_threshold = Simd::<i64, N>::splat(1_000_000);

    let mut r_x = Simd::<i64, N>::splat(0);
    let mut r_y = Simd::<i64, N>::splat(0);

    let mut count = Mask::from_array({
        let mut count = [false; N];
        let mut i = 0;
        loop {
            count[i] = i < size;
            i += 1;
            if i == N {
                break;
            }
        }
        count
    });

    let mut i = 0;
    while i < 100 {
        i += 1;

        let rr_x = r_x * r_x - r_y * r_y;
        let rr_y = r_x * r_y * Simd::splat(2);

        r_x = rr_x / Simd::splat(100_000) + a_x;
        r_y = rr_y / Simd::splat(100_000) + a_y;

        count &= r_x.simd_ge(lower_threshold)
            & r_x.simd_le(higher_threshold)
            & r_y.simd_ge(lower_threshold)
            & r_y.simd_le(higher_threshold);

        if !count.any() {
            return 0;
        }
    }

    -count.to_int().reduce_sum() as usize
}

#[allow(clippy::cast_possible_wrap)]
pub fn mandelbrot<const N: usize, const SIZE: usize, const STEP: usize>(a: Complex) -> usize
where
    LaneCount<N>: SupportedLaneCount,
{
    let a_x = (0..const { SIZE / STEP }.next_multiple_of(N))
        .map(|i| a.x + (i * STEP) as i64)
        .collect::<Vec<_>>();
    let a_y = (0..=const { SIZE / STEP })
        .map(|i| a.y + (i * STEP) as i64)
        .collect::<Vec<_>>();

    a_y.par_iter()
        .map(|y| {
            let a_y = Simd::splat(*y);

            let mut count = 0;
            let mut size = SIZE + 1;
            for x in a_x.chunks(N) {
                let a_x = Simd::from_slice(x);
                count += mandelbrot_x::<N, STEP>(size.min(N), &a_x, &a_y);
                size -= N;
            }
            count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_1_64() {
        let a = Complex::from([35300, -64910]);
        assert_eq!(
            crate::simple::mandelbrot::<1_000, 1>(a),
            mandelbrot::<64, 1_000, 1>(a),
        );
    }

    #[test]
    fn test_same_10_64() {
        let a = Complex::from([35300, -64910]);
        assert_eq!(
            crate::simple::mandelbrot::<1_000, 10>(a),
            mandelbrot::<64, 1_000, 10>(a),
        );
    }

    #[test]
    fn test_same_1_32() {
        let a = Complex::from([35300, -64910]);
        assert_eq!(
            crate::simple::mandelbrot::<1_000, 1>(a),
            mandelbrot::<32, 1_000, 1>(a),
        );
    }

    #[test]
    fn test_same_10_32() {
        let a = Complex::from([35300, -64910]);
        assert_eq!(
            crate::simple::mandelbrot::<1_000, 10>(a),
            mandelbrot::<32, 1_000, 10>(a),
        );
    }

    #[test]
    fn test_same_1_16() {
        let a = Complex::from([35300, -64910]);
        assert_eq!(
            crate::simple::mandelbrot::<1_000, 1>(a),
            mandelbrot::<16, 1_000, 1>(a),
        );
    }

    #[test]
    fn test_same_10_16() {
        let a = Complex::from([35300, -64910]);
        assert_eq!(
            crate::simple::mandelbrot::<1_000, 10>(a),
            mandelbrot::<16, 1_000, 10>(a),
        );
    }
}
