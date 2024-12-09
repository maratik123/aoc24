use common::input;
use day08_lib::{load_input, Input, DAY};
use std::collections::HashSet;
use std::hash::RandomState;
use tinyvec::ArrayVec;

fn signed_overflowing_sub(n1: usize, n2: usize) -> isize {
    let (res, overflowed) = n1.overflowing_sub(n2);
    let res = res as isize;
    assert_eq!(overflowed, res < 0);
    res
}

fn antinodes(
    [(i1, j1), (i2, j2)]: [(usize, usize); 2],
    (height, width): (usize, usize),
) -> ArrayVec<[(usize, usize); 2]> {
    let mut result = ArrayVec::new();
    if (i1, j1) != (i2, j2) {
        // n1 - n2 = d, a1 - n1 = d, n2 - a2 = d
        let (diff_i, diff_j) = (
            signed_overflowing_sub(i1, i2),
            signed_overflowing_sub(j1, j2),
        );
        result.extend(
            i1.checked_add_signed(diff_i)
                .filter(|&i| i < height)
                .and_then(|i| {
                    j1.checked_add_signed(diff_j)
                        .filter(|&j| j < width)
                        .map(|j| (i, j))
                }),
        );
        result.extend(
            i2.checked_add_signed(-diff_i)
                .filter(|&i| i < height)
                .and_then(|i| {
                    j2.checked_add_signed(-diff_j)
                        .filter(|&j| j < width)
                        .map(|j| (i, j))
                }),
        );
    }
    result
}

fn antinodes_count(Input { antennas, size }: Input) -> usize {
    HashSet::<_, RandomState>::from_iter(antennas.values().flat_map(|antennas| {
        antennas.iter().flat_map(|&antenna1| {
            antennas
                .iter()
                .flat_map(move |&antenna2| antinodes([antenna1, antenna2], size))
        })
    }))
    .len()
}

fn main() {
    println!("{}", antinodes_count(load_input(input(DAY, ""))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_signed_overflowing_sub() {
        assert_eq!(signed_overflowing_sub(1, 2), -1);
        assert_eq!(signed_overflowing_sub(2, 1), 1);
    }

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
        assert_eq!(antinodes_count(load_input(test_input(DAY, "1"))), 2);
        assert_eq!(antinodes_count(load_input(test_input(DAY, "2"))), 4);
        assert_eq!(antinodes_count(load_input(test_input(DAY, "3"))), 4);
        assert_eq!(antinodes_count(load_input(test_input(DAY, "4"))), 14);
    }
}
