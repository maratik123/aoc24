use day01_lib::{input, load_input};
use std::iter;

fn total_distance(mut a: Vec<u32>, mut b: Vec<u32>) -> u32 {
    a.sort_unstable();
    b.sort_unstable();
    iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn main() {
    let (a, b) = load_input(input());
    println!("{}", total_distance(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day01_lib::test_input;

    #[test]
    fn test_total_distance() {
        let (a, b) = load_input(test_input());
        assert_eq!(total_distance(a, b), 11);
    }
}
