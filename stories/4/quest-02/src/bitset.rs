pub struct BitSet([i128; 128]);

impl BitSet {
    pub fn new() -> Self {
        Self([0; 128])
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn insert(&mut self, (mut r, mut c): (i64, i64)) -> bool {
        r = r.rem_euclid(128);
        c = c.rem_euclid(128);

        let result = self.0[r as usize] & (1 << c) == 0;

        self.0[r as usize] |= 1 << c;

        result
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn remove(&mut self, &(mut r, mut c): &(i64, i64)) {
        r = r.rem_euclid(128);
        c = c.rem_euclid(128);

        self.0[r as usize] &= !(1 << c);
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn contains(&self, &(mut r, mut c): &(i64, i64)) -> bool {
        r = r.rem_euclid(128);
        c = c.rem_euclid(128);

        self.0[r as usize] & (1 << c) != 0
    }

    pub fn union(&mut self, set: &BitSet) {
        for (row, other_row) in self.0.iter_mut().zip(set.0.iter()) {
            *row |= other_row;
        }
    }

    pub fn difference(&mut self, set: &BitSet) {
        for (row, other_row) in self.0.iter_mut().zip(set.0.iter()) {
            *row &= !other_row;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|row| *row == 0)
    }
}
