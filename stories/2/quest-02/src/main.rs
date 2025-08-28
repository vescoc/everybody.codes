use story_2_02 as event;

fn main() {
    println!("part 1: {}", event::part_1(include_str!("../data/part_1")));
    println!(
        "part 2: {}",
        event::part_2::<100>(include_str!("../data/part_2"))
    );
    println!("part 3: {}", event::part_3(include_str!("../data/part_3")));
}
