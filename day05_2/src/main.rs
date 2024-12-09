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
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn fix_order(pages: &mut [u32], ordering_rules: &HashMap<u32, HashSet<u32>>) {
    pages.sort_unstable_by(|a, b| {
        if a == b {
            Ordering::Equal
        } else if let Some(later_pages) = ordering_rules.get(a) {
            if later_pages.contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    });
}

fn sum_fix_order_middles(
    Input {
        pages,
        ordering_rules,
    }: Input,
) -> u32 {
    pages
        .into_iter()
        .filter(|pages| !is_right_order(pages, &ordering_rules))
        .map(|mut pages| {
            fix_order(pages.as_mut_slice(), &ordering_rules);
            pages
        })
        .map(|pages| middle(pages.as_slice()))
        .sum()
}

fn main() {
    println!("{}", sum_fix_order_middles(load_input(input(DAY, ""))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_fix_order() {
        let Input {
            pages: _,
            ordering_rules,
        } = load_input(test_input(DAY, ""));
        let tester = |mut pages: Vec<u32>, expected| {
            fix_order(pages.as_mut_slice(), &ordering_rules);
            assert_eq!(pages, expected);
        };
        tester(vec![75, 97, 47, 61, 53], vec![97, 75, 47, 61, 53]);
        tester(vec![], vec![]);
        tester(vec![61, 13, 29], vec![61, 29, 13]);
        tester(vec![97, 13, 75, 29, 47], vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_sum_fix_order_middles() {
        assert_eq!(sum_fix_order_middles(load_input(test_input(DAY, ""))), 123);
    }
}
