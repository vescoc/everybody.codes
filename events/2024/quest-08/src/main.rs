#![allow(clippy::unreadable_literal)]

use event_2024_08 as event;

fn main() {
    println!(
        "part 1: {}",
        event::part_1(include_bytes!("../data/part_1"))
    );
    println!(
        "part 2: {}",
        event::part_2::<1111, 20240000>(include_bytes!("../data/part_2"))
    );
    println!(
        "part 3: {}",
        event::part_3::<10, 202400000>(include_bytes!("../data/part_3"))
    );
}
