use common::input;
use day07_lib::{load_input, total_sum, Eval, DAY};
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Op {
    Add,
    Multiply,
    Concat,
}

fn next_round(num: u64) -> Option<u64> {
    10u64.checked_pow(1 + num.checked_ilog10().unwrap_or(0))
}

impl Eval for Op {
    fn eval(&self, left: u64, right: u32) -> Option<u64> {
        let right = right as u64;
        match self {
            Op::Add => left.checked_add(right),
            Op::Multiply => left.checked_mul(right),
            Op::Concat => next_round(right)
                .and_then(|n| left.checked_mul(n))
                .and_then(|n| n.checked_add(right)),
        }
    }
}

fn main() {
    println!("{}", total_sum::<Op>(load_input(input(DAY, "")).as_slice()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_total_sum() {
        assert_eq!(
            total_sum::<Op>(load_input(test_input(DAY, "")).as_slice()),
            11387
        );
    }

    #[test]
    fn test_concat() {
        assert_eq!(Op::Concat.eval(12, 345), Some(12345));
        assert_eq!(Op::Concat.eval(10, 0), Some(100));
        assert_eq!(Op::Concat.eval(10, 10), Some(1010));
    }
}
