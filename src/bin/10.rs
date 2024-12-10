use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut trail_heads = VecDeque::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = c.to_digit(10).unwrap();

                    if height == 0 {
                        trail_heads.push_back(((x, y), (x, y)));
                    }

                    height
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();

    while let Some(((x, y), start)) = trail_heads.pop_front() {
        let level = map[y][x];

        if level == 9 {
            visited.insert(((x, y), start));
            continue;
        }

        trail_heads.extend(
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .flat_map(|(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
                .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                .filter(|&(_, height)| height as i32 - level as i32 == 1)
                .map(|(pos, _)| (pos, start)),
        );
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut trail_heads = VecDeque::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = c.to_digit(10).unwrap();

                    if height == 0 {
                        trail_heads.push_back((x, y));
                    }

                    height
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    while let Some((x, y)) = trail_heads.pop_front() {
        let level = map[y][x];

        if level == 9 {
            count += 1;
            continue;
        }

        trail_heads.extend(
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .flat_map(|(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
                .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                .filter(|&(_, height)| height as i32 - level as i32 == 1)
                .map(|(pos, _)| pos),
        );
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
