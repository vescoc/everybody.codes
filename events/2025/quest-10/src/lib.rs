mod part_1;
pub use part_1::solve as part_1;

mod part_2;
pub use part_2::solve as part_2;

mod part_3;
pub use part_3::solve as part_3;

const JUMPS: [(isize, isize); 8] = [
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
];
