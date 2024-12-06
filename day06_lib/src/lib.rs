use std::cell::OnceCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "06";

#[derive(Debug, Eq, PartialEq)]
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
}
