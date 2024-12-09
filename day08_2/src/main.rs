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
use day08_lib::{antinodes_count, load_input, signed_overflowing_sub, DAY};

fn antinodes(
    [(i1, j1), (i2, j2)]: [(usize, usize); 2],
    (height, width): (usize, usize),
) -> Vec<(usize, usize)> {
    assert_ne!((i1, j1), (i2, j2));
    let mut result = vec![(i1, j1), (i2, j2)];
    // n1 - n2 = d, a1 - n1 = d, n2 - a2 = d
    let (diff_i, diff_j) = (
        signed_overflowing_sub(i1, i2),
        signed_overflowing_sub(j1, j2),
    );
    let mut ext = |diff_i: isize, diff_j: isize, i: usize, j: usize| {
        result.extend((1..).map_while(|n| {
            i.checked_add_signed(n * diff_i)
                .filter(|&i| i < height)
                .and_then(|i| {
                    j.checked_add_signed(n * diff_j)
                        .filter(|&j| j < width)
                        .map(|j| (i, j))
                })
        }));
    };
    ext(diff_i, diff_j, i1, j1);
    ext(-diff_i, -diff_j, i2, j2);
    result
}

fn main() {
    println!("{}", antinodes_count(load_input(input(DAY, "")), antinodes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;
    use std::collections::HashSet;

    #[test]
    fn test_antinodes() {
        assert_eq!(
            HashSet::from_iter(antinodes([(0, 0), (1, 3)], (10, 10))),
            HashSet::from([(2, 6), (3, 9), (0, 0), (1, 3)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(2, 1), (1, 3)], (10, 10))),
            HashSet::from([(0, 5), (2, 1), (1, 3)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(0, 0), (2, 1)], (10, 10))),
            HashSet::from([(4, 2), (6, 3), (8, 4), (0, 0), (2, 1)])
        );
    }

    #[test]
    fn test_antinodes_count() {
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "5")), antinodes),
            9
        );
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "4")), antinodes),
            34
        );
    }
}
