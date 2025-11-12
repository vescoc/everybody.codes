use std::collections::HashMap;

const SIZE: usize = (b'z' - b'a' + 1) as usize * 2;

fn index(letter: u8) -> usize {
    (match letter {
        b'a'..=b'z' => letter - b'a',
        b'A'..=b'Z' => letter - b'A' + b'z' - b'a' + 1,
        _ => unreachable!(),
    }) as usize
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> &str {
    let (names, rules_part) = data.split_once("\n\n").expect("invalid data");

    let mut rules = [[false; SIZE]; SIZE];
    for rule in rules_part.lines() {
        let (head, tail) = rule.split_once(" > ").expect("invalid rule");
        
        let head = head.bytes().next().unwrap();
        for letter in tail.split(',') {
            let target = letter.bytes().next().unwrap();
            
            rules[index(head)][index(target)] = true;
        }
    }

    names
        .split(',')
        .find(|name| {
            name.as_bytes()
                .windows(2)
                .all(|pair| rules[index(pair[0])][index(pair[1])])
        })
        .expect("not found")
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> usize {
    let (names, rules_part) = data.split_once("\n\n").expect("invalid data");

    let mut rules = [[false; SIZE]; SIZE];
    for rule in rules_part.lines() {
        let (head, tail) = rule.split_once(" > ").expect("invalid rule");
        
        let head = head.bytes().next().unwrap();
        for letter in tail.split(',') {
            let target = letter.bytes().next().unwrap();
            
            rules[index(head)][index(target)] = true;
        }
    }

    names
        .split(',')
        .enumerate()
        .filter_map(|(i, name)| {
            let valid = name
                .as_bytes()
                .windows(2)
                .all(|pair| rules[index(pair[0])][index(pair[1])]);
            if valid { Some(i + 1) } else { None }
        })
        .sum()
}

fn unique_names(
    memoize: &mut HashMap<(u8, usize), usize>,
    generation_rules: &HashMap<u8, Vec<u8>>,
    letter: u8,
    prefix_length: usize,
) -> usize {
    if let Some(result) = memoize.get(&(letter, prefix_length)) {
        return *result;
    }

    let result = if prefix_length <= 11 {
        usize::from(prefix_length >= 7)
            + generation_rules.get(&letter).map_or(0, |targets| {
                targets
                    .iter()
                    .map(|target| {
                        unique_names(memoize, generation_rules, *target, prefix_length + 1)
                    })
                    .sum()
            })
    } else {
        0
    };

    memoize.insert((letter, prefix_length), result);

    result
}

/// # Panics
#[must_use]
pub fn part_3(data: &str) -> usize {
    let (names, rules_part) = data.split_once("\n\n").expect("invalid data");

    let mut rules = [[false; SIZE]; SIZE];
    let mut generation_rules = HashMap::<u8, Vec<u8>>::with_capacity(4096);
    for rule in rules_part.lines() {
        let (head, tail) = rule.split_once(" > ").expect("invalid rule");
        
        let head = head.bytes().next().unwrap();
        for letter in tail.split(',') {
            let target = letter.bytes().next().unwrap();
            
            rules[index(head)][index(target)] = true;

            let list = generation_rules.entry(head).or_default();
            list.push(target);
        }
    }

    let names = names.split(',').map(str::as_bytes).collect::<Vec<_>>();

    let mut memoize = HashMap::with_capacity(4096);
    names
        .iter()
        .filter_map(|name| {
            let valid = names
                .iter()
                .all(|prefix| name == prefix || !name.starts_with(prefix))
                && name
                    .windows(2)
                    .all(|pair| rules[index(pair[0])][index(pair[1])]);
            if valid {
                Some(unique_names(
                    &mut memoize,
                    &generation_rules,
                    *name.last().unwrap(),
                    name.len(),
                ))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h"
            ),
            "Oroneth",
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"
            ),
            23,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t"
            ),
            25,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"
            ),
            1154,
        );
    }
}
