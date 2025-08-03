#![allow(clippy::unreadable_literal)]

use quest_2024_08 as quest;

fn main() {
    println!(
        "part 1: {}",
        quest::part_1(include_bytes!("../data/part_1"))
    );
    println!(
        "part 2: {}",
        quest::part_2::<1111, 20240000>(include_bytes!("../data/part_2"))
    );
    println!(
        "part 3: {}",
        quest::part_3::<10, 202400000>(include_bytes!("../data/part_3"))
    );
}
