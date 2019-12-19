use anyhow::{anyhow, Result};

use crate::intcode::{Interpreter, State};

fn adjust_and_compute(mut computer: Interpreter, noun: isize, verb: isize) -> Result<isize> {
    computer.put(1, noun);
    computer.put(2, verb);
    if let State::Halt(mem) = computer.run()? {
        Ok(mem[&0])
    } else {
        Err(anyhow!(
            "Program tried to do IO, but it's not supported today"
        ))
    }
}

fn find_noun_verb(computer: Interpreter, mem_start: isize) -> Result<(isize, isize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if adjust_and_compute(computer.clone(), noun, verb)? == mem_start {
                return Ok((noun, verb));
            }
        }
    }

    Err(anyhow!(
        "Unable to find a noun and verb that matches the given predicate"
    ))
}

pub fn main(args: &[String]) -> Result<(isize, Option<isize>)> {
    let computer = Interpreter::from_path(&args[0])?;
    let (noun, verb) = find_noun_verb(computer.clone(), 19690720)?;
    Ok((
        adjust_and_compute(computer, 12, 2)?,
        Some(100 * noun + verb),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_and_compute() -> Result<()> {
        assert_eq!(
            adjust_and_compute(Interpreter::from_iter(vec![1, 0, 0, 0, 99]), 1, 2)?,
            3,
        );
        Ok(())
    }
}
