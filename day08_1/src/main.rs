use common::input;
use day08_lib::{antinodes_count, load_input, signed_overflowing_sub, DAY};
use tinyvec::ArrayVec;

fn antinodes(
    [(i1, j1), (i2, j2)]: [(usize, usize); 2],
    (height, width): (usize, usize),
) -> ArrayVec<[(usize, usize); 2]> {
    assert_ne!((i1, j1), (i2, j2));
    let mut result = ArrayVec::new();
    // n1 - n2 = d, a1 - n1 = d, n2 - a2 = d
    let (diff_i, diff_j) = (
        signed_overflowing_sub(i1, i2),
        signed_overflowing_sub(j1, j2),
    );
    let mut ext = |diff_i: isize, diff_j: isize, i: usize, j: usize| {
        result.extend(
            i.checked_add_signed(diff_i)
                .filter(|&i| i < height)
                .and_then(|i| {
                    j.checked_add_signed(diff_j)
                        .filter(|&j| j < width)
                        .map(|j| (i, j))
                }),
        );
    };
    ext(diff_i, diff_j, i1, j1);
    ext(-diff_i, -diff_j, i2, j2);
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
            HashSet::from_iter(antinodes([(3, 4), (5, 5)], (10, 10))),
            HashSet::from([(1, 3), (7, 6)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(5, 5), (3, 4)], (10, 10))),
            HashSet::from([(1, 3), (7, 6)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(3, 4), (4, 8)], (10, 10))),
            HashSet::from([(2, 0)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(3, 4), (4, 0)], (10, 10))),
            HashSet::from([(2, 8)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(4, 3), (8, 4)], (10, 10))),
            HashSet::from([(0, 2)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(5, 5), (4, 8)], (10, 10))),
            HashSet::from([(6, 2)])
        );
        assert_eq!(
            HashSet::from_iter(antinodes([(5, 5), (8, 4)], (10, 10))),
            HashSet::from([(2, 6)])
        );
    }

    #[test]
    fn test_antinodes_count() {
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "1")), antinodes),
            2
        );
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "2")), antinodes),
            4
        );
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "3")), antinodes),
            4
        );
        assert_eq!(
            antinodes_count(load_input(test_input(DAY, "4")), antinodes),
            14
        );
    }
}
