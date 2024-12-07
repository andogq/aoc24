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

struct Walker {
    position: Option<(u32, u32)>,
    direction: Direction,
    map: Vec<Vec<bool>>,
}

impl Walker {
    pub fn new(position: (u32, u32), map: Vec<Vec<bool>>) -> Self {
        Self {
            position: Some(position),
            direction: Direction::default(),
            map,
        }
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_wall(mut self, (x, y): (u32, u32)) -> Self {
        self.map[y as usize][x as usize] = false;
        self
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

impl Iterator for Walker {
    type Item = ((u32, u32), Direction);

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

        Some((position?, self.direction))
    }
}

fn parse(input: &str) -> ((u32, u32), Vec<Vec<bool>>) {
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
                        pos = (x as u32, y as u32);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (pos, map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pos, map) = parse(input);

    Some(
        Walker::new(pos, map)
            .fold(HashSet::new(), |mut visited, (pos, _)| {
                visited.insert(pos);

                visited
            })
            .len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_pos, map) = parse(input);

    let mut visited = HashSet::new();

    Some(
        Walker::new(start_pos, map.clone())
            .filter_map(|(pos, direction)| {
                // Mark this position as visited
                visited.insert(pos);

                let wall_pos = {
                    let direction = <(i32, i32)>::from(direction);

                    let x = pos.0.checked_add_signed(direction.0)?;
                    let y = pos.1.checked_add_signed(direction.1)?;

                    if y as usize >= map.len() || x as usize >= map[y as usize].len() {
                        return None;
                    }

                    (x, y)
                };

                if wall_pos == start_pos ||
                    // Make sure that wall isn't placed on a previous location
                    visited.contains(&wall_pos) ||
                    // Make sure that a wall isn't being placed over an existing wall
                    !map[wall_pos.1 as usize][wall_pos.0 as usize]
                {
                    return None;
                }

                Some((pos, direction, wall_pos))
            })
            .fold(HashSet::new(), |mut walls, (pos, direction, wall_pos)| {
                // Deduplicate the walls
                if walls.contains(&wall_pos) {
                    return walls;
                }

                let mut visited = HashSet::new();

                for (pos, dir) in Walker::new(pos, map.clone())
                    .with_wall(wall_pos)
                    .with_direction(direction.next())
                {
                    if visited.contains(&(pos, dir)) {
                        // Loop found, save this wall
                        walls.insert(wall_pos);
                        break;
                    }

                    visited.insert((pos, dir));
                }

                walls
            })
            .len() as u32,
    )
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
        assert_eq!(result, Some(6));
    }
}
