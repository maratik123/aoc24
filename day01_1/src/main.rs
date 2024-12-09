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
use day01_lib::{load_input, DAY};
use std::iter;

fn total_distance(mut a: Vec<u32>, mut b: Vec<u32>) -> u32 {
    a.sort_unstable();
    b.sort_unstable();
    iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn main() {
    let (a, b) = load_input(input(DAY, ""));
    println!("{}", total_distance(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_total_distance() {
        let (a, b) = load_input(test_input(DAY, ""));
        assert_eq!(total_distance(a, b), 11);
    }
}
