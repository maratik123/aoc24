use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DAY: &str = "07";

pub fn load_input(path: impl AsRef<Path>) -> Vec<(u64, Vec<u32>)> {
    let input = BufReader::new(File::open(path).unwrap());
    input
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let (result, args) = line.split_once(':').unwrap();
            (
                result.parse().unwrap(),
                args.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        let input = load_input(test_input(DAY, ""));
        assert_eq!(
            input,
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ]
        );
    }
}
