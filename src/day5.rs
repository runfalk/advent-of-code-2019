use anyhow::{anyhow, Result};
use std::convert::TryInto;

use crate::get_params;
use crate::intcode::{intcode_from_path, Opcode, State};

pub fn add(state: &mut State, op: Opcode) -> Result<()> {
    let (a, b, output) = get_params!(state, &op, input, input, output)?;
    state.write(output, a + b)?;
    Ok(())
}

pub fn mul(state: &mut State, op: Opcode) -> Result<()> {
    let (a, b, output) = get_params!(state, &op, input, input, output)?;
    state.write(output, a * b)?;
    Ok(())
}

pub fn io_in(state: &mut State, op: Opcode) -> Result<()> {
    let pos = get_params!(state, &op, output)?;
    let input = state.pop_input()?;
    state.write(pos, input)?;
    Ok(())
}

pub fn io_out(state: &mut State, op: Opcode) -> Result<()> {
    let value = get_params!(state, &op, input)?;
    state.push_output(value);
    Ok(())
}

pub fn jmp(state: &mut State, op: Opcode, jmp_if_true: bool) -> Result<()> {
    let (cmp, jmp_target) = get_params!(state, &op, input, input)?;
    if (cmp != 0) == jmp_if_true {
        state.set_pc(jmp_target.try_into()?)?;
    }

    Ok(())
}

pub fn lt(state: &mut State, op: Opcode) -> Result<()> {
    let (a, b, output) = get_params!(state, &op, input, input, output)?;
    if a < b {
        state.write(output, 1)?;
    } else {
        state.write(output, 0)?;
    }
    Ok(())
}

pub fn eq(state: &mut State, op: Opcode) -> Result<()> {
    let (a, b, output) = get_params!(state, &op, input, input, output)?;
    if a == b {
        state.write(output, 1)?;
    } else {
        state.write(output, 0)?;
    }
    Ok(())
}

pub fn step(state: &mut State) -> Result<bool> {
    let op = Opcode::new(state.read_by_val()?);
    match op.code() {
        1 => add(state, op)?,
        2 => mul(state, op)?,
        3 => io_in(state, op)?,
        4 => io_out(state, op)?,
        5 => jmp(state, op, true)?,
        6 => jmp(state, op, false)?,
        7 => lt(state, op)?,
        8 => eq(state, op)?,
        99 => return Ok(false),
        op => return Err(anyhow!("Got invalid opcode {}", op)),
    }
    Ok(true)
}

pub fn compute<T: IntoIterator<Item = isize>>(program: Vec<isize>, input: T) -> Result<State> {
    let mut state = State::new(program);
    state.extend_input(input);

    loop {
        if !step(&mut state)? {
            break;
        }
    }
    Ok(state)
}

pub fn main(args: &[String]) -> Result<(isize, Option<isize>)> {
    let program = intcode_from_path(&args[0])?;
    let mut a = compute(program.clone(), vec![1])?;
    let mut b = compute(program, vec![5])?;

    Ok((
        a.drain_output()
            .last()
            .ok_or(anyhow!("No output for part A"))?,
        b.drain_output().last(),
    ))
}
