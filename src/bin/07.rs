use std::collections::VecDeque;

advent_of_code::solution!(7);

fn check_mul(n: u128, total: u128) -> Option<u128> {
    (n != 0 && total % n == 0).then(|| total / n)
}

fn check_add(n: u128, total: u128) -> Option<u128> {
    total.checked_sub(n)
}

fn check_append(n: u128, total: u128) -> Option<u128> {
    // Build a base 10 mask of the number
    //   12  ->  100 (tens)
    //   123 -> 1000 (hundreds)
    // This mask can be used to extract a portion of the total.
    let mask = 10u128.pow((n as f64).log10() as u32 + 1);

    (total % mask == n).then_some(total / mask)
}

fn solve(input: &str, operator_checks: impl AsRef<[fn(u128, u128) -> Option<u128>]>) -> u128 {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(total, numbers)| {
            (
                total.parse().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    // Operators are applied left to right, so reverse them right to left
                    .rev(),
            )
        })
        .filter_map(|(original_total, numbers)| {
            let mut search = VecDeque::from_iter([(original_total, numbers)]);

            while let Some((total, mut numbers)) = search.pop_front() {
                let Some(n) = numbers.next() else {
                    // Total has been found!
                    if total == 0 {
                        return Some(original_total);
                    }

                    continue;
                };

                search.extend(
                    operator_checks
                        .as_ref()
                        .iter()
                        .flat_map(|check| check(n, total))
                        .map(|total| (total, numbers.clone())),
                );
            }

            None
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u128> {
    Some(solve(input, [check_mul, check_add]))
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(solve(input, [check_mul, check_add, check_append]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
