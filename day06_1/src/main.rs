use common::input;
use day06_lib::{load_input, ObstructionMap, DAY};
use std::collections::HashSet;

enum Direction {
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

struct Guard<'a> {
    direction: Direction,
    pos: (usize, usize),
    map: &'a ObstructionMap,
}

impl<'a> Guard<'a> {
    fn new(pos: (usize, usize), map: &'a ObstructionMap) -> Self {
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

    fn move_to_next_point(&mut self) -> bool {
        if let Some(new_pos) = self.next_point() {
            self.pos = new_pos;
            true
        } else {
            false
        }
    }

    fn turn_clockwise(&mut self) {
        self.direction = self.direction.turn_clockwise();
    }

    fn is_looking_at_obstacle(&self) -> bool {
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

fn count_visited_points((map, guard_pos): (ObstructionMap, (usize, usize))) -> usize {
    let mut visited = HashSet::from([guard_pos]);

    let mut guard = Guard::new(guard_pos, &map);
    loop {
        while guard.is_looking_at_obstacle() {
            guard.turn_clockwise();
        }

        if !guard.move_to_next_point() {
            break;
        }

        visited.insert(guard.pos);
    }

    visited.len()
}

fn main() {
    println!("{}", count_visited_points(load_input(input(DAY, ""))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

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
        let input = load_input(test_input(DAY, ""));
        assert_eq!(count_visited_points(input), 41);
    }
}
