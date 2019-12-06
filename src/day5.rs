use anyhow::{anyhow, Result};
use std::convert::TryInto;
use std::fs::read_to_string;
use std::result::Result as StdResult;

struct DigitIterator {
    inner: isize,
}

struct Opcode {
    inner: isize,
}

struct State<T: Iterator<Item = isize>> {
    memory: Vec<isize>,
    pc: usize,
    input: T,
    output: Vec<isize>,
}

impl Iterator for DigitIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        // Technically this doesn't yield anything if the digits start at 0. This is however
        // exactly what we want for parameter modes
        if self.inner == 0 {
            return None;
        }

        let value = self.inner % 10;
        self.inner /= 10;
        Some(value)
    }
}

impl Opcode {
    fn new(inner: isize) -> Self {
        Self { inner }
    }

    fn code(&self) -> isize {
        self.inner % 100
    }

    fn param_modes(&self) -> impl Iterator<Item = bool> {
        let digits = DigitIterator {
            inner: self.inner / 100,
        };
        digits.map(|x| x != 0).chain(std::iter::repeat(false))
    }
}

impl<T: Iterator<Item = isize>> State<T> {
    fn new(memory: Vec<isize>, input: T) -> Self {
        Self {
            memory,
            pc: 0,
            input,
            output: Vec::new(),
        }
    }

    fn set_pc(&mut self, i: usize) -> Result<usize> {
        if i >= self.memory.len() {
            return Err(anyhow!(
                "Tried to set program counter outside of the program"
            ));
        }
        self.pc = i;
        Ok(i)
    }

    fn get(&self, i: usize) -> Result<isize> {
        Ok(self
            .memory
            .get(i)
            .ok_or(anyhow!(
                "Tried to read instruction beyond the end of the memory ({})",
                i
            ))?
            .clone())
    }

    fn peek(&self, by_val: bool) -> Result<isize> {
        if by_val {
            self.get(self.pc)
        } else {
            self.get(self.get(self.pc)?.try_into()?)
        }
    }

    fn read(&mut self, by_val: bool) -> Result<isize> {
        let value = self.peek(by_val)?;
        self.pc += 1;
        Ok(value)
    }

    fn read_by_val(&mut self) -> Result<isize> {
        self.read(true)
    }

    fn write(&mut self, pos: usize, value: isize) -> Result<isize> {
        let item = self
            .memory
            .get_mut(pos)
            .ok_or(anyhow!("Tried to write instruction outside of memory"))?;
        *item = value;
        Ok(value)
    }

    fn get_input(&mut self) -> Result<isize> {
        self.input
            .next()
            .ok_or(anyhow!("Tried to get input but it's empty"))
    }

    fn output(&mut self, value: isize) {
        self.output.push(value);
    }
}

fn get_binop_params<T: Iterator<Item = isize>>(
    state: &mut State<T>,
    op: &Opcode,
) -> Result<(isize, isize, usize)> {
    let mut param_iter = op.param_modes().map(|by_val| state.read(by_val));

    let a = param_iter.next().unwrap()?;
    let b = param_iter.next().unwrap()?;
    let output = state.read_by_val()?;

    Ok((a, b, output.try_into()?))
}

fn add<T: Iterator<Item = isize>>(state: &mut State<T>, op: Opcode) -> Result<()> {
    let (a, b, output) = get_binop_params(state, &op)?;
    state.write(output, a + b)?;
    Ok(())
}

fn mul<T: Iterator<Item = isize>>(state: &mut State<T>, op: Opcode) -> Result<()> {
    let (a, b, output) = get_binop_params(state, &op)?;
    state.write(output, a * b)?;
    Ok(())
}

fn io_in<T: Iterator<Item = isize>>(state: &mut State<T>, _op: Opcode) -> Result<()> {
    let pos = state.read_by_val()?.try_into()?;
    let input = state.get_input()?;
    state.write(pos, input)?;
    Ok(())
}

fn io_out<T: Iterator<Item = isize>>(state: &mut State<T>, op: Opcode) -> Result<()> {
    let value = state.read(op.param_modes().next().unwrap())?;
    state.output(value);
    Ok(())
}

fn jmp<T: Iterator<Item = isize>>(
    state: &mut State<T>,
    op: Opcode,
    jmp_if_true: bool,
) -> Result<()> {
    let mut param_iter = op.param_modes().map(|by_val| state.read(by_val));

    let cmp = param_iter.next().unwrap()?;
    let jmp_target = param_iter.next().unwrap()?;

    if (cmp != 0) == jmp_if_true {
        state.set_pc(jmp_target.try_into()?)?;
    }

    Ok(())
}

fn lt<T: Iterator<Item = isize>>(state: &mut State<T>, op: Opcode) -> Result<()> {
    let (a, b, output) = get_binop_params(state, &op)?;
    if a < b {
        state.write(output, 1)?;
    } else {
        state.write(output, 0)?;
    }
    Ok(())
}

fn eq<T: Iterator<Item = isize>>(state: &mut State<T>, op: Opcode) -> Result<()> {
    let (a, b, output) = get_binop_params(state, &op)?;
    if a == b {
        state.write(output, 1)?;
    } else {
        state.write(output, 0)?;
    }
    Ok(())
}

fn compute<T: Iterator<Item = isize>>(program: Vec<isize>, input: T) -> Result<State<T>> {
    if program.len() == 0 {
        return Err(anyhow!(
            "Program is too short (expected at least 1, got {})",
            program.len()
        ));
    }

    let mut state = State::new(program, input);
    loop {
        let op = Opcode::new(state.read_by_val()?);
        match op.code() {
            1 => add(&mut state, op)?,
            2 => mul(&mut state, op)?,
            3 => io_in(&mut state, op)?,
            4 => io_out(&mut state, op)?,
            5 => jmp(&mut state, op, true)?,
            6 => jmp(&mut state, op, false)?,
            7 => lt(&mut state, op)?,
            8 => eq(&mut state, op)?,
            99 => break,
            op => return Err(anyhow!("Got invalid opcode {}", op)),
        }
    }
    Ok(state)
}

pub fn main(args: &[String]) -> Result<(isize, Option<isize>)> {
    let str_input = read_to_string(&args[0])?;
    let program = str_input
        .trim_end()
        .split(",")
        .map(|x| x.parse())
        .collect::<StdResult<Vec<isize>, _>>()?;

    let a = compute(program.clone(), vec![1].into_iter())?;
    let b = compute(program, vec![5].into_iter())?;

    Ok((
        a.output.into_iter().last().unwrap(),
        b.output.into_iter().last(),
    ))
}
