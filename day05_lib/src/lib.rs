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

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "05";

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub ordering_rules: HashMap<u32, HashSet<u32>>,
    pub pages: Vec<Vec<u32>>,
}

pub fn load_input(path: impl AsRef<Path>) -> Input {
    let reader = BufReader::new(File::open(path).unwrap());

    let mut it = reader.lines();

    let mut ordering_rules: HashMap<_, HashSet<_>> = HashMap::new();

    loop {
        let line = it.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let (n1, n2) = line.trim_end().split_once('|').unwrap();
        let (n1, n2) = (n1.parse().unwrap(), n2.parse().unwrap());
        ordering_rules.entry(n1).or_default().insert(n2);
    }

    Input {
        ordering_rules,
        pages: it
            .map_while(Result::ok)
            .map(|line| {
                line.trim_end()
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect(),
    }
}

pub fn is_right_order(pages: &[u32], ordering_rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for (index, page) in pages.iter().enumerate() {
        let tail = &pages[index + 1..];
        if let Some(later_pages) = ordering_rules.get(page) {
            if !tail.iter().all(|page| later_pages.contains(page)) {
                return false;
            }
        } else if !tail.is_empty() {
            return false;
        }
    }
    true
}

pub fn middle(pages: &[u32]) -> u32 {
    pages[pages.len() / 2]
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
            Input {
                ordering_rules: HashMap::from([
                    (47, HashSet::from([53, 13, 61, 29])),
                    (97, HashSet::from([13, 61, 47, 29, 53, 75])),
                    (75, HashSet::from([29, 53, 47, 61, 13])),
                    (61, HashSet::from([13, 53, 29])),
                    (29, HashSet::from([13])),
                    (53, HashSet::from([29, 13])),
                ]),
                pages: vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47]
                ]
            }
        );
    }

    #[test]
    fn test_is_right_order() {
        let Input {
            ordering_rules,
            pages,
        } = load_input(test_input(DAY, ""));
        assert!(is_right_order(pages[0].as_slice(), &ordering_rules));
        assert!(is_right_order(pages[1].as_slice(), &ordering_rules));
        assert!(is_right_order(pages[2].as_slice(), &ordering_rules));
        assert!(!is_right_order(pages[3].as_slice(), &ordering_rules));
        assert!(!is_right_order(pages[4].as_slice(), &ordering_rules));
        assert!(!is_right_order(pages[5].as_slice(), &ordering_rules));
    }

    #[test]
    fn test_middle() {
        assert_eq!(61, middle(&[75, 47, 61, 53, 29]));
    }
}
