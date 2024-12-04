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
            Instruction::Mul(n1, n2) if enabled => (Some(n1 * n2), enabled),
            Instruction::Mul(_, _) => (None, enabled),
            Instruction::Do => (None, true),
            Instruction::Dont => (None, false),
        }
    }
}

fn calc_sum(insns: &[Instruction]) -> u32 {
    insns
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
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap());
    static RE_MUL: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap());
    static RE_DO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"do\(\)").unwrap());
    static RE_DONT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"don't\(\)").unwrap());
    RE.find_iter(input)
        .map(|m| m.as_str())
        .map(|m| {
            if let Some((_, [n1, n2])) = RE_MUL.captures(m).map(|c| c.extract()) {
                Instruction::Mul(n1.parse().unwrap(), n2.parse().unwrap())
            } else if RE_DO.is_match(m) {
                Instruction::Do
            } else if RE_DONT.is_match(m) {
                Instruction::Dont
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
    use day03_lib::{load_input, DAY};

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
