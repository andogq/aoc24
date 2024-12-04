advent_of_code::solution!(4);

const CHARS: &[char] = &['X', 'M', 'A', 'S'];
const XMAS_BOUNDS: usize = 2;

///   0 1 2
/// 0 M   S
/// 1   A
/// 2 M   S
const XMAS: &[((usize, usize), char)] = &[
    ((0, 0), 'M'),
    ((1, 1), 'A'),
    ((2, 2), 'S'),
    ((0, 2), 'M'),
    ((2, 0), 'S'),
];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        // Pull out every (x, y) combo
        (0..grid.len())
            .flat_map(|y| (0..grid[0].len()).map(move |x| (x, y)))
            .map(|(x, y)| {
                // Check in each dx/dy direction
                (-1..=1)
                    .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                    // For each coord and direction, ensure the characters line up
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
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        (0..grid.len())
            .flat_map(|y| (0..grid[0].len()).map(move |x| (x, y)))
            .map(|(x, y)| {
                // Weird way of generating a bit pattern to 'rotate' the X
                (0..4)
                    .map(|i| (i & 1 == 1, i & 2 == 2))
                    .map(|(flip_x, flip_y)| {
                        XMAS.iter().map(move |((mut x, mut y), c)| {
                            if flip_x == flip_y {
                                std::mem::swap(&mut x, &mut y);
                            }

                            (
                                (
                                    if flip_x { XMAS_BOUNDS - x } else { x },
                                    if flip_y { XMAS_BOUNDS - y } else { y },
                                ),
                                *c,
                            )
                        })
                    })
                    .map(|mut xmas| {
                        xmas.all(|((dx, dy), c)| {
                            let Some(test) = grid.get(y + dy).and_then(|row| row.get(x + dx))
                            else {
                                return false;
                            };

                            *test == c
                        })
                    })
                    .filter(|valid| *valid)
                    .count() as u32
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
