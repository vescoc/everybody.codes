#![no_std]

use rayon::prelude::*;

fn decode(color: &str) -> u32 {
    color
        .chars()
        .fold(0, |a, c| (a << 1) + u32::from(c.is_uppercase()))
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    data.lines()
        .filter_map(|line| {
            let (scale, colors) = line.split_once(':').expect("invalid line");
            let mut colors = colors.split_whitespace().map(decode);

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
    data.par_lines()
        .map(|line| {
            let (scale, colors) = line.split_once(':').expect("invalid line");
            let mut colors = colors.split_whitespace().map(decode);

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
    use core::sync::atomic::{self, AtomicU64, Ordering};

    let color_infos = [const { (AtomicU64::new(0), AtomicU64::new(0)) }; 6];
    data.par_lines().for_each(|line| {
        let (scale, colors) = line.split_once(':').expect("invalid line");
        let mut colors = colors.split_whitespace().map(decode);

        let red = colors.next().expect("cannot find red");
        let green = colors.next().expect("cannot find green");
        let blue = colors.next().expect("cannot find blue");
        let shine = colors.next().expect("cannot find shine");

        let matte = shine <= 30;
        let shiny = shine >= 33;

        let color_infos = if matte {
            Some(&color_infos[0..3])
        } else if shiny {
            Some(&color_infos[3..6])
        } else {
            None
        };

        let color_info = if let Some(color_infos) = color_infos {
            let is_red = red > green && red > blue;
            let is_green = green > red && green > blue;
            let is_blue = blue > red && blue > green;

            if is_red {
                color_infos.get(0)
            } else if is_green {
                color_infos.get(1)
            } else if is_blue {
                color_infos.get(2)
            } else {
                None
            }
        } else {
            None
        };

        if let Some((count, total)) = color_info {
            count.fetch_add(1, Ordering::Acquire);
            total.fetch_add(
                scale.parse::<u64>().expect("invalid scale"),
                Ordering::Acquire,
            );
        }
    });

    atomic::compiler_fence(Ordering::Release);

    let (mut max_count, mut max_scale) = (u64::MIN, u64::MIN);
    for (count, scale) in color_infos {
        let (count, scale) = (count.into_inner(), scale.into_inner());
        if max_count < count {
            max_count = count;
            max_scale = scale;
        }
    }

    max_scale
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
