use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::Path;

fn parse_line(line: &str) -> Option<(u32, u32)> {
    let mut it = line.split_whitespace();
    Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
}

fn load_input(path: &Path) -> (Vec<u32>, Vec<u32>) {
    let file = BufReader::new(File::open(path).unwrap());
    file.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .flat_map(|line| parse_line(line.as_str()))
        .collect()
}

fn total_distance(mut a: Vec<u32>, mut b: Vec<u32>) -> u32 {
    a.sort_unstable();
    b.sort_unstable();
    iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn main() {
    let (a, b) = load_input(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")));
    println!("{}", total_distance(a, b));
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

    #[test]
    fn test_total_distance() {
        let (a, b) = load_input(test_input());
        assert_eq!(total_distance(a, b), 11);
    }

    fn test_input() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test_input.txt"))
    }
}
