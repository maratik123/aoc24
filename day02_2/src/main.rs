use day02_lib::{input, is_safe_report, load_input};

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
            for (i, _) in row.iter().enumerate() {
                buffer.extend(row[..i].iter().chain(row[i + 1..].iter()));
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
    let rows = load_input(input());
    println!("{}", count_safe_reports(&rows));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day02_lib::test_input;

    #[test]
    fn test_count_safe_reports() {
        let rows = load_input(test_input());
        assert_eq!(count_safe_reports(&rows), 4);
    }
}
