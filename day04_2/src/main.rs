use common::input;
use day04_lib::{load_input, DAY};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
enum Direction {
    UpLeftToDownRight,
    UpRightToDownLeft,
}

impl Direction {
    fn pos(&self, (i, j): (usize, usize), pos: usize, backward: bool) -> (usize, usize) {
        match (self, backward) {
            (Direction::UpLeftToDownRight, false) => (i + pos - 1, j + pos - 1),
            (Direction::UpLeftToDownRight, true) => (i + 1 - pos, j + 1 - pos),
            (Direction::UpRightToDownLeft, false) => (i + pos - 1, j + 1 - pos),
            (Direction::UpRightToDownLeft, true) => (i + 1 - pos, j + pos - 1),
        }
    }

    fn check_words(&self, hay: &[Vec<char>], (i, j): (usize, usize)) -> bool {
        ['M', 'S'].iter().enumerate().all(|(pos, letter)| {
            let pos = pos * 2;
            [false, true].iter().any(|&backward| {
                let (i, j) = self.pos((i, j), pos, backward);
                hay[i][j] == *letter
            })
        })
    }
}

fn xmas_count((input, line_size): (&[Vec<char>], usize)) -> usize {
    input
        .iter()
        .inspect(|line| debug_assert_eq!(line.len(), line_size))
        .enumerate()
        .skip(1)
        .take(input.len() - 2)
        .flat_map(|(i, line)| {
            (1..line_size - 1).filter(move |&j| {
                line[j] == 'A' && Direction::iter().all(move |dir| dir.check_words(input, (i, j)))
            })
        })
        .count()
}

fn main() {
    let (input, line_size) = load_input(input(DAY, ""));
    println!("{}", xmas_count((input.as_slice(), line_size)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_check_word() {
        let (input, line_size) = load_input(test_input(DAY, ""));
        assert_eq!(input.len(), 10);
        assert_eq!(line_size, 10);
        let (i, j) = (1, 2);
        assert_eq!(input[i][j], 'A');
        assert!(Direction::UpRightToDownLeft.check_words(input.as_slice(), (i, j)));
    }

    #[test]
    fn test_xmas_count() {
        let (input, line_size) = load_input(test_input(DAY, ""));
        assert_eq!(xmas_count((input.as_slice(), line_size)), 9);
    }
}
