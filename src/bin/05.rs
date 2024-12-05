use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(PartialEq, Eq)]
enum Order {
    Before,
    After,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            line.split("|")
                .map(|n| n.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .fold(HashMap::new(), |mut map, (left, right)| {
            map.insert((left, right), Order::Before);
            map.insert((right, left), Order::After);

            map
        });

    Some(
        pages
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|line| {
                line.iter().enumerate().all(|(i, left)| {
                    line[i + 1..]
                        .iter()
                        .flat_map(|right| rules.get(&(*left, *right)))
                        .all(|order| *order == Order::Before)
                })
            })
            .map(|line| line[line.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            line.split("|")
                .map(|n| n.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .fold(HashMap::new(), |mut map, (left, right)| {
            map.insert((left, right), Order::Before);
            map.insert((right, left), Order::After);

            map
        });

    Some(
        pages
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|line| {
                !line.iter().enumerate().all(|(i, left)| {
                    line[i + 1..]
                        .iter()
                        .flat_map(|right| rules.get(&(*left, *right)))
                        .all(|order| *order == Order::Before)
                })
            })
            .map(|mut line| {
                line.sort_unstable_by(|a, b| match rules.get(&(*a, *b)) {
                    Some(Order::Before) => Ordering::Less,
                    Some(Order::After) => Ordering::Greater,
                    None => Ordering::Equal,
                });
                line
            })
            .map(|line| line[line.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
