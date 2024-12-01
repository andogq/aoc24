advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let line_count = input.lines().count();

    let (mut left, mut right) = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            (
                line.next().unwrap().parse::<u32>().unwrap(),
                line.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .fold(
            (
                Vec::with_capacity(line_count),
                Vec::with_capacity(line_count),
            ),
            |(mut left, mut right), (left_num, right_num)| {
                left.push(left_num);
                right.push(right_num);

                (left, right)
            },
        );

    left.sort_unstable();
    right.sort_unstable();

    Some(
        left.into_iter()
            .zip(right)
            .map(|(left, right)| left.abs_diff(right))
            .sum(),
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
