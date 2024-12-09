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
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "06";

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ObstructionMap {
    pub map: Vec<Vec<bool>>,
    pub line_size: usize,
}

pub fn load_input(path: impl AsRef<Path>) -> (ObstructionMap, (usize, usize)) {
    let mut map = vec![];
    let guard_position = OnceCell::new();
    let line_size = OnceCell::new();
    for (i, line) in BufReader::new(File::open(path).unwrap())
        .lines()
        .enumerate()
    {
        let line = line.unwrap();
        let mut row = vec![];
        if let Some(line_size) = line_size.get() {
            row.reserve(*line_size);
        }
        for (j, ch) in line.chars().enumerate() {
            row.push(ch == '#');
            if ch == '^' {
                guard_position.set((i, j)).unwrap();
            }
        }
        assert_eq!(line_size.get_or_init(|| row.len()), &row.len());
        map.push(row);
    }

    (
        ObstructionMap {
            map,
            line_size: line_size.get().copied().unwrap(),
        },
        guard_position.get().copied().unwrap(),
    )
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_point(
        &self,
        (i, j): (usize, usize),
        (height, width): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => i.checked_sub(1).map(|i| (i, j)),
            Direction::Down => Some(i + 1)
                .map(|i| (i, j))
                .filter(|&(i, j)| is_in_rect((i, j), (height, width))),
            Direction::Left => j.checked_sub(1).map(|j| (i, j)),
            Direction::Right => Some(j + 1)
                .map(|j| (i, j))
                .filter(|&(i, j)| is_in_rect((i, j), (height, width))),
        }
    }

    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub struct Guard<'a> {
    pub direction: Direction,
    pub pos: (usize, usize),
    map: &'a ObstructionMap,
}

impl<'a> Guard<'a> {
    pub fn new(pos: (usize, usize), map: &'a ObstructionMap) -> Self {
        Self {
            direction: Direction::Up,
            pos,
            map,
        }
    }

    fn next_point(&self) -> Option<(usize, usize)> {
        self.direction
            .next_point(self.pos, (self.map.map.len(), self.map.line_size))
    }

    pub fn move_to_next_point(&mut self) -> bool {
        if let Some(new_pos) = self.next_point() {
            self.pos = new_pos;
            true
        } else {
            false
        }
    }

    pub fn turn_clockwise(&mut self) {
        self.direction = self.direction.turn_clockwise();
    }

    pub fn is_looking_at_obstacle(&self) -> bool {
        if let Some((i, j)) = self.next_point() {
            self.map.map[i][j]
        } else {
            false
        }
    }
}

fn is_in_rect((i, j): (usize, usize), (height, width): (usize, usize)) -> bool {
    i < height && j < width
}

pub fn visited_points(
    (map, guard_pos): (&ObstructionMap, (usize, usize)),
) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::from([guard_pos]);

    let mut guard = Guard::new(guard_pos, map);
    loop {
        while guard.is_looking_at_obstacle() {
            guard.turn_clockwise();
        }

        if !guard.move_to_next_point() {
            break;
        }

        visited.insert(guard.pos);
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        let input = load_input(test_input(DAY, ""));
        const I: bool = false;
        const W: bool = true;
        assert_eq!(
            input,
            (
                ObstructionMap {
                    map: vec![
                        vec![I, I, I, I, W, I, I, I, I, I],
                        vec![I, I, I, I, I, I, I, I, I, W],
                        vec![I, I, I, I, I, I, I, I, I, I],
                        vec![I, I, W, I, I, I, I, I, I, I],
                        vec![I, I, I, I, I, I, I, W, I, I],
                        vec![I, I, I, I, I, I, I, I, I, I],
                        vec![I, W, I, I, I, I, I, I, I, I],
                        vec![I, I, I, I, I, I, I, I, W, I],
                        vec![W, I, I, I, I, I, I, I, I, I],
                        vec![I, I, I, I, I, I, W, I, I, I],
                    ],
                    line_size: 10
                },
                (6, 4)
            )
        );
    }

    #[test]
    fn test_next_point() {
        assert_eq!(Direction::Up.next_point((0, 0), (10, 10)), None);
        assert_eq!(Direction::Left.next_point((0, 0), (10, 10)), None);
        assert_eq!(Direction::Down.next_point((0, 0), (10, 10)), Some((1, 0)));
        assert_eq!(Direction::Right.next_point((0, 0), (10, 10)), Some((0, 1)));
        assert_eq!(Direction::Up.next_point((9, 9), (10, 10)), Some((8, 9)));
        assert_eq!(Direction::Left.next_point((9, 9), (10, 10)), Some((9, 8)));
        assert_eq!(Direction::Down.next_point((9, 9), (10, 10)), None);
        assert_eq!(Direction::Right.next_point((9, 9), (10, 10)), None);
    }

    #[test]
    fn test_count_visited_points() {
        let (map, guard_pos) = load_input(test_input(DAY, ""));
        assert_eq!(visited_points((&map, guard_pos)).len(), 41);
    }
}
