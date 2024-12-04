use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub const DAY: &str = "03";

pub fn load_input(path: impl AsRef<Path>) -> Vec<u8> {
    let mut result = vec![];
    BufReader::new(File::open(path).unwrap())
        .read_to_end(&mut result)
        .unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_load_input() {
        let input = load_input(test_input(DAY, "1"));
        assert_eq!(
            String::from_utf8_lossy(input.as_slice()),
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n"
        );
    }
}
