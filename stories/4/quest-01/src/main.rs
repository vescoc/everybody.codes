use story_4_01 as story;

fn main() {
    println!(
        "part 1: {}",
        story::part_1(include_str!("../data/part_1")).expect("Invalid input data")
    );
    println!(
        "part 2: {}",
        story::part_2(include_str!("../data/part_2")).expect("Invalid input data")
    );
    println!(
        "part 3: {}",
        story::part_3(include_str!("../data/part_3")).expect("Invalid input data")
    );
}
