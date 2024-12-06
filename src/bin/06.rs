use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl From<Direction> for (i32, i32) {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

struct Walker<'map> {
    position: Option<(u32, u32)>,
    direction: Direction,
    map: &'map Vec<Vec<bool>>,
}

impl<'map> Walker<'map> {
    pub fn new(position: (u32, u32), map: &'map Vec<Vec<bool>>) -> Self {
        Self {
            position: Some(position),
            direction: Direction::default(),
            map,
        }
    }

    fn next_position(&self) -> Option<(u32, u32)> {
        let direction = <(i32, i32)>::from(self.direction);

        let x = self.position?.0.checked_add_signed(direction.0)?;
        let y = self.position?.1.checked_add_signed(direction.1)?;

        if y as usize >= self.map.len() || x as usize >= self.map[y as usize].len() {
            return None;
        }

        Some((x, y))
    }
}

impl Iterator for Walker<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        // Emit this position
        let position = self.position;

        // Determine the next position
        let next_position = self.next_position();

        if let Some((x, y)) = next_position {
            // Look at the next position to work out where to go
            match self.map[y as usize][x as usize] {
                true => {
                    // Safe to advance
                }
                false => {
                    // Will hit a wall, so turn...
                    self.direction = self.direction.next();

                    // ...and try again
                    return self.next();
                }
            }
        }

        // Prepare for next run
        self.position = next_position;

        position
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

    Some(
        Walker::new((pos.0 as u32, pos.1 as u32), &map)
            .fold(HashSet::new(), |mut visited, pos| {
                visited.insert(pos);

                visited
            })
            .len() as u32,
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
