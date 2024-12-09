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
use day05_lib::{is_right_order, load_input, middle, Input, DAY};

fn sum_right_order_middles(
    Input {
        pages,
        ordering_rules,
    }: Input,
) -> u32 {
    pages
        .into_iter()
        .filter(|pages| is_right_order(pages, &ordering_rules))
        .map(|pages| middle(pages.as_slice()))
        .sum()
}

fn main() {
    println!("{}", sum_right_order_middles(load_input(input(DAY, ""))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_sum_right_order_middles() {
        assert_eq!(
            sum_right_order_middles(load_input(test_input(DAY, ""))),
            143
        );
    }
}
