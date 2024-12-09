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

use std::cell::OnceCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "04";

pub fn load_input(path: impl AsRef<Path>) -> (Vec<Vec<char>>, usize) {
    let mut result = vec![];
    let mut buffer = String::new();
    let mut reader = BufReader::new(File::open(path).unwrap());
    let line_size = OnceCell::new();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        let trimmed = buffer.trim_end();
        if trimmed.is_empty() {
            break;
        }
        assert_eq!(line_size.get_or_init(|| trimmed.len()), &trimmed.len());
        result.push(trimmed.chars().collect());
        buffer.clear();
    }
    (result, line_size.get().copied().unwrap_or_default())
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
            (
                vec![
                    vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                    vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                    vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                    vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                    vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                    vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                    vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                    vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                    vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                    vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
                ],
                10
            )
        );
    }
}
