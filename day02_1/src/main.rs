use day02_lib::{input, load_input};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction<'a> {
    Increasing(&'a [u32]),
    Decreasing(&'a [u32]),
}

fn eval_direction(row: &[u32]) -> Option<Direction> {
    match row {
        [a, b, ..] if a > b && (1..=3).contains(&(a - b)) => Some(Direction::Decreasing(&row[1..])),
        [a, b, ..] if a < b && (1..=3).contains(&(b - a)) => Some(Direction::Increasing(&row[1..])),
        _ => None,
    }
}

fn check_direction(dir: Direction) -> bool {
    match dir {
        Direction::Increasing([head, tail @ ..]) => tail
            .iter()
            .scan(*head, |prev, it| {
                let result = (*prev, *it);
                *prev = *it;
                Some(result)
            })
            .all(|(prev, it)| prev < it && (1..=3).contains(&(it - prev))),
        Direction::Decreasing([head, tail @ ..]) => tail
            .iter()
            .scan(*head, |prev, it| {
                let result = (*prev, *it);
                *prev = *it;
                Some(result)
            })
            .all(|(prev, it)| prev > it && (1..=3).contains(&(prev - it))),
        _ => true,
    }
}

fn is_safe_report(row: &[u32]) -> bool {
    if let Some(dir) = eval_direction(row) {
        check_direction(dir)
    } else {
        false
    }
}

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
    use day02_lib::{load_input, test_input};

    #[test]
    fn test_eval_direction() {
        assert_eq!(
            eval_direction(&[7, 6, 4, 2, 1]),
            Some(Direction::Decreasing(&[6, 4, 2, 1]))
        );
        assert_eq!(eval_direction(&[]), None);
        assert_eq!(eval_direction(&[0]), None);
        assert_eq!(eval_direction(&[0, 7]), None);
        assert_eq!(eval_direction(&[0, 0]), None);
        assert_eq!(eval_direction(&[0, 3]), Some(Direction::Increasing(&[3])));
    }

    #[test]
    fn test_check_direction() {
        assert!(check_direction(Direction::Increasing(&[])));
        assert!(check_direction(Direction::Decreasing(&[])));
        assert!(check_direction(Direction::Increasing(&[0])));
        assert!(check_direction(Direction::Decreasing(&[1])));
        assert!(check_direction(Direction::Decreasing(&[1, 0])));
        assert!(!check_direction(Direction::Increasing(&[1, 0])));
        assert!(!check_direction(Direction::Decreasing(&[0, 1])));
        assert!(check_direction(Direction::Increasing(&[0, 1])));
        assert!(!check_direction(Direction::Decreasing(&[0, 4])));
        assert!(!check_direction(Direction::Increasing(&[0, 4])));
        assert!(!check_direction(Direction::Decreasing(&[0, 0])));
        assert!(!check_direction(Direction::Increasing(&[0, 0])));
        assert!(!check_direction(Direction::Decreasing(&[0, 1, 0])));
        assert!(!check_direction(Direction::Increasing(&[0, 1, 0])));
    }

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
