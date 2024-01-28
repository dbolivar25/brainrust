#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Plus(usize),
    Minus(usize),
    Left(usize),
    Right(usize),
    RightBracket(usize),
    LeftBracket(usize),
    Dot(usize),
    Comma(usize),
    NoOp,
}

impl Op {
    pub fn set_jump(&mut self, jump: usize) {
        match self {
            Op::RightBracket(n) => *n = jump,
            Op::LeftBracket(n) => *n = jump,
            _ => panic!("Cannot set jump on non-bracket op"),
        }
    }
}
