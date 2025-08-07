use event_2024_07 as event;

fn main() {
    println!(
        "part 1: {}",
        event::part_1(include_bytes!("../data/part_1"))
    );
    println!(
        "part 2: {}",
        event::part_2(include_bytes!("../data/part_2"), event::ROUND_2_TERRAIN)
    );
    println!(
        "part 3: {}",
        event::part_3(include_bytes!("../data/part_3"), event::ROUND_3_TERRAIN)
    );
}
