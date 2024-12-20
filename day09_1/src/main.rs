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

fn defrag<T>(mut disk_map: &mut [Option<T>]) {
    while let Some((leftmost_empty_index, rightmost_file_index)) = disk_map
        .iter()
        .position(|block| block.is_none())
        .and_then(|leftmost_empty_index| {
            disk_map
                .iter()
                .rposition(|block| block.is_some())
                .map(|rightmost_file_index| (leftmost_empty_index, rightmost_file_index))
        })
    {
        disk_map = &mut disk_map[leftmost_empty_index..=rightmost_file_index];
        if disk_map.is_empty() {
            break;
        }
        disk_map.swap(0, disk_map.len() - 1);
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
    fn test_defrag() {
        let mut disk_map = vec![None, Some(1), None, None, Some(2), None];
        defrag(&mut disk_map);
        assert_eq!(disk_map, vec![Some(2), Some(1), None, None, None, None]);

        let mut disk_map = load_input(test_input(DAY, "1"));
        defrag(&mut disk_map);
        assert_eq!(
            disk_map,
            vec![
                //"0099811188827773336446555566.............."
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]
        );

        assert_eq!(checksum(disk_map.as_slice()), 1928);
    }
}
