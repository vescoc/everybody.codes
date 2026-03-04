#![no_std]

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    data.lines()
        .filter_map(|line| {
            let (scale, colors) = line.split_once(':').expect("invalid line");
            let mut colors = colors.split_whitespace().map(|color| {
                color
                    .chars()
                    .fold(0u32, |a, c| a * 2 + u32::from(c.is_uppercase()))
            });

            let red = colors.next().expect("cannot find red");
            let green = colors.next().expect("cannot find green");
            let blue = colors.next().expect("cannot find blue");
            if green > red && green > blue {
                Some(scale.parse::<u64>().expect("invalid scale"))
            } else {
                None
            }
        })
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            let (scale, colors) = line.split_once(':').expect("invalid line");
            let mut colors = colors.split_whitespace().map(|color| {
                color
                    .chars()
                    .fold(0u32, |a, c| a * 2 + u32::from(c.is_uppercase()))
            });

            let red = colors.next().expect("cannot find red");
            let green = colors.next().expect("cannot find green");
            let blue = colors.next().expect("cannot find blue");
            let shine = colors.next().expect("cannot find shine");

            (scale, (red + green + blue, shine))
        })
        .max_by(|(_, (a_color, a_shine)), (_, (b_color, b_shine))| {
            a_shine.cmp(b_shine).then(b_color.cmp(a_color))
        })
        .expect("cannot find shining color")
        .0
        .parse()
        .expect("invalid scale")
}

/// # Panics
#[allow(clippy::similar_names)]
#[must_use]
pub fn part_3(data: &str) -> u64 {
    let mut color_infos = [(0, 0); 6];
    for line in data.lines() {
        let (scale, colors) = line.split_once(':').expect("invalid line");
        let mut colors = colors.split_whitespace().map(|color| {
            color
                .chars()
                .fold(0u32, |a, c| a * 2 + u32::from(c.is_uppercase()))
        });

        let red = colors.next().expect("cannot find red");
        let green = colors.next().expect("cannot find green");
        let blue = colors.next().expect("cannot find blue");
        let shine = colors.next().expect("cannot find shine");

        let matte = shine <= 30;
        let shiny = shine >= 33;

        let color_infos = if matte {
            Some(&mut color_infos[0..3])
        } else if shiny {
            Some(&mut color_infos[3..6])
        } else {
            None
        };

        if let Some(color_infos) = color_infos {
            let is_red = red > green && red > blue;
            let is_green = green > red && green > blue;
            let is_blue = blue > red && blue > green;

            let color_info = if is_red {
                color_infos.get_mut(0)
            } else if is_green {
                color_infos.get_mut(1)
            } else if is_blue {
                color_infos.get_mut(2)
            } else {
                None
            };

            if let Some((count, total)) = color_info {
                *count += 1;
                *total += scale.parse::<u64>().expect("invalid scale");
            }
        }
    }

    color_infos
        .iter()
        .max_by_key(|(count, _)| *count)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = r"2456:rrrrrr ggGgGG bbbbBB
7689:rrRrrr ggGggg bbbBBB
3145:rrRrRr gggGgg bbbbBB
6710:rrrRRr ggGGGg bbBBbB";
        assert_eq!(part_1(data), 9166);
    }

    #[test]
    fn test_part_2() {
        let data = r"2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss";
        assert_eq!(part_2(data), 2456);
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_part_3() {
        let data = r"15437:rRrrRR gGGGGG BBBBBB sSSSSS
94682:RrRrrR gGGggG bBBBBB ssSSSs
56513:RRRrrr ggGGgG bbbBbb ssSsSS
76346:rRRrrR GGgggg bbbBBB ssssSs
87569:rrRRrR gGGGGg BbbbbB SssSss
44191:rrrrrr gGgGGG bBBbbB sSssSS
49176:rRRrRr GggggG BbBbbb sSSssS
85071:RRrrrr GgGGgg BBbbbb SSsSss
44303:rRRrrR gGggGg bBbBBB SsSSSs
94978:rrRrRR ggGggG BBbBBb SSSSSS
26325:rrRRrr gGGGgg BBbBbb SssssS
43463:rrrrRR gGgGgg bBBbBB sSssSs
15059:RRrrrR GGgggG bbBBbb sSSsSS
85004:RRRrrR GgGgGG bbbBBB sSssss
56121:RRrRrr gGgGgg BbbbBB sSsSSs
80219:rRRrRR GGGggg BBbbbb SssSSs";
        assert_eq!(part_3(data), 292320);
    }
}
