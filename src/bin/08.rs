use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn solve<F, I>(input: &str, get_points: F) -> u32
where
    F: Fn((usize, usize), [(usize, usize); 2]) -> I,
    I: Iterator<Item = (usize, usize)>,
{
    // Determine the bounds of the map
    let (height, width) = {
        let mut lines = input.lines();

        let width = lines.next().unwrap().len();
        let height = lines.count() + 1;

        (height, width)
    };

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '.' => None,
                c => Some(((x, y), c)),
            })
        })
        // Group the antennas by frequency
        .fold(
            HashMap::<char, Vec<(usize, usize)>>::new(),
            |mut antennas, (pos, freqency)| {
                antennas.entry(freqency).or_default().push(pos);

                antennas
            },
        )
        .values()
        // Generate individal combinations for each antenna
        .flat_map(|antennas| antennas.iter().tuple_combinations())
        // Call out to generate points for an antenna pair
        .flat_map(|(a1, a2)| get_points((height, width), [*a1, *a2]))
        // De-dupe and count resonance points
        .collect::<HashSet<_>>()
        .len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, |(height, width), [a1, a2]| {
        let delta = (a2.0 as isize - a1.0 as isize, a2.1 as isize - a1.1 as isize);

        [
            (
                a1.0.checked_add_signed(-delta.0),
                a1.1.checked_add_signed(-delta.1),
            ),
            (
                a2.0.checked_add_signed(delta.0),
                a2.1.checked_add_signed(delta.1),
            ),
        ]
        .into_iter()
        .flat_map(|(x, y)| Some((x?, y?)))
        .filter(move |(x, y)| *x < width && *y < height)
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, |(height, width), [a1, a2]| {
        let delta = (a2.0 as isize - a1.0 as isize, a2.1 as isize - a1.1 as isize);

        [(a1, -1), (a2, 1)].into_iter().flat_map(move |(p, d)| {
            let delta = (delta.0 * d, delta.1 * d);

            (0..)
                .map(move |i| (delta.0 * i, delta.1 * i))
                .map(move |(dx, dy)| (p.0.checked_add_signed(dx), p.1.checked_add_signed(dy)))
                .map(move |(x, y)| {
                    let x = x?;
                    let y = y?;

                    if x < width && y < height {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .take_while(|p| p.is_some())
                .flatten()
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
