use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn parse_line(line: &str) -> Option<(u32, u32)> {
    let mut it = line.split_whitespace();
    Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
}

pub fn load_input(path: impl AsRef<Path>) -> (Vec<u32>, Vec<u32>) {
    let file = BufReader::new(File::open(path).unwrap());
    file.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .flat_map(|line| parse_line(line.as_str()))
        .collect()
}

pub fn input() -> PathBuf {
    let mut path = parent_of_manifest();
    path.push("data");
    path.push("day01");
    path.push("input.txt");
    path
}

pub fn test_input() -> PathBuf {
    let mut path = parent_of_manifest();
    path.push("test_data");
    path.push("day01");
    path.push("test_input.txt");
    path
}

fn parent_of_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("3   4"), Some((3, 4)));
        assert_eq!(parse_line("35134   63205"), Some((35134, 63205)));
    }

    #[test]
    fn test_load_input() {
        assert_eq!(
            load_input(test_input()),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }
}
