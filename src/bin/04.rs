advent_of_code::solution!(4);

const CHARS: &[char] = &['X', 'M', 'A', 'S'];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        (0..grid.len())
            .flat_map(|y| (0..grid[0].len()).map(move |x| (x, y)))
            .map(|(x, y)| {
                (-1..=1)
                    .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                    .filter(|&(dx, dy)| {
                        CHARS
                            .iter()
                            .try_fold((Some(x), Some(y)), |(x, y), c| {
                                let (x, y) = (x?, y?);
                                if grid.get(y)?.get(x)? == c {
                                    Some((x.checked_add_signed(dx), y.checked_add_signed(dy)))
                                } else {
                                    None
                                }
                            })
                            .is_some()
                    })
                    .count() as u32
            })
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
