use anyhow::{anyhow, Result};
use aoc_2019::{day1, day2, day3, day4, day5};

fn as_result<A: ToString, B: ToString>(value: (A, Option<B>)) -> (String, Option<String>) {
    (value.0.to_string(), value.1.map(|x| x.to_string()))
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
        1..=25 => return Err(anyhow!("No implementation for this day yet")),
        day => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
    };

    println!("A: {}", result.0);
    if let Some(b) = result.1 {
        println!("B: {}", b);
    }

    Ok(())
}
