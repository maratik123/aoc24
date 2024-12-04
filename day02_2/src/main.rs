use common::input;
use day02_lib::{is_safe_report, load_input, DAY};

fn count_safe_reports(rows: &[Vec<u32>]) -> usize {
    let mut buffer: Vec<u32> = vec![];
    rows.iter()
        .filter(|&row| {
            if is_safe_report(row) {
                return true;
            }
            if row.is_empty() {
                return false;
            }
            buffer.reserve(row.len() - 1);
            for i in 0..row.len() {
                buffer.extend(&row[..i]);
                buffer.extend(&row[i + 1..]);
                if is_safe_report(&buffer) {
                    return true;
                }
                buffer.clear();
            }
            false
        })
        .count()
}

fn main() {
    let rows = load_input(input(DAY, ""));
    println!("{}", count_safe_reports(&rows));
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::test_input;

    #[test]
    fn test_count_safe_reports() {
        let rows = load_input(test_input(DAY, ""));
        assert_eq!(count_safe_reports(&rows), 4);
    }
}
