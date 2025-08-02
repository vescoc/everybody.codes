#![cfg_attr(feature = "nightly", feature(portable_simd))]

#[cfg(feature = "nightly")]
use std::simd::prelude::*;  

fn parse(data: &[u8]) -> u32 {
    data.iter()
        .fold(0, |acc, &c| acc * 10 + u32::from(c) - u32::from(b'0'))
}

fn parse_nails(data: &[u8]) -> Vec<u32> {
    data.split(|&c| c == b'\n').map(parse).collect::<Vec<_>>()
}

#[cfg(not(feature = "nightly"))]
fn black_hammer(data: &[u8]) -> u32 {
    let nails = parse_nails(data);

    let min = nails.iter().min().unwrap();
    nails.iter().map(|nail| nail - min).sum()
}

#[cfg(feature = "nightly")]
fn black_hammer(data: &[u8]) -> u32 {
    let nails = parse_nails(data);

    let min = *nails.iter().min().unwrap();
    let min_simd = u32x64::splat(min);

    let (prefix, middle, suffix) = nails.as_simd::<64>();

    let mins = |part: &[u32]| { part.iter().map(|nail| nail - min).sum() };

    let result = u32x64::load_or_default(&[mins(prefix), mins(suffix)]);
    let result = middle.iter().fold(result, |acc, value| acc + (value - min_simd));

    result.reduce_sum()
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> u32 {
    black_hammer(data)
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> u32 {
    black_hammer(data)
}

/// # Panics
#[must_use]
pub fn part_3(data: &[u8]) -> u32 {
    #[cfg(not(feature = "nightly"))]
    let result = hammer(data);

    #[cfg(feature = "nightly")]
    let result = simd_hammer::<2>(data);

    result
}

/// # Panics
#[cfg(not(feature = "nightly"))]
#[inline]
fn hammer(data: &[u8]) -> u32 {
    let mut nails = parse_nails(data);
    
    nails.sort_unstable();

    let median = nails[(nails.len() - 1) / 2];
    nails
        .iter()
        .map(|&nail| {
            median.abs_diff(nail)
        })
        .sum()
}

/// # Panics
#[cfg(feature = "nightly")]
#[inline]
fn simd_hammer<const LANES: usize>(data: &[u8]) -> u32
where std::simd::LaneCount<LANES>: std::simd::SupportedLaneCount
{
    let mut nails = parse_nails(data);
    
    nails.sort_unstable();

    let idx = (nails.len() - 1) / 2;

    let median = nails[idx];
    let median_simd = Simd::<u32, LANES>::splat(median);

    let (prefix, middle, suffix) = nails[..idx].as_simd::<LANES>();

    let sums = Simd::<u32, LANES>::load_or_default(&[
        prefix.iter().map(|&nail| median - nail).sum(),
        suffix.iter().map(|&nail| median - nail).sum(),
    ]);

    let sums_low = middle.iter().fold(sums, |acc, nail| acc + (median_simd - nail));
        
    let (prefix, middle, suffix) = nails[idx..].as_simd::<LANES>();

    let sums = Simd::<u32, LANES>::load_or_default(&[
        prefix.iter().map(|&nail| nail - median).sum(),
        suffix.iter().map(|&nail| nail - median).sum(),
    ]);

    let sums_high = middle.iter().fold(sums, |acc, nail| acc + (nail - median_simd));

    (sums_low + sums_high).reduce_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            10,
            part_1(
                br"3
4
7
8"
            )
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            8,
            part_3(
                br"2
4
5
6
8"
            )
        );
    }
}
