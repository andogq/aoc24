use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (height, width) = {
        let mut lines = input.lines();

        let width = lines.next().unwrap().len();
        let height = lines.count() + 1;

        (height, width)
    };

    Some(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| match c {
                    '.' => None,
                    c => Some(((x, y), c)),
                })
            })
            .fold(
                HashMap::<char, Vec<(usize, usize)>>::new(),
                |mut antennas, (pos, freqency)| {
                    antennas.entry(freqency).or_default().push(pos);

                    antennas
                },
            )
            .values()
            .flat_map(|antennas| {
                antennas.iter().tuple_combinations().flat_map(|(a1, a2)| {
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
                    .filter(|(x, y)| *x < width && *y < height)
                })
            })
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
