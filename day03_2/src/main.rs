use common::input;
use day03_lib::{load_input, DAY};
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    fn eval(&self, enabled: bool) -> (Option<u32>, bool) {
        match self {
            Instruction::Mul(n1, n2) => (if enabled { Some(n1 * n2) } else { None }, enabled),
            Instruction::Do => (None, true),
            Instruction::Dont => (None, false),
        }
    }
}

fn calc_sum(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .scan(true, |old_state, instr| {
            let (val, new_state) = instr.eval(*old_state);
            *old_state = new_state;
            Some(val)
        })
        .flatten()
        .sum()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap()
    });
    RE.captures_iter(input)
        .map(|c| {
            if c.get(1).is_some() {
                Instruction::Do
            } else if c.get(2).is_some() {
                Instruction::Dont
            } else if let Some((n1, n2)) = c.get(3).and_then(|m1| c.get(4).map(|m2| (m1, m2))) {
                Instruction::Mul(n1.as_str().parse().unwrap(), n2.as_str().parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn main() {
    let input = load_input(input(DAY, ""));
    let input = String::from_utf8_lossy(input.as_ref());
    println!("{}", calc_sum(parse_input(input.as_ref()).as_slice()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_parse_input() {
        let input = load_input(test_input(DAY, "2"));
        let input = String::from_utf8_lossy(input.as_slice());
        assert_eq!(
            parse_input(input.as_ref()),
            vec![
                Instruction::Mul(2, 4),
                Instruction::Dont,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Do,
                Instruction::Mul(8, 5),
            ]
        );
    }

    #[test]
    fn test_calc_sum() {
        let input = load_input(test_input(DAY, "2"));
        let input = String::from_utf8_lossy(input.as_slice());
        assert_eq!(calc_sum(parse_input(input.as_ref()).as_slice()), 48)
    }
}
