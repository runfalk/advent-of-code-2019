use anyhow::{anyhow, Result};

use crate::intcode::{Interpreter, State};

pub fn compute(computer: Interpreter, value: isize) -> Result<isize> {
    let mut value = Some(value);
    let mut output = None;
    let mut state = computer.run()?;
    loop {
        match state {
            State::Input(c) => {
                state = c.resume(
                    value
                        .take()
                        .ok_or(anyhow!("Input value already consumed"))?,
                )?;
            }
            State::Output(c) => {
                output = Some(c.get());
                state = c.resume()?;
            }
            State::Halt(_) => break,
        }
    }
    Ok(output.ok_or(anyhow!("No output produced by computer"))?)
}

pub fn main(args: &[String]) -> Result<(isize, Option<isize>)> {
    let computer = Interpreter::from_path(&args[0])?;
    let a = compute(computer.clone(), 1)?;
    let b = compute(computer, 5)?;
    Ok((a, Some(b)))
}
