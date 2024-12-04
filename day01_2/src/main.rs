use common::input;
use day01_lib::{load_input, DAY};
use std::collections::HashMap;

fn similarity_score(a: impl IntoIterator<Item = u32>, b: impl IntoIterator<Item = u32>) -> u32 {
    let b_occurrences = b.into_iter().fold(HashMap::new(), |mut acc, b| {
        acc.entry(b).and_modify(|count| *count += 1).or_insert(1);
        acc
    });
    a.into_iter()
        .filter_map(|a| b_occurrences.get(&a).map(|b| a * b))
        .sum()
}

fn main() {
    let (a, b) = load_input(input(DAY, ""));
    println!("{}", similarity_score(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_similarity_score() {
        let (a, b) = load_input(test_input(DAY, ""));
        assert_eq!(similarity_score(a, b), 31);
    }
}
