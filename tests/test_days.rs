use aoc_2019::{day1, day2, day3};

#[test]
fn test_day1() {
    assert_eq!(
        day1::main(&["data/day1.txt".to_owned()]).unwrap(),
        (3481005, Some(5218616))
    );
}

#[test]
fn test_day2() {
    assert_eq!(
        day2::main(&["data/day2.txt".to_owned()]).unwrap(),
        (3267740, Some(7870))
    );
}

#[test]
fn test_day3() {
    assert_eq!(
        day3::main(&["data/day3.txt".to_owned()]).unwrap(),
        (1017, Some(11432))
    );
}
