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

impl Block {
    pub fn shrink(&mut self, shrink_size: u64) -> Option<u64> {
        let size = match self {
            Block::File { size, .. } => size,
            Block::Free(size) => size,
        };

        *size = size.checked_sub(shrink_size)?;

        Some(*size)
    }

    pub fn free(&mut self) {
        let Self::File { size, .. } = self else {
            return;
        };

        *self = Self::Free(*size);
    }
}

#[derive(Default)]
struct FileSystem(Vec<Block>);

impl FileSystem {
    pub fn defrag(&mut self) {
        let mut file_i = self.0.len();

        while file_i > 0 {
            // Move backwards to next item
            file_i -= 1;

            let Block::File { id, size } = self.0[file_i] else {
                continue;
            };

            for _ in 0..size {
                // Find a free block
                let Some(free_i) = (0..file_i).find(|i| matches!(self.0[*i], Block::Free(_)))
                else {
                    // No more free blocks
                    return;
                };

                // Move can take place, update values before indexes are invalidated
                self.0[file_i].shrink(1).unwrap(); // Reduce file size
                let new_free_size = self.0[free_i].shrink(1).unwrap(); // Reduce free size

                if new_free_size == 0 {
                    // Free block is empty, remove it
                    self.0.remove(free_i);

                    // Deleted something before the file, update the pointer
                    file_i -= 1;
                }

                match free_i.checked_sub(1).map(|i| &mut self.0[i]) {
                    Some(Block::File { id: prev_id, size }) if *prev_id == id => {
                        // Re-use an existing block
                        *size += 1;
                    }
                    _ => {
                        // Insert a new one
                        self.0.insert(free_i, Block::File { id, size: 1 });

                        // Added something before the file, update the pointer
                        file_i += 1;
                    }
                }
            }

            // File block is now empty, remove it
            self.0.remove(file_i);
        }
    }

    pub fn better_defrag(&mut self) {
        let mut file_i = self.0.len();
        let mut last_id = self
            .0
            .iter()
            .rev()
            .find_map(|b| {
                if let Block::File { id, .. } = b {
                    Some(*id)
                } else {
                    None
                }
            })
            .unwrap()
            + 1;

        while file_i > 0 {
            // Move backwards to next item
            file_i -= 1;

            // Fetch a file
            let Block::File { id, size } = self.0[file_i] else {
                continue;
            };

            if id >= last_id {
                continue;
            }
            last_id = id;

            // Find a block
            let Some((free_i, free_size)) =
                self.0
                    .iter_mut()
                    .take(file_i)
                    .enumerate()
                    .find_map(|(i, block)| match block {
                        Block::Free(free_size) if *free_size >= size => Some((i, free_size)),
                        _ => None,
                    })
            else {
                // No suitable free blocks found
                continue;
            };

            // Update the free block size
            *free_size -= size;
            let free_size = *free_size;

            // Free the space used by the file
            self.0[file_i].free();

            // Remove the empty free block, if required
            if free_size == 0 {
                self.0.remove(free_i);
                file_i -= 1;
            }

            // Insert the moved file block
            self.0.insert(free_i, Block::File { id, size });
            file_i += 1;
        }
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
    let mut fs = FileSystem::from(input);
    fs.defrag();
    Some(fs.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fs = FileSystem::from(input);
    fs.better_defrag();
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
