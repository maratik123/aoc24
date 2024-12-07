use common::input;
use day06_lib::{load_input, visited_points, Guard, ObstructionMap, DAY};
use std::collections::HashSet;

fn is_loop((map, guard_pos): (&ObstructionMap, (usize, usize))) -> bool {
    let mut turning_point = HashSet::new();
    let mut guard = Guard::new(guard_pos, map);
    'outer: loop {
        while guard.is_looking_at_obstacle() {
            if !turning_point.insert((guard.direction, guard.pos)) {
                break 'outer true;
            }
            guard.turn_clockwise();
        }

        if !guard.move_to_next_point() {
            break false;
        }
    }
}

fn count_loops((mut map, guard_pos): (ObstructionMap, (usize, usize))) -> usize {
    let mut visited = visited_points((&map, guard_pos));
    visited.remove(&guard_pos);

    visited
        .into_iter()
        .filter(|&(i, j)| {
            map.map[i][j] = true;
            let result = is_loop((&map, guard_pos));
            map.map[i][j] = false;
            result
        })
        .count()
}

fn main() {
    println!("{}", count_loops(load_input(input(DAY, ""))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_is_loop() {
        let (map, guard_pos) = load_input(test_input(DAY, ""));
        assert!(!is_loop((&map, guard_pos)));

        let test_looping = |i, j| {
            let mut mod_map = map.clone();
            let line: &mut Vec<_> = &mut mod_map.map[i];
            line[j] = true;
            assert!(is_loop((&mod_map, guard_pos)));
        };

        test_looping(6, 3);
        test_looping(7, 6);
        test_looping(7, 7);
        test_looping(8, 1);
        test_looping(8, 3);
        test_looping(9, 7);
    }

    #[test]
    fn test_count_loops() {
        assert_eq!(count_loops(load_input(test_input(DAY, ""))), 6);
    }
}
