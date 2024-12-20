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
use day04_lib::{load_input, DAY};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
enum Direction {
    Left,
    Right,
    UpLeft,
    Up,
    UpRight,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    fn is_up(&self) -> bool {
        matches!(self, Direction::Up | Direction::UpRight | Direction::UpLeft)
    }

    fn is_right(&self) -> bool {
        matches!(
            self,
            Direction::Right | Direction::DownRight | Direction::UpRight
        )
    }

    fn is_down(&self) -> bool {
        matches!(
            self,
            Direction::Down | Direction::DownLeft | Direction::DownRight
        )
    }

    fn is_left(&self) -> bool {
        matches!(
            self,
            Direction::Left | Direction::DownLeft | Direction::UpLeft
        )
    }

    fn check_range(
        &self,
        len: usize,
        (i, j): (usize, usize),
        (height, width): (usize, usize),
    ) -> bool {
        (!self.is_up() || i >= len)
            && (!self.is_left() || j >= len)
            && (!self.is_down() || i < height - len)
            && (!self.is_right() || j < width - len)
    }

    fn pos(&self, (i, j): (usize, usize), pos: usize) -> (usize, usize) {
        match self {
            Direction::Up => (i - pos, j),
            Direction::UpRight => (i - pos, j + pos),
            Direction::Right => (i, j + pos),
            Direction::DownRight => (i + pos, j + pos),
            Direction::Down => (i + pos, j),
            Direction::DownLeft => (i + pos, j - pos),
            Direction::Left => (i, j - pos),
            Direction::UpLeft => (i - pos, j - pos),
        }
    }

    fn check_word(&self, (hay, line_size): (&[Vec<char>], usize), (i, j): (usize, usize)) -> bool {
        const WORD: [char; 3] = ['M', 'A', 'S'];
        self.check_range(WORD.len(), (i, j), (hay.len(), line_size))
            && WORD.iter().enumerate().all(|(pos, letter)| {
                let (i, j) = self.pos((i, j), pos + 1);
                hay[i][j] == *letter
            })
    }
}

fn xmas_count(inp @ (input, line_size): (&[Vec<char>], usize)) -> usize {
    input
        .iter()
        .inspect(|line| debug_assert_eq!(line.len(), line_size))
        .enumerate()
        .flat_map(|(i, line)| {
            (0..line_size)
                .filter(|j| line[*j] == 'X')
                .flat_map(move |j| Direction::iter().filter(move |dir| dir.check_word(inp, (i, j))))
        })
        .count()
}

fn main() {
    let (input, line_size) = load_input(input(DAY, ""));
    println!("{}", xmas_count((input.as_slice(), line_size)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_check_range() {
        for dir in Direction::iter() {
            for i in 0..10 {
                for j in 0..10 {
                    assert!(
                        dir.check_range(0, (i, j), (10, 10)),
                        "false at (i, j) = ({i}, {j})"
                    );
                }
            }
        }
    }

    #[test]
    fn test_check_word() {
        let (input, line_size) = load_input(test_input(DAY, ""));
        assert_eq!(input.len(), 10);
        assert_eq!(line_size, 10);
        let (i, j) = (0, 4);
        assert_eq!(input[i][j], 'X');
        assert!(Direction::DownRight.check_range(3, (i, j), (10, 10)));
        assert!(Direction::DownRight.check_word((input.as_slice(), line_size), (i, j)));
    }

    #[test]
    fn test_xmas_count() {
        let (input, line_size) = load_input(test_input(DAY, ""));
        assert_eq!(xmas_count((input.as_slice(), line_size)), 18);
    }
}
