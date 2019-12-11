use anyhow::Result;
use aoc_2019::{day1, day2, day3, day4, day5, day6, day8};

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

#[test]
fn test_day4() {
    assert_eq!(
        day4::main(&["236491".to_owned(), "713787".to_owned()]).unwrap(),
        (1169, Some(757))
    );
}

#[test]
fn test_day5() {
    assert_eq!(
        day5::main(&["data/day5.txt".to_owned()]).unwrap(),
        (8332629, Some(8805067))
    );
}

#[test]
fn test_day6() -> Result<()> {
    assert_eq!(
        day6::main(&["data/day6.txt".to_owned()])?,
        (171213, Some(292))
    );
    Ok(())
}

#[test]
fn test_day8() -> Result<()> {
    assert_eq!(
        day8::main(&["data/day8.txt".to_owned()])?,
        (
            2176,
            Some(
                vec![
                    " ##  #   ##  # ###  #   #",
                    "#  # #   ## #  #  # #   #",
                    "#     # # ##   ###   # # ",
                    "#      #  # #  #  #   #  ",
                    "#  #   #  # #  #  #   #  ",
                    " ##    #  #  # ###    #  ",
                ]
                .join("\n")
            )
        )
    );
    Ok(())
}
