use common::input;
use day07_lib::{load_input, DAY};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn eval(&self, left: u64, right: u32) -> Option<u64> {
        let right = right as u64;
        match self {
            Op::Add => left.checked_add(right),
            Op::Multiply => left.checked_mul(right),
        }
    }
}

fn dfs(end: u64, args: &[u32]) -> bool {
    let mut s = vec![(0, args[0] as u64)];
    while let Some((depth, val)) = s.pop() {
        let next_depth = depth + 1;
        for op in Op::iter() {
            if let Some(new_val) = op.eval(val, args[next_depth]) {
                if next_depth + 1 < args.len() {
                    s.push((next_depth, new_val));
                } else if new_val == end {
                    return true;
                }
            }
        }
    }
    false
}

fn total_sum(input: &[(u64, Vec<u32>)]) -> u64 {
    input
        .iter()
        .filter(|(end, args)| dfs(*end, args))
        .map(|(end, _)| end)
        .sum()
}

fn main() {
    println!("{}", total_sum(load_input(input(DAY, "")).as_slice()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_dfs() {
        assert!(dfs(190, &[10, 19]));
        assert!(dfs(3267, &[81, 40, 27]));
    }

    #[test]
    fn test_total_sum() {
        assert_eq!(total_sum(load_input(test_input(DAY, "")).as_slice()), 3749);
    }
}
