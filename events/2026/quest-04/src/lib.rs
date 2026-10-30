use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::{IResult, Parser};

fn parse_tag(t: &str) -> impl Fn(&str) -> IResult<&str, usize> {
    move |input| {
        let (input, _) = tag(t)(input)?;
        let (input, id) = digit1(input)?;
        Ok((input, id.parse().unwrap()))
    }
}

fn parse_time(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("time=")(input)?;
    let (input, time) = digit1(input)?;
    Ok((input, time.parse().unwrap()))
}

/// # Panics
#[must_use]
pub fn part_1(data: &str) -> u64 {
    let mut len = 0;
    let mut ranking = [0u64; 10];
    for line in data.lines() {
        let (_, (id, _, _quest, _, _part, _, time)) = (
            parse_tag("id="),
            space1,
            parse_tag("quest="),
            space1,
            parse_tag("part="),
            space1,
            parse_time,
        )
            .parse(line)
            .expect("invalid line");

        len = len.max(id);

        ranking[id - 1] += time;
    }

    ranking[..len].sort_unstable();

    ranking
        .iter()
        .take(len)
        .enumerate()
        .map(|(index, infos)| (index + 1) as u64 * infos)
        .sum()
}

/// # Panics
#[must_use]
pub fn part_2(data: &str) -> u64 {
    let mut len = 0;
    let mut ranking = [(0usize, 0u64); 100];
    for line in data.lines() {
        let (_, (id, _, _quest, _, _part, _, time)) = (
            parse_tag("id="),
            space1,
            parse_tag("quest="),
            space1,
            parse_tag("part="),
            space1,
            parse_time,
        )
            .parse(line)
            .expect("invalid line");

        len = len.max(id);

        let infos = &mut ranking[id - 1];
        infos.0 += 1;
        infos.1 += time;
    }

    ranking[..len].sort_unstable_by(|(quests_1, time_1), (quests_2, time_2)| {
        quests_2.cmp(quests_1).then(time_1.cmp(time_2))
    });

    ranking
        .iter()
        .take(len)
        .enumerate()
        .map(|(index, infos)| (index + 1) as u64 * infos.1)
        .sum()
}

/// # Panics
#[must_use]
#[expect(clippy::large_stack_arrays, reason = "optimization (???)")]
pub fn part_3(data: &str) -> u64 {
    let mut max_id = 0;
    let mut max_quest = 0;
    let mut ranking = [[[None; 3]; 300]; 20];
    for line in data.lines() {
        let (_, (id, _, quest, _, part, _, time)) = (
            parse_tag("id="),
            space1,
            parse_tag("quest="),
            space1,
            parse_tag("part="),
            space1,
            parse_time,
        )
            .parse(line)
            .expect("invalid line");

        max_id = max_id.max(id);
        max_quest = max_quest.max(quest);

        ranking[quest - 1][id - 1][part - 1] = Some(time);
    }

    let mut points = [0u64; 300];

    for ids in ranking.iter_mut().take(max_quest) {
        let mut rank = [(u64::MAX, 0usize); 300];
        let mut calc_rank = |start, f: fn(&[Option<u64>; 3]) -> u64| {
            for (index, (id, id_info)) in rank.iter_mut().take(max_id).zip(ids.iter()).enumerate() {
                *id = (f(id_info), index);
            }

            rank[..max_id].sort_unstable();

            assign_points(start, &mut points[..max_id], &rank[..max_id]);
        };

        calc_rank(50, |id_info| id_info[0].unwrap_or(u64::MAX));

        calc_rank(100, |id_info| {
            id_info[0]
                .and_then(|time_1| id_info[1].map(|time_2| time_1 + time_2))
                .unwrap_or(u64::MAX)
        });

        calc_rank(150, |id_info| {
            id_info[0]
                .and_then(|time_1| id_info[1].map(|time_2| time_1 + time_2))
                .and_then(|time_1_2| id_info[2].map(|time_3| time_1_2 + time_3))
                .unwrap_or(u64::MAX)
        });
    }

    points[..max_id].sort_unstable_by(|points_1, points_2| points_2.cmp(points_1));

    let mut total_points = 0;
    let mut last_points = u64::MAX;
    let mut current_position = 1;
    for (next_position, p) in points.iter().take(max_id).enumerate() {
        if last_points != *p {
            current_position = next_position + 1;
            last_points = *p;
        }
        total_points += current_position as u64 * *p;
    }

    total_points
}

fn assign_points(mut start: u64, points: &mut [u64], rank: &[(u64, usize)]) {
    let mut current = start;
    let mut last = u64::MAX;
    for (time, id) in rank {
        if *time == u64::MAX {
            break;
        }

        if *time != last {
            current = start;
            last = *time;
        }

        points[*id] += current;

        if start == 0 {
            break;
        }

        start -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                r"id=1 quest=1 part=1 time=187
id=3 quest=1 part=1 time=413
id=5 quest=1 part=3 time=744
id=2 quest=1 part=3 time=745
id=4 quest=1 part=2 time=558
id=1 quest=1 part=2 time=299
id=1 quest=1 part=3 time=362
id=4 quest=1 part=3 time=1268
id=4 quest=1 part=1 time=457
id=5 quest=1 part=1 time=257
id=2 quest=1 part=1 time=192
id=3 quest=1 part=3 time=1168
id=5 quest=1 part=2 time=346
id=3 quest=1 part=2 time=455
id=2 quest=1 part=2 time=562"
            ),
            27598,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                r"id=2 quest=1 part=1 time=100
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=140
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=240
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=175
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=160"
            ),
            6470,
        );
    }

    #[test]
    fn test_part_3_1() {
        assert_eq!(
            part_3(
                r"id=2 quest=1 part=1 time=100
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=140
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=240
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=175
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=160"
            ),
            2998,
        );
    }

    #[test]
    fn test_part_3_2() {
        assert_eq!(
            part_3(
                r"id=2 quest=1 part=1 time=90
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=130
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=220
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=170
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=170"
            ),
            3004,
        );
    }

    #[test]
    #[expect(clippy::unreadable_literal, reason = "spec")]
    fn test_part_3_3() {
        assert_eq!(part_3(include_str!("../data/example_part_3"),), 6232480,);
    }
}
