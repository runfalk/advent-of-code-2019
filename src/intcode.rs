use anyhow::{anyhow, Result};
use std::collections::{vec_deque::Drain, VecDeque};
use std::convert::TryInto;
use std::fs::read_to_string;
use std::path::Path;

pub struct DigitIterator {
    inner: isize,
}

pub struct Opcode {
    inner: isize,
}

pub struct State {
    memory: Vec<isize>,
    pc: usize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}

pub fn intcode_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<isize>> {
    let str_input = read_to_string(path)?;
    let program = str_input
        .trim_end()
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<Vec<isize>, _>>()?;

    if program.len() == 0 {
        return Err(anyhow!(
            "Program is too short (expected at least 1 element, got 0)"
        ));
    }

    Ok(program)
}

/// Helper macro to extract parameters for instructions
#[macro_export]
macro_rules! get_params {
    ($state:expr, $param_modes:expr, fetch_param, input) => {
        {
            // Input parameters respect parameter modes
            let is_by_val = $param_modes.next().unwrap();
            $state.read(is_by_val)
        }
    };
    ($state:expr, $param_modes:expr, fetch_param, output) => {
        {
            // Output parameters are always returned by value since the index we use the value as
            // index when writing to memory
            let is_by_val = $param_modes.next().unwrap();

            if is_by_val {
                Err(anyhow!("Output parameter must not be in immediate mode"))
            } else {
                use std::convert::TryFrom;
                let out: Result<usize> = match $state.read(true) {
                    Ok(pos) => Ok(usize::try_from(pos)?).into(),
                    e => e.map(|_| 0).into(),
                };
                out
            }
        }
    };
    ($state:expr, $op:expr, $($mode:ident),+) => {
        {
            let get_params = |state: &mut State, op: &Opcode| -> Result<_> {
                let mut param_mode_iter = op.param_modes();

                // Unused parens happens when there is only one parameter
                #[allow(unused_parens)]
                Ok((
                    $(get_params!(state, param_mode_iter, fetch_param, $mode)?),+
                ))
            };

            get_params($state, $op)
        }
    };
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
    pub fn new(inner: isize) -> Self {
        Self { inner }
    }

    pub fn code(&self) -> isize {
        self.inner % 100
    }

    pub fn param_modes(&self) -> impl Iterator<Item = bool> {
        let digits = DigitIterator {
            inner: self.inner / 100,
        };
        digits.map(|x| x != 0).chain(std::iter::repeat(false))
    }
}

impl State {
    pub fn new(memory: Vec<isize>) -> Self {
        Self {
            memory,
            pc: 0,
            input: VecDeque::default(),
            output: VecDeque::default(),
        }
    }

    pub fn set_pc(&mut self, i: usize) -> Result<usize> {
        if i >= self.memory.len() {
            return Err(anyhow!(
                "Tried to set program counter outside of the program"
            ));
        }
        self.pc = i;
        Ok(i)
    }

    pub fn get(&self, i: usize) -> Result<isize> {
        Ok(self
            .memory
            .get(i)
            .ok_or(anyhow!(
                "Tried to read instruction beyond the end of the memory ({})",
                i
            ))?
            .clone())
    }

    pub fn peek(&self, by_val: bool) -> Result<isize> {
        if by_val {
            self.get(self.pc)
        } else {
            self.get(self.get(self.pc)?.try_into()?)
        }
    }

    pub fn read(&mut self, by_val: bool) -> Result<isize> {
        let value = self.peek(by_val)?;
        self.pc += 1;
        Ok(value)
    }

    pub fn read_by_val(&mut self) -> Result<isize> {
        self.read(true)
    }

    pub fn write(&mut self, pos: usize, value: isize) -> Result<isize> {
        let item = self
            .memory
            .get_mut(pos)
            .ok_or(anyhow!("Tried to write instruction outside of memory"))?;
        *item = value;
        Ok(value)
    }

    pub fn pop_input(&mut self) -> Result<isize> {
        self.input
            .pop_front()
            .ok_or(anyhow!("Tried to get input but it's empty"))
    }

    pub fn push_input(&mut self, value: isize) {
        self.input.push_back(value);
    }

    pub fn extend_input<T: IntoIterator<Item = isize>>(&mut self, it: T) {
        self.input.extend(it);
    }

    pub fn push_output(&mut self, value: isize) {
        self.output.push_back(value);
    }

    pub fn drain_output(&mut self) -> Drain<isize> {
        self.output.drain(..)
    }
}
