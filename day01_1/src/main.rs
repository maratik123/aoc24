use day01_lib::{input, load_input};
use std::collections::HashMap;

fn similarity_score(a: &[u32], b: &[u32]) -> u32 {
    let b_occurrences = b.iter().fold(HashMap::new(), |mut acc, x| {
        acc.entry(x).and_modify(|x| *x += 1).or_insert(1);
        acc
    });
    a.iter()
        .map(|a| a * b_occurrences.get(&a).unwrap_or(&0))
        .sum()
}

fn main() {
    let (a, b) = load_input(input());
    println!("{}", similarity_score(a.as_slice(), b.as_slice()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day01_lib::test_input;

    #[test]
    fn test_similarity_score() {
        let (a, b) = load_input(test_input());
        assert_eq!(similarity_score(a.as_slice(), b.as_slice()), 31);
    }
}
