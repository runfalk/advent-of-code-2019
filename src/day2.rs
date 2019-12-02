use anyhow::{anyhow, Result};
use std::fs::read_to_string;
use std::result::Result as StdResult;

fn compute(mut program: Vec<usize>) -> Result<Vec<usize>> {
    if program.len() == 0 {
        return Err(anyhow!(
            "Program is too short (expected at least 1, got {})",
            program.len()
        ));
    }

    let mut i = 0;
    loop {
        let result = match program[i] {
            1 => program[program[i + 1]] + program[program[i + 2]],
            2 => program[program[i + 1]] * program[program[i + 2]],
            99 => break,
            op => return Err(anyhow!("Got invalid opcode {}", op)),
        };

        let target = program[i + 3];
        program[target] = result;

        // All opcodes are 4 wide except 99 which is a break condition
        i += 4;
    }
    Ok(program)
}

fn adjust_and_compute(input: &[usize], noun: usize, verb: usize) -> Result<Vec<usize>> {
    if input.len() < 3 {
        return Err(anyhow!(
            "Program is too short (expected at least 3, got {})",
            input.len()
        ));
    }

    let mut program = Vec::new();
    program.extend(input);

    program[1] = noun;
    program[2] = verb;

    compute(program)
}

fn find_noun_verb<F: Fn(&[usize]) -> bool>(
    input: &[usize],
    is_solution: F,
) -> Result<(usize, usize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let res = adjust_and_compute(input, noun, verb)?;
            if is_solution(res.as_slice()) {
                return Ok((noun, verb));
            }
        }
    }

    Err(anyhow!(
        "Unable to find a noun and verb that matches the given predicate"
    ))
}

pub fn main(args: &[String]) -> Result<(usize, Option<usize>)> {
    let str_input = read_to_string(&args[0])?;
    let program = str_input
        .trim_end()
        .split(",")
        .map(|x| x.parse())
        .collect::<StdResult<Vec<usize>, _>>()?;

    // Find the noun and verb that yields the correct result
    let b = find_noun_verb(program.as_slice(), |program| program[0] == 19690720)?;

    Ok((
        adjust_and_compute(program.as_slice(), 12, 2)?[0],
        Some(100 * b.0 + b.1),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() -> Result<()> {
        assert_eq!(compute(vec![1, 0, 0, 0, 99])?, vec![2, 0, 0, 0, 99]);
        assert_eq!(compute(vec![2, 3, 0, 3, 99])?, vec![2, 3, 0, 6, 99]);
        assert_eq!(
            compute(vec![2, 4, 4, 5, 99, 0])?,
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            compute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])?,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        Ok(())
    }

    #[test]
    fn test_adjust_and_compute() -> Result<()> {
        assert_eq!(
            adjust_and_compute(&[1, 0, 0, 0, 99], 1, 2)?,
            vec![3, 1, 2, 0, 99]
        );
        Ok(())
    }
}
