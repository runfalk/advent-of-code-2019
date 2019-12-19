use anyhow::{anyhow, Result};
use aoc_2019::{day1, day2, day3, day4, day5, day6, day8, day9};

fn pad_newlines(answer: String) -> String {
    answer.lines().collect::<Vec<_>>().join("\n   ")
}

fn as_result<A: ToString, B: ToString>(value: (A, Option<B>)) -> (String, Option<String>) {
    (
        value.0.to_string(),
        value.1.map(|answer| answer.to_string()),
    )
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments"));
    }

    let result = match args[1].parse()? {
        1 => as_result(day1::main(&args[2..])?),
        2 => as_result(day2::main(&args[2..])?),
        3 => as_result(day3::main(&args[2..])?),
        4 => as_result(day4::main(&args[2..])?),
        5 => as_result(day5::main(&args[2..])?),
        6 => as_result(day6::main(&args[2..])?),
        8 => as_result(day8::main(&args[2..])?),
        9 => as_result(day9::main(&args[2..])?),
        1..=25 => return Err(anyhow!("No implementation for this day yet")),
        day => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
    };

    println!("A: {}", pad_newlines(result.0));
    if let Some(b) = result.1 {
        println!("B: {}", pad_newlines(b));
    }

    Ok(())
}
