use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|line| ((line[0] as i32 - line[1] as i32).signum(), line))
            .filter(|(dir, line)| {
                line.iter().tuple_windows().all(|(a, b)| {
                    let diff = *a as i32 - *b as i32;

                    diff.signum() == *dir && (1..=3).contains(&diff.abs())
                })
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|line| {
                (0..=line.len()).any(|skip| {
                    let mut iter = line
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != skip)
                        .map(|(_, n)| n)
                        .tuple_windows()
                        .peekable();

                    let dir = iter
                        .peek()
                        .map(|(a, b)| (**a as i32 - **b as i32).signum())
                        .unwrap();

                    iter.all(|(a, b)| {
                        let diff = *a as i32 - *b as i32;

                        diff.signum() == dir && (1..=3).contains(&diff.abs())
                    })
                })
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
