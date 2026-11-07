use event_2026_10 as event;

#[expect(clippy::unreadable_literal, reason = "spec")]
fn main() {
    println!("part 1: {}", event::part_1(include_str!("../data/part_1")));
    println!(
        "part 2: {}",
        event::part_2::<10000>(include_str!("../data/part_2"))
    );
    println!(
        "part 3: {}",
        event::part_3::<1000000000000>(include_str!("../data/part_3"))
    );
}
