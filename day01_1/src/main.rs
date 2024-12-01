use day01_lib::{input, load_input};
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
    let (a, b) = load_input(input());
    println!("{}", similarity_score(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day01_lib::test_input;

    #[test]
    fn test_similarity_score() {
        let (a, b) = load_input(test_input());
        assert_eq!(similarity_score(a, b), 31);
    }
}
