use common::input;
use day05_lib::{is_right_order, load_input, middle, Input, DAY};

fn sum_right_order_middles(
    Input {
        pages,
        ordering_rules,
    }: Input,
) -> u32 {
    pages
        .into_iter()
        .filter(|pages| is_right_order(pages, &ordering_rules))
        .map(|pages| middle(pages.as_slice()))
        .sum()
}

fn main() {
    println!("{}", sum_right_order_middles(load_input(input(DAY, ""))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_sum_right_order_middles() {
        assert_eq!(
            sum_right_order_middles(load_input(test_input(DAY, ""))),
            143
        );
    }
}
