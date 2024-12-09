/*
  Copyright 2024 Marat Bukharov

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

use common::input;
use day03_lib::{load_input, DAY};
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mul(u32, u32),
}

impl Instruction {
    fn eval(&self) -> u32 {
        match self {
            Instruction::Mul(n1, n2) => n1 * n2,
        }
    }
}

fn calc_sum(instructions: &[Instruction]) -> u32 {
    instructions.iter().map(|i| i.eval()).sum()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap());
    RE.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| Instruction::Mul(n1.parse().unwrap(), n2.parse().unwrap()))
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
        let input = load_input(test_input(DAY, "1"));
        let input = String::from_utf8_lossy(input.as_slice());
        assert_eq!(
            parse_input(input.as_ref()),
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5),
            ]
        );
    }

    #[test]
    fn test_calc_sum() {
        let input = load_input(test_input(DAY, "1"));
        let input = String::from_utf8_lossy(input.as_slice());
        assert_eq!(calc_sum(parse_input(input.as_ref()).as_slice()), 161)
    }
}
