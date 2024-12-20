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

pub const DAY: &str = "01";

fn parse_line(line: &str) -> Option<(u32, u32)> {
    let mut it = line.split_whitespace().map_while(|s| s.parse().ok());
    Some((it.next()?, it.next()?))
}

pub fn load_input(path: impl AsRef<Path>) -> (Vec<u32>, Vec<u32>) {
    let file = BufReader::new(File::open(path).unwrap());
    file.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .filter_map(|line| parse_line(line.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("3   4"), Some((3, 4)));
        assert_eq!(parse_line("35134   63205"), Some((35134, 63205)));
    }

    #[test]
    fn test_load_input() {
        assert_eq!(
            load_input(test_input(DAY, "")),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }
}
