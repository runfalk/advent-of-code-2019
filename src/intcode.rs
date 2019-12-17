use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
pub enum Mode {
    Pos,
    Immediate,
    Relative,
}

#[derive(Debug)]
pub enum State {
    Input(PausedInterpreterInput),
    Output(PausedInterpreterOutput),
    Halt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Opcode {
    inner: usize,
}

#[derive(Clone, Debug)]
pub struct Interpreter {
    memory: HashMap<usize, isize>,
    pc: usize,
    rel_base: isize,
}

#[derive(Clone, Debug)]
pub struct PausedInterpreterInput {
    inner: Interpreter,
    pos: usize,
}

#[derive(Clone, Debug)]
pub struct PausedInterpreterOutput {
    inner: Interpreter,
    value: isize,
}

impl Opcode {
    pub fn new(inner: usize) -> Self {
        Self { inner }
    }

    pub fn code(&self) -> usize {
        self.inner % 100
    }

    pub fn param_mode(&self, i: u32) -> Result<Mode> {
        match (self.inner / 10usize.pow(i + 2)) % 10 {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            mode => Err(anyhow!(
                "Invalid parameter mode {} for parameter {}",
                mode,
                i
            )),
        }
    }
}

impl Interpreter {
    pub fn new(memory: HashMap<usize, isize>) -> Self {
        Self {
            memory,
            pc: 0,
            rel_base: 0,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let str_input = read_to_string(path)?;
        let program = str_input
            .trim_end()
            .split(",")
            .enumerate()
            .map(|(i, x)| -> Result<(usize, isize)> { Ok((i, x.parse()?)) })
            .collect::<Result<HashMap<usize, isize>, _>>()?;

        if program.len() == 0 {
            return Err(anyhow!(
                "Program is too short (expected at least 1 element, got 0)"
            ));
        }

        Ok(Self::new(program))
    }

    pub fn get(&self, i: usize) -> isize {
        self.memory.get(&i).unwrap_or(&0).clone()
    }

    pub fn put(&mut self, pos: usize, value: isize) {
        *self.memory.entry(pos).or_insert(0) = value;
    }

    pub fn read_opcode(&mut self) -> Result<Opcode> {
        Ok(Opcode::new(
            self.read_input_param(Mode::Immediate)?.try_into()?,
        ))
    }

    pub fn read_input_param(&mut self, mode: Mode) -> Result<isize> {
        let value = match mode {
            Mode::Pos => self.get(self.get(self.pc).try_into()?),
            Mode::Immediate => self.get(self.pc),
            Mode::Relative => self.get((self.rel_base + self.get(self.pc)).try_into()?),
        };
        self.pc += 1;
        Ok(value)
    }

    pub fn read_output_param(&mut self, mode: Mode) -> Result<usize> {
        let value = match mode {
            Mode::Pos => self.get(self.pc),
            Mode::Immediate => {
                return Err(anyhow!("Output parameter must not be in immediate mode"))
            }
            Mode::Relative => self.get((self.rel_base + self.get(self.pc)).try_into()?),
        };
        self.pc += 1;
        Ok(value.try_into()?)
    }

    fn read_binop_params(&mut self, op: Opcode) -> Result<(isize, isize, usize)> {
        Ok((
            self.read_input_param(op.param_mode(0)?)?,
            self.read_input_param(op.param_mode(1)?)?,
            self.read_output_param(op.param_mode(2)?)?,
        ))
    }

    pub fn add(&mut self, op: Opcode) -> Result<()> {
        let (a, b, output) = self.read_binop_params(op)?;
        self.put(output, a + b);
        Ok(())
    }

    pub fn mul(&mut self, op: Opcode) -> Result<()> {
        let (a, b, output) = self.read_binop_params(op)?;
        self.put(output, a * b);
        Ok(())
    }

    pub fn jmp(&mut self, op: Opcode) -> Result<()> {
        let cmp = self.read_input_param(op.param_mode(0)?)?;
        let jmp_target = self.read_input_param(op.param_mode(1)?)?;
        if (cmp != 0) == (op.code() == 5) {
            self.pc = jmp_target.try_into()?;
        }

        Ok(())
    }

    pub fn lt(&mut self, op: Opcode) -> Result<()> {
        let (a, b, output) = self.read_binop_params(op)?;
        if a < b {
            self.put(output, 1);
        } else {
            self.put(output, 0);
        }
        Ok(())
    }

    pub fn eq(&mut self, op: Opcode) -> Result<()> {
        let (a, b, output) = self.read_binop_params(op)?;
        if a == b {
            self.put(output, 1);
        } else {
            self.put(output, 0);
        }
        Ok(())
    }

    pub fn set_rel_base(&mut self, op: Opcode) -> Result<()> {
        self.rel_base += self.read_input_param(op.param_mode(0)?)?;
        Ok(())
    }

    pub fn run(mut self) -> Result<State> {
        loop {
            let op = self.read_opcode()?;
            match op.code() {
                1 => self.add(op)?,
                2 => self.mul(op)?,
                3 => {
                    let pos = self.read_output_param(op.param_mode(0)?)?;
                    return Ok(State::Input(PausedInterpreterInput { inner: self, pos }));
                }
                4 => {
                    let value = self.read_input_param(op.param_mode(0)?)?;
                    return Ok(State::Output(PausedInterpreterOutput {
                        inner: self,
                        value,
                    }));
                }
                5 | 6 => self.jmp(op)?,
                7 => self.lt(op)?,
                8 => self.eq(op)?,
                9 => self.set_rel_base(op)?,
                99 => return Ok(State::Halt),
                op => return Err(anyhow!("Got invalid opcode {}", op)),
            }
        }
    }
}

impl PausedInterpreterInput {
    pub fn resume(mut self, value: isize) -> Result<State> {
        self.inner.put(self.pos, value);
        self.inner.run()
    }
}

impl PausedInterpreterOutput {
    pub fn get(&self) -> isize {
        self.value
    }

    pub fn resume(self) -> Result<State> {
        self.inner.run()
    }
}
