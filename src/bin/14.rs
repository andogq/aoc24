use itertools::Itertools;

advent_of_code::solution!(14);

const DURATION: u64 = 100;

#[cfg(not(test))]
mod size {
    pub const HEIGHT: u64 = 103;
    pub const WIDTH: u64 = 101;
}

#[cfg(test)]
mod size {
    pub const HEIGHT: u64 = 7;
    pub const WIDTH: u64 = 11;
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (p, v) = line
                    .split_whitespace()
                    .map(|section| {
                        section[2..]
                            .split(",")
                            .map(|n| n.parse::<i64>().unwrap())
                            .collect_tuple::<(_, _)>()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap();

                ((p.0 as u64, p.1 as u64), v)
            })
            .map(|(mut position, velocity)| {
                (0..DURATION).for_each(|_| {
                    position = (
                        (position.0 + size::WIDTH)
                            .checked_add_signed(velocity.0)
                            .unwrap()
                            % size::WIDTH,
                        (position.1 + size::HEIGHT)
                            .checked_add_signed(velocity.1)
                            .unwrap()
                            % size::HEIGHT,
                    );
                });

                position
            })
            .flat_map(|(x, y)| {
                if size::WIDTH % 2 == 1 && x == size::WIDTH / 2 {
                    return None;
                }

                if size::HEIGHT % 2 == 1 && y == size::HEIGHT / 2 {
                    return None;
                }

                // Determine the quadrant
                let x_quad = (x < size::WIDTH / 2) as usize;
                let y_quad = (y < size::HEIGHT / 2) as usize;

                Some((x_quad << 1) + y_quad)
            })
            .fold([0; 4], |mut quads, quad| {
                quads[quad] += 1;

                quads
            })
            .into_iter()
            .product::<u32>(),
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
