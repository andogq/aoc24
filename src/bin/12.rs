use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut fences = Vec::new();

    for (x, y) in (0..map.len()).flat_map(|y| (0..map[y].len()).map(move |x| (x, y))) {
        if visited.contains(&(x, y)) {
            continue;
        }

        let c = map[y][x];

        let mut area = 0;
        let mut perimeter = 0;

        let mut search = VecDeque::from([(x, y)]);

        while let Some((x, y)) = search.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }

            // Mark this location as visited
            visited.insert((x, y));

            // Increase the area of this region
            area += 1;

            // Immediately count 4 different neighbours
            perimeter += 4;

            // Find matching neighbours
            search.extend(
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .flat_map(|(dx, dy)| {
                        Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
                    })
                    .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                    .filter(|(_, neighbour)| *neighbour == c)
                    .map(|(pos, _)| pos)
                    .inspect(|_| {
                        // Same neighbour, so reduce perimeter
                        perimeter -= 1;
                    }),
            );
        }

        fences.push((area, perimeter));
    }

    Some(
        fences
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut fences = Vec::new();

    for (x, y) in (0..map.len()).flat_map(|y| (0..map[y].len()).map(move |x| (x, y))) {
        if visited.contains(&(x, y)) {
            continue;
        }

        let c = map[y][x];

        let mut area = 0;
        let mut corners = 0;

        let mut search = VecDeque::from([(x, y)]);

        while let Some((x, y)) = search.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }

            // Mark this location as visited
            visited.insert((x, y));

            // Increase the area of this region
            area += 1;

            // Count the corners
            corners += std::iter::repeat([(-1, 0), (0, -1), (1, 0), (0, 1)])
                .flatten()
                .tuple_windows()
                .take(4)
                .map(|(side_a, side_b)| {
                    [side_a, side_b, (side_a.0 + side_b.0, side_a.1 + side_b.1)].map(|(dx, dy)| {
                        let x = x.checked_add_signed(dx)?;
                        let y = y.checked_add_signed(dy)?;
                        let side = *map.get(y)?.get(x)?;

                        Some(side)
                    })
                })
                .filter(|[side_a, side_b, diag]| {
                    (Some(c) != *side_a && Some(c) != *side_b)
                        || (Some(c) == *side_a && Some(c) == *side_b && Some(c) != *diag)
                })
                .count();

            // Find matching neighbours
            search.extend(
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .flat_map(|(dx, dy)| {
                        Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
                    })
                    .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                    .filter(|(_, neighbour)| *neighbour == c)
                    .map(|(pos, _)| pos),
            );
        }

        fences.push((area, corners));
    }

    Some(
        fences
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
