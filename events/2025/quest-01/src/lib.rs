#![no_std]

type Vec<T> = heapless::Vec<T, 256>;

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> &str {
    let (names, moves) = data.split_once("\n\n").expect("invalid data");

    let list = names.split(',').collect::<Vec<_>>();

    let index = moves.split(',').fold(0usize, |current, current_move| {
        let mut mv = current_move.chars();
        let dir = mv.next();
        let steps = mv.as_str().parse::<usize>().expect("invalid move steps");
        match dir {
            Some('R') => current.saturating_add(steps).min(list.len() - 1),
            Some('L') => current.saturating_sub(steps),
            _ => panic!("invalid move"),
        }
    });

    list[index]
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> &str {
    let (names, moves) = data.split_once("\n\n").expect("invalid data");

    let list = names.split(',').collect::<Vec<_>>();

    let index = moves.split(',').fold(0usize, |current, current_move| {
        let mut mv = current_move.chars();
        let dir = mv.next();
        let steps = mv.as_str().parse::<usize>().expect("invalid move steps");
        match dir {
            Some('R') => current.saturating_add(steps),
            Some('L') => {
                if steps >= current {
                    list.len() - (steps - current).rem_euclid(list.len())
                } else {
                    current - steps
                }
            }
            _ => panic!("invalid move"),
        }
        .rem_euclid(list.len())
    });

    list[index]
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> &str {
    let (names, moves) = data.split_once("\n\n").expect("invalid data");

    let mut list = names.split(',').collect::<Vec<_>>();
    for current_move in moves.split(',') {
        let mut mv = current_move.chars();
        let dir = mv.next();
        let steps = mv.as_str().parse::<usize>().expect("invalid move steps");
        let target = match dir {
            Some('R') => steps,
            Some('L') => list.len() - steps.rem_euclid(list.len()),
            _ => panic!("invalid move"),
        }
        .rem_euclid(list.len());

        list.swap(0, target);
    }

    list[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"
            ),
            "Fyrryn",
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"
            ),
            "Elarzris",
        );
    }

    #[test]
    fn test_part_3() {
        assert_eq!(
            part_3(
                r"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3"
            ),
            "Drakzyph",
        );
    }
}
