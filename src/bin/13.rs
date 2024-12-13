use itertools::Itertools;

advent_of_code::solution!(13);

struct Machine<N = u32> {
    button_a: (N, N),
    button_b: (N, N),
    prize: (N, N),
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|machine| {
                let (button_a, button_b, prize) = machine
                    .lines()
                    .map(|line| {
                        line.split_once(": ")
                            .unwrap()
                            .1
                            .split(", ")
                            .map(|n| n[2..].parse::<u32>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap();

                Machine {
                    button_a,
                    button_b,
                    prize,
                }
            })
            .filter_map(|machine| {
                let mul = u32::min(
                    machine.prize.0 / (machine.button_a.0 + machine.button_b.0),
                    machine.prize.1 / (machine.button_a.1 + machine.button_b.1),
                );

                for i in (0..=mul).rev() {
                    // Determine how much is remaining
                    let x = machine.prize.0 - ((machine.button_a.0 + machine.button_b.0) * i);
                    let y = machine.prize.1 - ((machine.button_a.1 + machine.button_b.1) * i);

                    let button_a = x % machine.button_a.0 == 0 && y % machine.button_a.1 == 0;
                    let button_b = x % machine.button_b.0 == 0 && y % machine.button_b.1 == 0;

                    let score = (3 + 1) * i;

                    if button_a {
                        let dx = x / machine.button_a.0;
                        let dy = y / machine.button_a.1;

                        if dx == dy {
                            return Some(score + (3 * dx));
                        }
                    }

                    if button_b {
                        let dx = x / machine.button_b.0;
                        let dy = y / machine.button_b.1;

                        if dx == dy {
                            return Some(score + dx);
                        }
                    }
                }

                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split("\n\n")
            .map(|machine| {
                let (button_a, button_b, prize) = machine
                    .lines()
                    .map(|line| {
                        line.split_once(": ")
                            .unwrap()
                            .1
                            .split(", ")
                            .map(|n| n[2..].parse::<u64>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap();

                Machine {
                    button_a,
                    button_b,
                    prize: (prize.0 + 10000000000000, prize.1 + 10000000000000),
                }
            })
            .filter_map(|machine| {
                let a = machine.button_a.0 as i64;
                let b = machine.button_b.0 as i64;
                let c = machine.prize.0 as i64;
                let d = machine.button_a.1 as i64;
                let e = machine.button_b.1 as i64;
                let f = machine.prize.1 as i64;

                let xa = c * e - b * f;
                let xb = a * e - b * d;

                let ya = -c * d + a * f;
                let yb = a * e - b * d;

                if xa % xb != 0 || ya % yb != 0 {
                    return None;
                }

                Some(((xa / xb) * 3 + ya / yb) as u64)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
