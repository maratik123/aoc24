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
}
