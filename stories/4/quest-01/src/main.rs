use story_4_01 as event;

fn main() {
    println!(
        "part 1: {}",
        event::part_1(include_str!("../data/part_1")).expect("Invalid input data")
    );
    println!(
        "part 2: {}",
        event::part_2(include_str!("../data/part_2")).expect("Invalid input data")
    );
    println!(
        "part 3: {}",
        event::part_3(include_str!("../data/part_3")).expect("Invalid input data")
    );
}
