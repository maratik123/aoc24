use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map_while(|s| s.parse().ok())
        .collect()
}

pub fn load_input(path: impl AsRef<Path>) -> Vec<Vec<u32>> {
    let file = BufReader::new(File::open(path).unwrap());
    file.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn input() -> PathBuf {
    let mut path = parent_of_manifest();
    path.push("data");
    path.push("day02");
    path.push("input.txt");
    path
}

pub fn test_input() -> PathBuf {
    let mut path = parent_of_manifest();
    path.push("test_data");
    path.push("day02");
    path.push("input.txt");
    path
}

fn parent_of_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction<'a> {
    Increasing(&'a [u32]),
    Decreasing(&'a [u32]),
}

fn eval_direction(row: &[u32]) -> Option<Direction> {
    match row {
        [a, b, ..] if a > b && (1..=3).contains(&(a - b)) => Some(Direction::Decreasing(&row[1..])),
        [a, b, ..] if a < b && (1..=3).contains(&(b - a)) => Some(Direction::Increasing(&row[1..])),
        _ => None,
    }
}

fn check_direction(dir: Direction) -> bool {
    match dir {
        Direction::Increasing([head, tail @ ..]) => tail
            .iter()
            .scan(*head, |prev, it| {
                let result = (*prev, *it);
                *prev = *it;
                Some(result)
            })
            .all(|(prev, it)| prev < it && (1..=3).contains(&(it - prev))),
        Direction::Decreasing([head, tail @ ..]) => tail
            .iter()
            .scan(*head, |prev, it| {
                let result = (*prev, *it);
                *prev = *it;
                Some(result)
            })
            .all(|(prev, it)| prev > it && (1..=3).contains(&(prev - it))),
        _ => true,
    }
}

pub fn is_safe_report(row: &[u32]) -> bool {
    if let Some(dir) = eval_direction(row) {
        check_direction(dir)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(""), vec![]);
        assert_eq!(parse_line("7 6 4 2 1"), vec![7, 6, 4, 2, 1]);
        assert_eq!(
            parse_line("11 12 15 18 19 18"),
            vec![11, 12, 15, 18, 19, 18]
        );
    }

    #[test]
    fn test_load_input() {
        assert_eq!(
            load_input(test_input()),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }

    #[test]
    fn test_eval_direction() {
        assert_eq!(
            eval_direction(&[7, 6, 4, 2, 1]),
            Some(Direction::Decreasing(&[6, 4, 2, 1]))
        );
        assert_eq!(eval_direction(&[]), None);
        assert_eq!(eval_direction(&[0]), None);
        assert_eq!(eval_direction(&[0, 7]), None);
        assert_eq!(eval_direction(&[0, 0]), None);
        assert_eq!(eval_direction(&[0, 3]), Some(Direction::Increasing(&[3])));
    }

    #[test]
    fn test_check_direction() {
        assert!(check_direction(Direction::Increasing(&[])));
        assert!(check_direction(Direction::Decreasing(&[])));
        assert!(check_direction(Direction::Increasing(&[0])));
        assert!(check_direction(Direction::Decreasing(&[1])));
        assert!(check_direction(Direction::Decreasing(&[1, 0])));
        assert!(!check_direction(Direction::Increasing(&[1, 0])));
        assert!(!check_direction(Direction::Decreasing(&[0, 1])));
        assert!(check_direction(Direction::Increasing(&[0, 1])));
        assert!(!check_direction(Direction::Decreasing(&[0, 4])));
        assert!(!check_direction(Direction::Increasing(&[0, 4])));
        assert!(!check_direction(Direction::Decreasing(&[0, 0])));
        assert!(!check_direction(Direction::Increasing(&[0, 0])));
        assert!(!check_direction(Direction::Decreasing(&[0, 1, 0])));
        assert!(!check_direction(Direction::Increasing(&[0, 1, 0])));
    }
}
