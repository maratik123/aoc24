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
use day07_lib::{load_input, total_sum, Eval, DAY};
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Op {
    Add,
    Multiply,
}

impl Eval for Op {
    fn eval(&self, left: u64, right: u32) -> Option<u64> {
        let right = right as u64;
        match self {
            Op::Add => left.checked_add(right),
            Op::Multiply => left.checked_mul(right),
        }
    }
}

fn main() {
    println!("{}", total_sum::<Op>(load_input(input(DAY, "")).as_slice()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;
    use day07_lib::dfs;

    #[test]
    fn test_dfs() {
        assert!(dfs::<Op>(10, &[10]));
        assert!(dfs::<Op>(190, &[10, 19]));
        assert!(dfs::<Op>(3267, &[81, 40, 27]));
    }

    #[test]
    fn test_total_sum() {
        assert_eq!(
            total_sum::<Op>(load_input(test_input(DAY, "")).as_slice()),
            3749
        );
    }
}
