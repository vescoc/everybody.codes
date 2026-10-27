pub mod calc;
pub mod lookup;

pub use calc::part_1;
pub use calc::part_2;
pub use calc::part_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/part_1");

        assert_eq!(lookup::part_1(data), calc::part_1(data));
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/part_2");

        assert_eq!(lookup::part_2(data), calc::part_2(data));
    }

    #[test]
    fn test_part_3() {
        let data = include_str!("../data/part_3");

        assert_eq!(lookup::part_3(data), calc::part_3(data));
    }
}
