use std::io::Read;

use anyhow::Result;

use crate::ops::*;

#[derive(Debug)]
pub struct VMBuilder<T> {
    m_program: T,
    m_pc: usize,
    m_data: [isize; 30000],
    m_dp: usize,
}

#[derive(Debug)]
pub struct VM {
    m_program: Vec<Op>,
    m_pc: usize,
    m_data: [isize; 30000],
    m_dp: usize,
}

#[derive(Debug, Clone)]
pub struct NoOps;
pub type Ops = Vec<Op>;

impl VMBuilder<NoOps> {
    fn new() -> Self {
        VMBuilder {
            m_program: NoOps,
            m_pc: 0,
            m_data: [0; 30000],
            m_dp: 0,
        }
    }

    pub fn with_program(self, program: impl Into<Ops>) -> VMBuilder<Ops> {
        return VMBuilder {
            m_program: program.into(),
            m_pc: self.m_pc,
            m_data: self.m_data,
            m_dp: self.m_dp,
        };
    }
}

impl VMBuilder<Ops> {
    pub fn build(self) -> VM {
        return VM {
            m_program: self.m_program,
            m_pc: self.m_pc,
            m_data: self.m_data,
            m_dp: self.m_dp,
        };
    }
}

impl VM {
    pub fn new() -> VMBuilder<NoOps> {
        return VMBuilder::new();
    }

    pub fn run(mut self) -> Result<()> {
        let data_len = self.m_data.len();
        let program_len = self.m_program.len();

        while self.m_pc < program_len {
            match self.m_program[self.m_pc] {
                Op::Plus(n) => {
                    self.m_data[self.m_dp] = self.m_data[self.m_dp] + n as isize;
                }
                Op::Minus(n) => {
                    self.m_data[self.m_dp] = self.m_data[self.m_dp] - n as isize;
                }
                Op::Left(n) => {
                    let result = self.m_dp - n;
                    if result >= data_len {
                        return Err(anyhow::anyhow!("DP underflow"));
                    }

                    self.m_dp = result;
                }
                Op::Right(n) => {
                    let result = self.m_dp + n;
                    if result >= data_len {
                        return Err(anyhow::anyhow!("DP overflow"));
                    }

                    self.m_dp = result;
                }
                Op::RightBracket(n) => {
                    if self.m_data[self.m_dp] != 0 {
                        self.m_pc = n;
                        continue;
                    }
                }
                Op::LeftBracket(n) => {
                    if self.m_data[self.m_dp] == 0 {
                        self.m_pc = n;
                        continue;
                    }
                }
                Op::Dot(n) => {
                    (0..n).for_each(|_| print!("{}", self.m_data[self.m_dp] as u8 as char));
                }
                Op::Comma(n) => {
                    let mut buf = [0; 1];
                    (0..n).for_each(|_| {
                        if let Err(_) = std::io::stdin().read_exact(&mut buf) {
                            return;
                        }

                        self.m_data[self.m_dp] = buf[0] as isize;
                    });
                }
                Op::NoOp => {}
            }
            self.m_pc += 1;
        }

        Ok(())
    }
}

impl<T> VMBuilder<T> {
    pub fn with_pc(mut self, idx: usize) -> Self {
        self.m_pc = idx;
        return self;
    }

    pub fn with_data(mut self, data: [isize; 30000]) -> Self {
        self.m_data = data;
        return self;
    }

    pub fn with_dp(mut self, idx: usize) -> Self {
        self.m_dp = idx;
        return self;
    }
}
