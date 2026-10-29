use rayon::prelude::*;

use super::parse_iso;

use z3::ast::Int;
use z3::{SatResult, Solver};

/// # Panics
#[must_use]
#[expect(clippy::cast_possible_truncation, reason = "numbers are small")]
pub fn part_3(data: &str) -> u64 {
    data.par_lines()
        .filter_map(|line| {
            let line = line
                .strip_prefix("ISO2-")
                .expect("'ISO2-' prefix not found");
            let (iso_standard, votes) = line.split_once(' ').expect("invalid line");
            let candidate = parse_iso(
                iso_standard,
                votes
                    .as_bytes()
                    .iter()
                    .map(|digit| u64::from(digit - b'0'))
                    .rev(),
            );

            let mut target = 0;
            let solver = Solver::new();
            let mut equals_to_candidate = Int::from_u64(0);
            let mut less_than_target = Int::from_u64(0);
            for (e, (digit, base)) in votes
                .as_bytes()
                .iter()
                .map(|digit| u64::from(digit - b'0'))
                .rev()
                .zip(
                    iso_standard
                        .as_bytes()
                        .iter()
                        .map(|digit| u64::from(digit - b'0'))
                        .rev()
                        .cycle(),
                )
                .enumerate()
            {
                let e = e as u32;

                target += digit * 10u64.pow(e);

                let x = Int::new_const(e);
                solver.assert(x.ge(0));
                solver.assert(x.lt(10));

                equals_to_candidate += &x * base.pow(e);
                less_than_target += &x * 10u64.pow(e);
            }

            solver.assert(equals_to_candidate.eq(candidate));
            solver.assert(less_than_target.lt(target));

            if SatResult::Unsat == solver.check() {
                Some(candidate)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"ISO2-3 23
ISO2-61 149
ISO2-978 99
ISO2-5887 5923
ISO2-76988 123749
ISO2-435657 929977
ISO2-3949 870737"
            ),
            50987,
        );
    }
}
