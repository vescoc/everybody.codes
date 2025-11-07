#![cfg_attr(feature = "simd", feature(portable_simd))]

use std::{fmt, ops};

pub mod simple;

#[cfg(feature = "simd")]
pub mod simd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex {
    x: i64,
    y: i64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Complex {
    /// # Errors
    pub fn parse(data: &str) -> Result<Self, &'static str> {
        if let Some((x, y)) = data.split_once(',') {
            let mut x = x.chars();
            x.next();
            x.next();
            x.next();
            let x = x.as_str().parse().map_err(|_| "invalid x")?;

            let (y, _) = y.split_once(']').ok_or("invalid y")?;
            let y = y.parse().map_err(|_| "invalid y")?;

            Ok(Complex { x, y })
        } else {
            Err("invalid format")
        }
    }
}

impl From<[i64; 2]> for Complex {
    fn from([x, y]: [i64; 2]) -> Self {
        Self { x, y }
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, Complex { x: x2, y: y2 }: Self) -> Self::Output {
        Complex {
            x: self.x * x2 - self.y * y2,
            y: self.x * y2 + self.y * x2,
        }
    }
}

impl ops::Div for Complex {
    type Output = Self;

    fn div(self, Complex { x: x2, y: y2 }: Self) -> Self::Output {
        Complex {
            x: self.x / x2,
            y: self.y / y2,
        }
    }
}

impl ops::Add for Complex {
    type Output = Self;

    fn add(self, Complex { x: x2, y: y2 }: Self) -> Self::Output {
        Complex {
            x: self.x + x2,
            y: self.y + y2,
        }
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> Complex {
    let a = Complex::parse(data).expect("invalid data");

    let mut r = Complex::from([0, 0]);
    for _ in 0..3 {
        r = r * r;
        r = r / [10, 10].into();
        r = r + a;
    }

    r
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let a = Complex::parse(data).expect("invalid data");

    #[cfg(not(feature = "simd"))]
    let r = simple::mandelbrot::<1_000, 10>(a);

    #[cfg(feature = "simd")]
    let r = simd::mandelbrot::<2, 1_000, 10>(a);

    r
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let a = Complex::parse(data).expect("invalid data");

    #[cfg(not(feature = "simd"))]
    let r = simple::mandelbrot::<1_000, 1>(a);

    #[cfg(feature = "simd")]
    let r = simd::mandelbrot::<2, 1_000, 1>(a);

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("A=[25,9]"), Complex::from([357, 862]));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("A=[35300,-64910]"), 4076);
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_3() {
        assert_eq!(part_3("A=[35300,-64910]"), 406954);
    }
}
