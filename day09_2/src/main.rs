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
use day09_lib::{checksum, load_input, DAY};
use std::collections::BTreeSet;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct SpaceMapEntry {
    size: u32,
    pos: u32,
}

fn free_space_map<T>(disk_map: &[Option<T>]) -> BTreeSet<SpaceMapEntry> {
    let mut free_space_map = BTreeSet::new();
    let mut last_pos = 0;
    while let Some(pos) = {
        disk_map[last_pos..]
            .iter()
            .position(|block| block.is_none())
            .map(|pos| last_pos + pos)
    } {
        let size = match disk_map[pos..].iter().position(|block| block.is_some()) {
            Some(pos) => pos,
            None => disk_map.len() - pos,
        };
        if size > 0 {
            free_space_map.insert(SpaceMapEntry {
                size: size as u32,
                pos: pos as u32,
            });
        }
        last_pos = pos + size;
    }
    free_space_map
}

fn defrag<T: Copy>(mut disk_map: &mut [Option<T>]) {
    if let Some(last_file_pos) = disk_map.iter().rposition(|block| block.is_some()) {
        let mut free_space_map = free_space_map(disk_map);
        let last_id = disk_map[last_file_pos].unwrap();
    }
}

fn main() {
    let mut disk_map = load_input(input(DAY, ""));
    defrag(&mut disk_map);
    println!("{}", checksum(disk_map.as_slice()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_free_space_map() {
        let disk_map = load_input(test_input(DAY, "2"));
        assert_eq!(
            free_space_map(disk_map.as_slice()),
            BTreeSet::from([
                SpaceMapEntry { size: 2, pos: 1 },
                SpaceMapEntry { size: 4, pos: 6 }
            ])
        );

        let disk_map = load_input(test_input(DAY, "1"));
        assert_eq!(
            free_space_map(disk_map.as_slice()),
            BTreeSet::from([
                SpaceMapEntry { size: 3, pos: 2 },
                SpaceMapEntry { size: 3, pos: 8 },
                SpaceMapEntry { size: 3, pos: 12 },
                SpaceMapEntry { size: 1, pos: 18 },
                SpaceMapEntry { size: 1, pos: 21 },
                SpaceMapEntry { size: 1, pos: 26 },
                SpaceMapEntry { size: 1, pos: 31 },
                SpaceMapEntry { size: 1, pos: 35 }
            ])
        );
    }
}
