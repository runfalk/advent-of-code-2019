use anyhow::{anyhow, Result};

fn is_sorted<T, I>(it: T) -> bool
where
    T: Clone + Iterator<Item = I>,
    I: Ord,
{
    let next_it = it.clone().skip(1);
    for (p, c) in it.zip(next_it) {
        if p > c {
            return false;
        }
    }
    return true;
}

fn has_repetions<T, I>(it: T) -> bool
where
    T: Clone + Iterator<Item = I>,
    I: PartialEq,
{
    let next_it = it.clone().skip(1);
    for (p, c) in it.zip(next_it) {
        if p == c {
            return true;
        }
    }
    return false;
}

fn has_pairs<T, I>(it: T) -> bool
where
    T: Clone + Iterator<Item = I>,
    I: PartialEq,
{
    let items: Vec<_> = it.collect();

    if items.len() < 2 {
        return false;
    }

    for i in 0..items.len() - 1 {
        if items[i] != items[i + 1] {
            continue;
        }

        if i > 0 && items[i - 1] == items[i] {
            continue;
        }

        if i + 2 < items.len() && items[i] == items[i + 2] {
            continue;
        }

        return true;
    }
    return false;
}

fn is_valid_a(pw: &str) -> bool {
    if pw.len() != 6 {
        return false;
    }

    if !is_sorted(pw.chars()) {
        return false;
    }

    has_repetions(pw.chars())
}

fn solve(range: impl Iterator<Item = usize>) -> (usize, Option<usize>) {
    let mut num_a = 0;
    let mut num_b = 0;

    for pw in range.map(|x| x.to_string()) {
        if !is_valid_a(&pw) {
            continue;
        }
        num_a += 1;

        if has_pairs(pw.chars()) {
            num_b += 1;
        }
    }

    (num_a, Some(num_b))
}

pub fn main(args: &[String]) -> Result<(usize, Option<usize>)> {
    if args.len() != 2 {
        return Err(anyhow!("Expected start and end"));
    }

    let interval = args[0].parse::<usize>()?..=args[1].parse::<usize>()?;

    Ok(solve(interval))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted("abc".chars()));
        assert!(is_sorted("123".chars()));
        assert!(is_sorted("1122333".chars()));
        assert!(!is_sorted("1132".chars()));
        assert!(!is_sorted("YX".chars()));
    }

    #[test]
    fn test_has_repetions() {
        assert!(has_repetions("1123".chars()));
        assert!(has_repetions("12223".chars()));
        assert!(has_repetions("1233".chars()));
        assert!(!has_repetions("123".chars()));
    }

    #[test]
    fn test_has_pairs() {
        assert!(has_pairs("1123".chars()));
        assert!(has_pairs("1233".chars()));
        assert!(has_pairs("111223".chars()));
        assert!(!has_pairs("11123".chars()));
        assert!(!has_pairs("123".chars()));
    }
}
