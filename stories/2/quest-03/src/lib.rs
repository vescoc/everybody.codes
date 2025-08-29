use std::collections::HashSet;

#[derive(Debug)]
struct Dice<'a> {
    id: &'a str,
    values: Vec<i64>,
    seed: i64,
    pulse: i64,
    spin: i64,
    roll_number: i64,
    total_spin: i64,
}

impl<'a> Dice<'a> {
    fn parse(line: &'a str) -> Self {
        let mut parts = line.split_whitespace();

        let id = parts.next().unwrap();
        let (id, _) = id.split_once(':').unwrap();

        let values = parts.next().unwrap();
        let (_, values) = values.split_once('[').unwrap();
        let (values, _) = values.split_once(']').unwrap();
        let values = values
            .split(',')
            .map(|value| value.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let seed = parts.next().unwrap();
        let (_, seed) = seed.split_once('=').unwrap();

        Self::new(id, values, seed.parse::<i64>().unwrap())
    }

    fn new(id: &'a str, values: Vec<i64>, seed: i64) -> Self {
        Self {
            id,
            values,
            seed,
            pulse: seed,
            spin: 0,
            roll_number: 1,
            total_spin: 0,
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn roll(&mut self) -> i64 {
        self.spin = self.roll_number * self.pulse;

        self.total_spin += self.spin;

        self.pulse =
            (self.pulse + self.spin).rem_euclid(self.seed) + 1 + self.roll_number + self.seed;

        self.roll_number += 1;

        self.values[self.total_spin.rem_euclid(self.values.len() as i64) as usize]
    }
}

struct Grid(Vec<HashSet<(isize, isize)>>);

impl Grid {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn get(&self, roll: i64) -> Option<&HashSet<(isize, isize)>> {
        self.0.get(roll as usize - 1)
    }
}

impl FromIterator<(i64, (isize, isize))> for Grid {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from_iter<II: IntoIterator<Item = (i64, (isize, isize))>>(ii: II) -> Self {
        let mut grid = vec![HashSet::new(); 9];
        for (digit, coord) in ii {
            grid[digit as usize - 1].insert(coord);
        }

        Self(grid)
    }
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> usize {
    let mut dices = data.lines().map(Dice::parse).collect::<Vec<_>>();

    let mut roll_number = 1;
    let mut sum = 0;
    loop {
        for dice in &mut dices {
            sum += dice.roll();
        }

        if sum >= 10_000 {
            return roll_number;
        }

        roll_number += 1;
    }
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> String {
    let (dices, track) = data.split_once("\n\n").unwrap();

    let mut dices = dices.lines().map(Dice::parse).collect::<Vec<_>>();

    let track = track
        .as_bytes()
        .iter()
        .map(|digit| i64::from(digit - b'0'))
        .collect::<Vec<_>>();

    let mut turns = dices
        .iter_mut()
        .map(|dice| {
            let mut turn = 0;
            for tile in &track {
                loop {
                    turn += 1;

                    let face = dice.roll();
                    if face == *tile {
                        break;
                    }
                }
            }

            (dice.id, turn)
        })
        .collect::<Vec<_>>();

    turns.sort_unstable_by_key(|(_, turn)| *turn);

    turns
        .iter()
        .map(|(id, _)| *id)
        .collect::<Vec<_>>()
        .join(",")
}

/// # Panics
#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn part_3(data: &str) -> usize {
    let (dices, grid) = data.split_once("\n\n").unwrap();

    let dices = dices.lines().map(Dice::parse).collect::<Vec<_>>();

    let grid = grid
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars().enumerate().map(move |(c, digit)| {
                (
                    i64::from(u32::from(digit) - u32::from('0')),
                    (r as isize, c as isize),
                )
            })
        })
        .collect::<Grid>();

    let mut all_paths = HashSet::with_capacity(515 * 515);
    for mut dice in dices {
        let first_roll = dice.roll();
        let mut paths = grid
            .get(first_roll)
            .map(|set| {
                set.iter()
                    .map(|coord| {
                        all_paths.insert(*coord);
                        *coord
                    })
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        while !paths.is_empty() {
            let mut new_paths = HashSet::new();
            let roll = dice.roll();
            if let Some(coords) = grid.get(roll) {
                for (token_r, token_c) in paths {
                    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                        let (new_token_r, new_token_c) = (token_r + dr, token_c + dc);
                        if coords.contains(&(new_token_r, new_token_c)) {
                            all_paths.insert((new_token_r, new_token_c));
                            new_paths.insert((new_token_r, new_token_c));
                        }
                    }
                }
            }

            paths = new_paths;
        }
    }

    all_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice() {
        let mut dice = Dice::new("1", vec![1, 2, 4, -1, 5, 7, 9], 3);

        assert_eq!(dice.roll(), -1);
        assert_eq!(dice.spin, 3);
        assert_eq!(dice.pulse, 5);

        assert_eq!(dice.roll(), 9);
        assert_eq!(dice.spin, 10);
        assert_eq!(dice.pulse, 6);

        assert_eq!(dice.roll(), -1);
        assert_eq!(dice.spin, 18);
        assert_eq!(dice.pulse, 7);

        assert_eq!(dice.roll(), -1);
        assert_eq!(dice.spin, 28);
        assert_eq!(dice.pulse, 10);

        assert_eq!(dice.roll(), 5);
        assert_eq!(dice.spin, 50);
        assert_eq!(dice.pulse, 9);
    }

    #[test]
    fn test_part_1() {
        let data = r"1: values=[1,2,3,4,5,6] seed=7
2: values=[-1,1,-1,1,-1] seed=13
3: values=[9,8,7,8,9] seed=17";

        assert_eq!(part_1(data), 844);
    }

    #[test]
    fn test_part_2() {
        let data = r"1: values=[1,2,3,4,5,6,7,8,9] seed=13
2: values=[1,2,3,4,5,6,7,8,9] seed=29
3: values=[1,2,3,4,5,6,7,8,9] seed=37
4: values=[1,2,3,4,5,6,7,8,9] seed=43

51257284";

        assert_eq!(&part_2(data), "1,3,4,2");
    }

    #[test]
    fn test_part_3_1() {
        let data = r"1: values=[1,2,3,4,5,6,7,8,9] seed=13

1523758297
4822941583
7627997892
4397697132
1799773472";

        assert_eq!(part_3(data), 33);
    }

    #[test]
    fn test_part_3_2() {
        let data = r"1: faces=[1,2,3,4,5,6,7,8,9] seed=339211
2: faces=[1,2,3,4,5,6,7,8,9] seed=339517
3: faces=[1,2,3,4,5,6,7,8,9] seed=339769
4: faces=[1,2,3,4,5,6,7,8,9] seed=339049
5: faces=[1,2,3,4,5,6,7,8,9] seed=338959
6: faces=[1,2,3,4,5,6,7,8,9] seed=340111
7: faces=[1,2,3,4,5,6,7,8,9] seed=339679
8: faces=[1,2,3,4,5,6,7,8,9] seed=339121
9: faces=[1,2,3,4,5,6,7,8,9] seed=338851

94129478611916584144567479397512595367821487689499329543245932151
45326719759656232865938673559697851227323497148536117267854241288
44425936468288462848395149959678842215853561564389485413422813386
64558359733811767982282485122488769592428259771817485135798694145
17145764554656647599363636643624443394141749674594439266267914738
89687344812176758317288229174788352467288242171125512646356965953
72436836424726621961424876248346712363842529736689287535527512173
18295771348356417112646514812963612341591986162693455745689374361
56445661964557624561727322332461348422854112571195242864151143533
77537797151985578367895335725777225518396231453691496787716283477
37666899356978497489345173784484282858559847597424967325966961183
26423131974661694562195955939964966722352323745667498767153191712
99821139398463125478734415536932821142852955688669975837535594682
17768265895455681847771319336534851247125295119363323122744953158
25655579913247189643736314385964221584784477663153155222414634387
62881693835262899543396571369125158422922821541597516885389448546
71751114798332662666694134456689735288947441583123159231519473489
94932859392146885633942828174712588132581248183339538341386944937
53828883514868969493559487848248847169557825166338328352792866332
54329673374115668178556175692459528276819221245996289611868492731
97799599164121988455613343238811122469229423272696867686953891233
56249752581283778997317243845187615584225693829653495119532543712
39171354221177772498317826968247939792845866251456175433557619425
56425749216121421458547849142439211299266255482219915528173596421
48679971256541851497913572722857258171788611888347747362797259539
32676924489943265499379145361515824954991343541956993467914114579
45733396847369746189956225365375253819969643711633873473662833395
42291594527499443926636288241672629499242134451937866578992236427
47615394883193571183931424851238451485822477158595936634849167455
16742896921499963113544858716552428241241973653655714294517865841
57496921774277833341488566199458567884285639693339942468585269698
22734249697451127789698862596688824444191118289959746248348491792
28575193613471799766369217455617858422158428235521423695479745656
74234343226976999161289522983885254212712515669681365845434541257
43457237419516813368452247532764649744546181229533942414983335895";

        assert_eq!(part_3(data), 1125);
    }
}
