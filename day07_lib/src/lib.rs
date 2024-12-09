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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use strum::IntoEnumIterator;

pub const DAY: &str = "07";

pub fn load_input(path: impl AsRef<Path>) -> Vec<(u64, Vec<u32>)> {
    let input = BufReader::new(File::open(path).unwrap());
    input
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let (result, args) = line.split_once(':').unwrap();
            (
                result.parse().unwrap(),
                args.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub trait Eval {
    fn eval(&self, left: u64, right: u32) -> Option<u64>;
}

pub fn dfs<T>(end: u64, args: &[u32]) -> bool
where
    T: Eval + IntoEnumIterator,
{
    if let [head, tail @ ..] = args {
        let mut s = vec![(*head as u64, tail)];
        while let Some((val, slice)) = s.pop() {
            if let [head, tail @ ..] = slice {
                for op in T::iter() {
                    if let Some(new_val) = op.eval(val, *head) {
                        s.push((new_val, tail));
                    }
                }
            } else if val == end {
                return true;
            }
        }
    }
    false
}

pub fn total_sum<T>(input: &[(u64, Vec<u32>)]) -> u64
where
    T: Eval + IntoEnumIterator,
{
    input
        .iter()
        .filter(|(end, args)| dfs::<T>(*end, args))
        .map(|(end, _)| end)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        let input = load_input(test_input(DAY, ""));
        assert_eq!(
            input,
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ]
        );
    }
}
