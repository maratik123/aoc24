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
use day02_lib::{is_safe_report, load_input, DAY};

fn count_safe_reports(rows: &[Vec<u32>]) -> usize {
    rows.iter().filter(|row| is_safe_report(row)).count()
}

fn main() {
    let rows = load_input(input(DAY, ""));
    println!("{}", count_safe_reports(&rows));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_is_safe_report() {
        assert!(is_safe_report(&[7, 6, 4, 2, 1]));
        assert!(is_safe_report(&[1, 2, 3]));
        assert!(!is_safe_report(&[1, 2, 1]));
        assert!(is_safe_report(&[1, 2]));
        assert!(!is_safe_report(&[1]));
    }

    #[test]
    fn test_count_safe_reports() {
        let rows = load_input(test_input(DAY, ""));
        assert_eq!(count_safe_reports(&rows), 2);
    }
}
