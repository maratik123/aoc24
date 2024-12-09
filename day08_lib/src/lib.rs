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
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::RandomState;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "08";

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub antennas: HashMap<char, HashSet<(usize, usize)>>,
    pub size: (usize, usize),
}

pub fn load_input(path: impl AsRef<Path>) -> Input {
    let mut antennas: HashMap<_, HashSet<_>> = HashMap::new();
    let width = OnceCell::new();
    let mut i = 0;

    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        let mut j = 0;
        for ch in line.chars() {
            if matches!(ch, 'A'..='Z' | 'a'..='z' | '0'..='9') {
                antennas.entry(ch).or_default().insert((i, j));
            }
            j += 1;
        }
        assert_eq!(width.get_or_init(|| j), &j);
        i += 1;
    }

    Input {
        antennas,
        size: (i, width.get().copied().unwrap_or_default()),
    }
}

pub fn signed_overflowing_sub(n1: usize, n2: usize) -> isize {
    let (res, overflowed) = n1.overflowing_sub(n2);
    let res = res as isize;
    assert_eq!(overflowed, res < 0);
    res
}

pub fn antinodes_count<F, R>(Input { antennas, size }: Input, antinodes: F) -> usize
where
    F: Fn([(usize, usize); 2], (usize, usize)) -> R,
    R: IntoIterator<Item = (usize, usize)>,
{
    HashSet::<_, RandomState>::from_iter(
        antennas
            .values()
            .filter(|antennas| antennas.len() > 1)
            .flat_map(|antennas| {
                antennas.iter().flat_map(|antenna1| {
                    antennas
                        .iter()
                        .filter(move |&antenna2| antenna1 != antenna2)
                        .flat_map(|antenna2| antinodes([*antenna1, *antenna2], size))
                })
            }),
    )
    .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        assert_eq!(
            load_input(test_input(DAY, "4")),
            Input {
                antennas: HashMap::from([
                    ('0', HashSet::from([(1, 8), (2, 5), (3, 7), (4, 4)])),
                    ('A', HashSet::from([(5, 6), (8, 8), (9, 9)]))
                ]),
                size: (12, 12),
            }
        );
    }

    #[test]
    fn test_signed_overflowing_sub() {
        assert_eq!(signed_overflowing_sub(1, 2), -1);
        assert_eq!(signed_overflowing_sub(2, 1), 1);
    }
}
