use std::fmt::Display;

advent_of_code::solution!(9);

#[derive(Debug)]
enum Block {
    File { id: u64, size: u64 },
    Free(u64),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free(size) => write!(f, "{}", ".".repeat(*size as usize)),
            Self::File { id, size } => write!(f, "{}", id.to_string().repeat(*size as usize)),
        }
    }
}

struct MoveAction {
    amount: u64,
    to: usize,
}

trait Solver {
    fn process_file(
        &mut self,
        size: u64,
        free_blocks: impl Iterator<Item = (usize, u64)>,
    ) -> Vec<MoveAction>;
}

#[derive(Default)]
struct FileSystem(Vec<Block>);

impl FileSystem {
    pub fn solve(&mut self, mut solver: impl Solver) {
        let mut i = self.0.len();
        let mut last_id = self.files().map(|(id, _)| id).max().unwrap() + 1;

        while i > 0 {
            // Move backwards to the next item
            i -= 1;

            // Only operate on files
            let Block::File { id, size } = self.0[i] else {
                continue;
            };

            // Don't re-process a file that's already been processed
            if id >= last_id {
                continue;
            }
            last_id = id;

            // Run the solver
            for MoveAction { amount, to } in solver.process_file(
                size,
                self.free_blocks().take_while(|(block_i, _)| *block_i < i),
            ) {
                let Block::Free(free_size) = &mut self.0[to] else {
                    unreachable!();
                };

                assert!(*free_size >= amount);

                // Update the free block
                *free_size -= amount;

                // Remove the free block if it's empty
                if *free_size == 0 {
                    self.0.remove(to);
                    i -= 1;
                }

                // Create or update an existing file
                match to.checked_sub(1).map(|prev| &mut self.0[prev]) {
                    Some(Block::File { id: prev_id, size }) if *prev_id == id => {
                        // Re-use an existing file
                        *size += amount;
                    }
                    _ => {
                        // Insert a new one
                        self.0.insert(to, Block::File { id, size: amount });

                        // Added something before the file, update the pointer
                        i += 1;
                    }
                }

                // Update the old file
                let Block::File { id: id2, size } = &mut self.0[i] else {
                    unreachable!();
                };

                assert_eq!(id, *id2);
                assert!(*size >= amount);

                // Shink the file
                *size -= amount;
                let size = *size;

                // Compensate with free space
                match self.0.get_mut(i + 1) {
                    Some(Block::Free(free_size)) => *free_size += amount,
                    _ => {
                        self.0.insert(i + 1, Block::Free(amount));
                    }
                }

                // Remove the file if it's empty
                if size == 0 {
                    self.0.remove(i);
                }
            }
        }
    }

    fn files(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        self.0.iter().filter_map(|block| {
            if let Block::File { id, size } = block {
                Some((*id, *size))
            } else {
                None
            }
        })
    }

    fn free_blocks(&self) -> impl Iterator<Item = (usize, u64)> + '_ {
        self.0.iter().enumerate().filter_map(|(i, block)| {
            if let Block::Free(size) = block {
                Some((i, *size))
            } else {
                None
            }
        })
    }

    pub fn checksum(&mut self) -> u64 {
        self.0
            .iter()
            .map(|block| match block {
                Block::File { id, size } => (*size, Some(*id)),
                Block::Free(size) => (*size, None),
            })
            .flat_map(|(size, id)| std::iter::repeat(id).take(size as usize))
            .enumerate()
            .filter_map(|(i, id)| Some(i as u64 * id?))
            .sum()
    }
}

impl<S> From<S> for FileSystem
where
    S: AsRef<str>,
{
    fn from(input: S) -> Self {
        input
            .as_ref()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap().into())
            .enumerate()
            .fold(Self::default(), |mut fs, (i, size)| {
                // Don't add any zero-sized blocks
                if size == 0 {
                    return fs;
                }

                fs.0.push(if i % 2 == 0 {
                    // File
                    Block::File {
                        id: i as u64 / 2,
                        size,
                    }
                } else {
                    // Free
                    Block::Free(size)
                });

                fs
            })
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|b| b.fmt(f))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    struct Part1;
    impl Solver for Part1 {
        fn process_file(
            &mut self,
            mut size: u64,
            free_blocks: impl Iterator<Item = (usize, u64)>,
        ) -> Vec<MoveAction> {
            free_blocks
                .map_while(|(i, block_size)| {
                    if size == 0 {
                        return None;
                    }

                    let move_amount = block_size.min(size);
                    size -= move_amount;

                    Some(MoveAction {
                        to: i,
                        amount: move_amount,
                    })
                })
                .collect()
        }
    }

    let mut fs = FileSystem::from(input);
    fs.solve(Part1);
    Some(fs.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    struct Part2;
    impl Solver for Part2 {
        fn process_file(
            &mut self,
            size: u64,
            mut free_blocks: impl Iterator<Item = (usize, u64)>,
        ) -> Vec<MoveAction> {
            free_blocks
                .find(|(_, free)| *free >= size)
                .map(|(i, _)| MoveAction {
                    to: i,
                    amount: size,
                })
                .into_iter()
                .collect()
        }
    }

    let mut fs = FileSystem::from(input);
    fs.solve(Part2);
    Some(fs.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
