use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub const DAY: &str = "09";

pub fn load_input(path: impl AsRef<Path>) -> Vec<Option<u32>> {
    let mut content = String::new();
    BufReader::new(File::open(path).unwrap())
        .read_to_string(&mut content)
        .unwrap();

    content
        .chars()
        .map_while(|c| c.to_digit(10))
        .scan((true, 0), |(is_file, id), size| {
            let result;
            if *is_file {
                *is_file = false;
                result = Some(*id);
                *id += 1;
            } else {
                *is_file = true;
                result = None;
            }
            Some((result, size))
        })
        .flat_map(|(result, size)| (0..size).map(move |_| result))
        .collect()
}

pub fn checksum(disk_map: &[Option<u32>]) -> usize {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|id| id as usize * i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        let input = load_input(test_input(DAY, "2"));
        assert_eq!(
            input,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(&[
                //"0099811188827773336446555566.............."
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]),
            1928
        );
    }
}
