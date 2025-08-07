use std::collections::HashMap;

use nalgebra::{DMatrix, DVector};

type Rules<const SIZE: usize> = HashMap<[u8; SIZE], Vec<[u8; SIZE]>>;

fn parse_data<const SIZE: usize>(data: &[u8]) -> Rules<SIZE> {
    data.split(|&c| c == b'\n')
        .map(|info| {
            let mut info = info.split(|&c| c == b':');
            let termite = info.next().unwrap().try_into().unwrap();
            let children = info
                .next()
                .unwrap()
                .split(|&c| c == b',')
                .map(|child| child.try_into().unwrap())
                .collect::<Vec<_>>();
            (termite, children)
        })
        .collect::<HashMap<_, _>>()
}

struct Freezed;

#[derive(Default)]
struct Modifiable;

#[derive(Default)]
#[allow(clippy::struct_field_names)]
struct Dictionary<const SIZE: usize, S> {
    next: usize,
    dictionary: HashMap<[u8; SIZE], usize>,
    _s: std::marker::PhantomData<S>,
}

impl<const SIZE: usize> Dictionary<SIZE, Modifiable> {
    fn new() -> Self {
        Self::default()
    }
    
    fn get(&mut self, key: [u8; SIZE]) -> usize {
        *self.dictionary.entry(key).or_insert_with(|| {
            let value = self.next;
            self.next += 1;
            value
        })
    }

    fn freeze(self) -> Dictionary<SIZE, Freezed> {
        Dictionary {
            next: self.next,
            dictionary: self.dictionary,
            _s: std::marker::PhantomData,
        }
    }
}

impl<const SIZE: usize> Dictionary<SIZE, Freezed> {
    fn get(&self, key: [u8; SIZE]) -> usize {
        self.dictionary[&key]
    }
}

fn parse<const SIZE: usize>(data: &[u8]) -> (Dictionary<SIZE, Freezed>, DMatrix<u32>) {
    let mut dictionary = Dictionary::<SIZE, Modifiable>::new();

    let mut data = data
        .split(|&c| c == b'\n')
        .map(|info| {
            let mut info = info.split(|&c| c == b':');
            let termite = dictionary.get(info.next().unwrap().try_into().unwrap());
            let children = info
                .next()
                .unwrap()
                .split(|&c| c == b',')
                .map(|child| dictionary.get(child.try_into().unwrap()))
                .collect::<Vec<_>>();
            (termite, children)
        })
        .collect::<HashMap<_, _>>();

    let mut matrix = DMatrix::zeros(data.len(), data.len());
    for (key, values) in data.drain() {
        for value in values {
            matrix[(value, key)] += 1;
        }
    }

    (dictionary.freeze(), matrix)
}

fn generate<const GEN: usize, const SIZE: usize>(rules: &Rules<SIZE>, origin: [u8; SIZE]) -> u32 {
    (0..)
        .scan(
            {
                let mut generation = HashMap::with_capacity(rules.len());
                generation.insert(origin, 1);
                generation
            },
            |generation, _i| {
                *generation = generation
                    .iter()
                    .flat_map(|(termite, &count)| {
                        rules[termite].iter().map(move |child| (*child, count))
                    })
                    .fold(
                        HashMap::with_capacity(rules.len()),
                        |mut acc, (termite, count)| {
                            *acc.entry(termite).or_default() += count;
                            acc
                        },
                    );
                Some(generation.values().sum())
            },
        )
        .nth(const { GEN - 1 })
        .unwrap()
}

/// # Panics
#[must_use]
pub fn part_1(data: &[u8]) -> u32 {
    let rules = parse_data(data);

    generate::<4, 1>(&rules, [b'A'])
}

/// # Panics
#[must_use]
pub fn part_1_matrix(data: &[u8]) -> u32 {
    let (dictionary, matrix) = parse(data);

    let matrix = matrix.pow(4);

    let mut v = DVector::zeros(matrix.nrows());
    v[dictionary.get([b'A'])] = 1;

    (matrix * v).sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &[u8]) -> u32 {
    let rules = parse_data(data);

    generate::<10, 1>(&rules, [b'Z'])
}

/// # Panics
#[must_use]
pub fn part_2_matrix(data: &[u8]) -> u32 {
    let (dictionary, matrix) = parse(data);

    let matrix = matrix.pow(10);

    let mut v = DVector::zeros(matrix.nrows());
    v[dictionary.get([b'Z'])] = 1;

    (matrix * v).sum()
}

/// # Panics
#[must_use]
pub fn part_3<const SIZE: usize>(data: &[u8]) -> u32 {
    let rules = parse_data(data);

    let (min, max) = rules
        .keys()
        .map(|origin| generate::<20, SIZE>(&rules, *origin))
        .fold((u32::MAX, u32::MIN), |(min, max), value| {
            (min.min(value), max.max(value))
        });

    max - min
}

/// # Panics
#[must_use]
pub fn part_3_matrix<const SIZE: usize>(data: &[u8]) -> u32 {
    let (_, matrix) = parse::<SIZE>(data);

    let matrix = matrix.pow(20);

    let mut tmp = DVector::zeros(matrix.nrows());
    let (min, max) = (0..matrix.nrows())
        .map(|i| {
            let mut v = DVector::zeros(matrix.nrows());
            v[i] = 1;
            matrix.mul_to(&v, &mut tmp);
            tmp.sum()
        })
        .fold((u32::MAX, u32::MIN), |(min, max), value| {
            (min.min(value), max.max(value))
        });

    max - min
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal)]
    
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            8,
            part_1(
                br"A:B,C
B:C,A
C:A"
            )
        );
    }

    #[test]
    fn test_part_1_matrix() {
        assert_eq!(
            8,
            part_1_matrix(
                br"A:B,C
B:C,A
C:A"
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(261085, part_2(include_bytes!("../data/part_2")));
    }

    #[test]
    fn test_part_2_matrix() {
        assert_eq!(261085, part_2_matrix(include_bytes!("../data/part_2")));
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            268815,
            part_3::<1>(
                br"A:B,C
B:C,A,A
C:A"
            )
        );
    }

    #[test]
    fn test_part_3_matrix() {
        assert_eq!(
            268815,
            part_3_matrix::<1>(
                br"A:B,C
B:C,A,A
C:A"
            )
        );
    }
}
