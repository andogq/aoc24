use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = input
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .fold(HashMap::<u64, u64>::new(), |mut stones, stone| {
            *stones.entry(stone).or_default() += 1;

            stones
        });

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|(stone, count)| {
                if stone == 0 {
                    return [Some((1, count)), None];
                }

                let num_digits = stone.ilog10() + 1;
                if num_digits % 2 == 0 {
                    return [
                        Some((stone / 10u64.pow(num_digits / 2), count)),
                        Some((stone % 10u64.pow(num_digits / 2), count)),
                    ];
                }

                [Some((stone * 2024, count)), None]
            })
            .flatten()
            .fold(HashMap::new(), |mut stones, (stone, count)| {
                *stones.entry(stone).or_default() += count;

                stones
            });
    }

    Some(stones.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
