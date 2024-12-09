use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "08";

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub antennas: HashMap<char, HashSet<(usize, usize)>>,
    pub size: (usize, usize),
}

pub fn load_input(path: impl AsRef<Path>) -> Input {
    let mut antennas: HashMap<_, HashSet<_>> = HashMap::new();
    let width = OnceCell::new();
    let mut i = 0;

    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        let mut j = 0;
        for ch in line.chars() {
            if matches!(ch, 'A'..='Z' | 'a'..='z' | '0'..='9') {
                antennas.entry(ch).or_default().insert((i, j));
            }
            j += 1;
        }
        assert_eq!(width.get_or_init(|| j), &j);
        i += 1;
    }

    Input {
        antennas,
        size: (i, width.get().copied().unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        assert_eq!(
            load_input(test_input(DAY, "4")),
            Input {
                antennas: HashMap::from([
                    ('0', HashSet::from([(1, 8), (2, 5), (3, 7), (4, 4)])),
                    ('A', HashSet::from([(5, 6), (8, 8), (9, 9)]))
                ]),
                size: (12, 12),
            }
        );
    }
}
