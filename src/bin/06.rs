use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Default)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn next(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

impl From<Direction> for (isize, isize) {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '#' => false,
                    '^' => {
                        pos = (x, y);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut direction = Direction::default();

    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);
        let (Some(x), Some(y)) = ({
            let delta = <(isize, isize)>::from(direction);

            (
                pos.0.checked_add_signed(delta.0),
                pos.1.checked_add_signed(delta.1),
            )
        }) else {
            break;
        };

        let Some(c) = map.get(y).and_then(|row| row.get(x)) else {
            break;
        };

        match c {
            true => {
                pos = (x, y);
            }
            false => {
                direction.next();
            }
        }
    }

    Some(visited.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
