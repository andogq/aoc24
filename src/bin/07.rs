use std::collections::VecDeque;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(total, numbers)| {
                (
                    total.parse::<u64>().unwrap(),
                    numbers
                        .split_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .rev(),
                )
            })
            .filter_map(|(original_total, numbers)| {
                let mut search = VecDeque::from_iter([(original_total, numbers)]);

                while let Some((total, mut numbers)) = search.pop_front() {
                    let Some(n) = numbers.next() else {
                        if total == 0 {
                            return Some(original_total);
                        }

                        continue;
                    };

                    if total % n == 0 {
                        search.push_back((total / n, numbers.clone()));
                    }

                    if let Some(total) = total.checked_sub(n) {
                        search.push_back((total, numbers));
                    }
                }

                None
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
