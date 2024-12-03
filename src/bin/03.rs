advent_of_code::solution!(3);

#[derive(Default)]
enum StateMachine1 {
    #[default]
    Mul,
    ParamA,
    ParamB {
        param_a: u32,
    },
}

impl StateMachine1 {
    pub fn step(&mut self, iter: &mut impl Iterator<Item = char>) -> Option<(u32, u32)> {
        match self {
            StateMachine1::Mul => {
                for c in ['m', 'u', 'l', '('] {
                    if iter.next().map(|next_c| c != next_c).unwrap_or(true) {
                        *self = Self::Mul;
                        return None;
                    }
                }

                *self = Self::ParamA;

                None
            }
            StateMachine1::ParamA => {
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
            StateMachine1::ParamB { param_a } => {
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
    let mut iter = input.chars().peekable();

    let mut machine = StateMachine1::default();
    let mut answer = 0;
    while iter.peek().is_some() {
        if let Some((a, b)) = machine.step(&mut iter) {
            answer += a * b;
        }
    }

    Some(answer)
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
