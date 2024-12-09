use common::input;
use day08_lib::{antinodes_count, load_input, signed_overflowing_sub, DAY};

fn antinodes(
    [(i1, j1), (i2, j2)]: [(usize, usize); 2],
    (height, width): (usize, usize),
) -> Vec<(usize, usize)> {
    assert_ne!((i1, j1), (i2, j2));
    let mut result = vec![];
    // n1 - n2 = d, a1 - n1 = d, n2 - a2 = d
    let (diff_i, diff_j) = (
        signed_overflowing_sub(i1, i2),
        signed_overflowing_sub(j1, j2),
    );
    result.extend((0..).map_while(|n| {
        i1.checked_add_signed(n * diff_i)
            .filter(|&i| i < height)
            .and_then(|i| {
                j1.checked_add_signed(n * diff_j)
                    .filter(|&j| j < width)
                    .map(|j| (i, j))
            })
    }));
    result.extend((0..).map_while(|n| {
        i2.checked_add_signed(-n * diff_i)
            .filter(|&i| i < height)
            .and_then(|i| {
                j2.checked_add_signed(-n * diff_j)
                    .filter(|&j| j < width)
                    .map(|j| (i, j))
            })
    }));
    result
}

fn main() {
    println!("{}", antinodes_count(load_input(input(DAY, "")), antinodes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;
    use std::collections::HashSet;

    #[test]
    fn test_antinodes() {
        assert_eq!(
            HashSet::from_iter(antinodes([(0, 0), (1, 3)], (10, 10))),
            HashSet::from([(2, 6), (3, 9), (0, 0), (1, 3)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(2, 1), (1, 3)], (10, 10))),
            HashSet::from([(0, 5), (2, 1), (1, 3)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(0, 0), (2, 1)], (10, 10))),
            HashSet::from([(4, 2), (6, 3), (8, 4), (0, 0), (2, 1)])
        );
    }

    #[test]
    fn test_antinodes_count() {
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "5")), antinodes),
            9
        );
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "4")), super::antinodes),
            34
        );
    }
}
