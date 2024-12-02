use day02_lib::{input, is_safe_report, load_input};

fn count_safe_reports(rows: &[Vec<u32>]) -> usize {
    rows.iter().filter(|row| is_safe_report(row)).count()
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
    fn test_is_safe_report() {
        assert!(is_safe_report(&[7, 6, 4, 2, 1]));
        assert!(is_safe_report(&[1, 2, 3]));
        assert!(!is_safe_report(&[1, 2, 1]));
        assert!(is_safe_report(&[1, 2]));
        assert!(!is_safe_report(&[1]));
    }

    #[test]
    fn test_count_safe_reports() {
        let rows = load_input(test_input());
        assert_eq!(count_safe_reports(&rows), 2);
    }
}
