use std::iter::Peekable;

use itertools::{EitherOrBoth, Itertools};

advent_of_code::solution!(3);

struct MemoryScanner<I: Iterator<Item = char>> {
    iter: Peekable<I>,
}

#[derive(Default, Debug, Clone, Copy)]
enum ScannerState {
    #[default]
    Start,
    Number(usize),
    Comma(usize),
}

impl<I: Iterator<Item = char>> MemoryScanner<I> {
    pub fn new(chars: impl IntoIterator<Item = char, IntoIter = I>) -> Self {
        Self {
            iter: chars.into_iter().peekable(),
        }
    }

    fn eat_string(&mut self, s: &str) -> Option<bool> {
        for next in s.chars().zip_longest(&mut self.iter).take(s.len()) {
            match next {
                EitherOrBoth::Both(a, b) if a == b => {
                    continue;
                }
                EitherOrBoth::Both(..) => {
                    // Expected string longer than iterator, or the chars don't match
                    return Some(false);
                }
                EitherOrBoth::Right(_) => {
                    // Iterator longer than expected string
                    return Some(true);
                }
                EitherOrBoth::Left(_) => {
                    // Iterator exhausted
                    return None;
                }
            }
        }

        // Exact size and match
        Some(true)
    }
}

impl<I: Iterator<Item = char>> Iterator for MemoryScanner<I> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = ScannerState::default();

        let mut next_number = 1;

        loop {
            match state {
                ScannerState::Start => {
                    if self.eat_string("mul(")? {
                        state = ScannerState::Number(0);
                        next_number = 1;
                    }
                }
                ScannerState::Number(i @ (0 | 1)) => {
                    let mut number = None::<u32>;

                    while let Some(n) = self
                        .iter
                        .next_if(|c| c.is_ascii_digit())
                        .map(|c| c.to_digit(10).unwrap())
                    {
                        number = Some((number.unwrap_or_default() * 10) + n);
                    }

                    let Some(number) = number else {
                        state = ScannerState::Start;
                        continue;
                    };

                    next_number *= number;
                    state = ScannerState::Comma(i + 1);
                }
                ScannerState::Number(_) => {
                    // Overran the available numbers
                    state = ScannerState::Start;
                }
                ScannerState::Comma(1) => {
                    if let Some(',') = self.iter.next() {
                        state = ScannerState::Number(1);
                    } else {
                        state = ScannerState::Start;
                    }
                }
                ScannerState::Comma(_) => {
                    if let Some(')') = self.iter.next() {
                        return Some(next_number);
                    } else {
                        state = ScannerState::Start;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
enum StateMachine2 {
    #[default]
    Mul,
    Dont,
    ParamA,
    ParamB {
        param_a: u32,
    },
}

impl StateMachine2 {
    pub fn step(&mut self, iter: &mut impl Iterator<Item = char>) -> Option<(u32, u32)> {
        match self {
            StateMachine2::Mul => {
                let Some((chars, next)) = (match iter.next() {
                    Some('m') => Some((['u', 'l', '('].as_slice(), Self::ParamA)),
                    Some('d') => Some((['o', 'n', '\'', 't', '(', ')'].as_slice(), Self::Dont)),
                    _ => None,
                }) else {
                    *self = Self::Mul;
                    return None;
                };

                for &c in chars {
                    if iter.next().map(|next_c| c != next_c).unwrap_or(true) {
                        *self = Self::Mul;
                        return None;
                    }
                }

                *self = next;

                None
            }
            StateMachine2::Dont => {
                for c in ['d', 'o', '(', ')'] {
                    if iter.next().map(|next_c| c != next_c).unwrap_or(true) {
                        *self = Self::Dont;
                        return None;
                    }
                }

                *self = Self::Mul;

                None
            }
            StateMachine2::ParamA => {
                let mut num = 0;

                loop {
                    match iter.next() {
                        Some(c) if c.is_ascii_digit() => {
                            num = (num * 10) + c.to_digit(10).unwrap();
                        }
                        Some(',') => {
                            *self = Self::ParamB { param_a: num };
                            return None;
                        }
                        _ => {
                            *self = Self::Mul;
                            return None;
                        }
                    }
                }
            }
            StateMachine2::ParamB { param_a } => {
                let mut num = 0;

                loop {
                    match iter.next() {
                        Some(c) if c.is_ascii_digit() => {
                            num = (num * 10) + c.to_digit(10).unwrap();
                        }
                        Some(')') | None => {
                            let answer = (*param_a, num);
                            *self = Self::Mul;
                            return Some(answer);
                        }
                        _ => {
                            *self = Self::Mul;
                            return None;
                        }
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(MemoryScanner::new(input.chars().peekable()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.chars().peekable();

    let mut machine = StateMachine2::default();
    let mut answer = 0;
    while iter.peek().is_some() {
        if let Some((a, b)) = machine.step(&mut iter) {
            answer += a * b;
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
