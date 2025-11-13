use event_2025_08 as event;

fn main() {
    println!(
        "part 1: {}",
        event::part_1::<32>(include_str!("../data/part_1"))
    );
    println!(
        "part 2: {}",
        event::part_2::<256>(include_str!("../data/part_2"))
    );
    println!(
        "part 3: {}",
        event::part_3::<256>(include_str!("../data/part_3"))
    );
}
