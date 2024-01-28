use nom::{
    self,
    branch::alt,
    bytes::complete::{take_till, take_until, take_while},
    character::complete::{char, multispace0},
    combinator::{eof, peek},
    error::VerboseError,
    multi::{many0, many1, many_till},
    sequence::preceded,
};

use crate::ops::*;

fn parse_plus(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    many1(preceded(multispace0, char('+')))(input).map(|(i, v)| (i, Op::Plus(v.len())))
}

fn parse_minus(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    many1(preceded(multispace0, char('-')))(input).map(|(i, v)| (i, Op::Minus(v.len())))
}

fn parse_left(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    many1(preceded(multispace0, char('<')))(input).map(|(i, v)| (i, Op::Left(v.len())))
}

fn parse_right(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    many1(preceded(multispace0, char('>')))(input).map(|(i, v)| (i, Op::Right(v.len())))
}

fn parse_right_bracket(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    preceded(multispace0, char(']'))(input).map(|(i, _)| (i, Op::RightBracket(0)))
}

fn parse_left_bracket(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    preceded(multispace0, char('['))(input).map(|(i, _)| (i, Op::LeftBracket(0)))
}

fn parse_dot(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    many1(preceded(multispace0, char('.')))(input).map(|(i, v)| (i, Op::Dot(v.len())))
}

fn parse_comma(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    many1(preceded(multispace0, char(',')))(input).map(|(i, v)| (i, Op::Comma(v.len())))
}

fn parse_comment(input: &str) -> nom::IResult<&str, Op, VerboseError<&str>> {
    take_till(|c| matches!(c, '+' | '-' | '<' | '>' | '[' | ']' | '.' | ','))(input)
        .map(|(i, _)| (i, Op::NoOp))
}

fn parse_ops(input: &str) -> Result<Vec<Op>, nom::Err<VerboseError<&str>>> {
    many_till(
        alt((
            parse_plus,
            parse_minus,
            parse_left,
            parse_right,
            parse_right_bracket,
            parse_left_bracket,
            parse_dot,
            parse_comma,
            parse_comment,
        )),
        preceded(multispace0, eof),
    )(input)
    .map(|(_, (v, _))| v)
}

pub fn parse_program(input: &str) -> Result<Vec<Op>, nom::Err<VerboseError<&str>>> {
    let mut ops = parse_ops(input)?;
    ops.retain(|op| !matches!(op, Op::NoOp));

    // use stack to match brackets and set the correct jump locations
    let mut stack = Vec::new();
    for (i, op) in ops.iter_mut().enumerate() {
        match op {
            Op::LeftBracket(_) => {
                stack.push((i, op));
            }
            Op::RightBracket(_) => {
                let (left_bracket_idx, left_bracket_op) = stack.pop().unwrap();
                left_bracket_op.set_jump(i);
                op.set_jump(left_bracket_idx);
            }
            _ => {}
        }
    }

    Ok(ops)
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn plus() {
        assert_eq!(parse_plus("+++"), Ok(("", Op::Plus(3))));

        assert_eq!(parse_plus("+++>"), Ok((">", Op::Plus(3))));

        assert_eq!(parse_plus("+++>+++"), Ok((">+++", Op::Plus(3))));

        assert_eq!(parse_plus("+++>+++<"), Ok((">+++<", Op::Plus(3))));
    }

    #[test]
    fn minus() {
        assert_eq!(parse_minus("---"), Ok(("", Op::Minus(3))));

        assert_eq!(parse_minus("---<"), Ok(("<", Op::Minus(3))));

        assert_eq!(parse_minus("---<---"), Ok(("<---", Op::Minus(3))));

        assert_eq!(parse_minus("---<--->"), Ok(("<--->", Op::Minus(3))));
    }

    #[test]
    fn left() {
        assert_eq!(parse_left("<<<"), Ok(("", Op::Left(3))));

        assert_eq!(parse_left("<<<>"), Ok((">", Op::Left(3))));

        assert_eq!(parse_left("<<<>>>"), Ok((">>>", Op::Left(3))));

        assert_eq!(parse_left("<<<>><<<"), Ok((">><<<", Op::Left(3))));
    }

    #[test]
    fn right() {
        assert_eq!(parse_right(">>>"), Ok(("", Op::Right(3))));

        assert_eq!(parse_right(">>>+"), Ok(("+", Op::Right(3))));

        assert_eq!(parse_right(">>>+++"), Ok(("+++", Op::Right(3))));

        assert_eq!(parse_right(">>>+++<<<"), Ok(("+++<<<", Op::Right(3))));
    }

    #[test]
    fn right_bracket() {
        assert_eq!(parse_right_bracket("]"), Ok(("", Op::RightBracket(0))));

        assert_eq!(parse_right_bracket("]>"), Ok((">", Op::RightBracket(0))));

        assert_eq!(parse_right_bracket("]>>"), Ok((">>", Op::RightBracket(0))));

        assert_eq!(
            parse_right_bracket("]>>>"),
            Ok((">>>", Op::RightBracket(0)))
        );
    }

    #[test]
    fn left_bracket() {
        assert_eq!(parse_left_bracket("["), Ok(("", Op::LeftBracket(0))));

        assert_eq!(parse_left_bracket("[<"), Ok(("<", Op::LeftBracket(0))));

        assert_eq!(parse_left_bracket("[<<"), Ok(("<<", Op::LeftBracket(0))));

        assert_eq!(parse_left_bracket("[<<<"), Ok(("<<<", Op::LeftBracket(0))));
    }

    #[test]
    fn dot() {
        assert_eq!(parse_dot("..."), Ok(("", Op::Dot(3))));

        assert_eq!(parse_dot("...+"), Ok(("+", Op::Dot(3))));

        assert_eq!(parse_dot("...+++"), Ok(("+++", Op::Dot(3))));

        assert_eq!(parse_dot("...+++<"), Ok(("+++<", Op::Dot(3))));
    }

    #[test]
    fn comma() {
        assert_eq!(parse_comma(",,,"), Ok(("", Op::Comma(3))));

        assert_eq!(parse_comma(",,,<"), Ok(("<", Op::Comma(3))));

        assert_eq!(parse_comma(",,,<<<"), Ok(("<<<", Op::Comma(3))));

        assert_eq!(parse_comma(",,,<<<>"), Ok(("<<<>", Op::Comma(3))));
    }

    #[test]
    fn comment() {
        assert_eq!(
            parse_comment("hello world...+++<"),
            Ok(("...+++<", Op::NoOp))
        );

        assert_eq!(
            parse_comment("           hello world...+++<"),
            Ok(("...+++<", Op::NoOp))
        )
    }

    #[test]
    fn ops() {
        assert_eq!(parse_ops("+++"), Ok(vec![Op::Plus(3)]));

        assert_eq!(parse_ops("---"), Ok(vec![Op::Minus(3)]));

        assert_eq!(parse_ops(">>>"), Ok(vec![Op::Right(3)]));

        assert_eq!(parse_ops("<<<"), Ok(vec![Op::Left(3)]));

        assert_eq!(
            parse_ops("[[]]"),
            Ok(vec![
                Op::LeftBracket(0),
                Op::LeftBracket(0),
                Op::RightBracket(0),
                Op::RightBracket(0)
            ])
        );
    }

    #[test]
    fn program() {
        assert_eq!(
            parse_program("+++---"),
            Ok(vec![Op::Plus(3), Op::Minus(3)])
        );

        assert_eq!(
            parse_program("+++---<<<>>>"),
            Ok(vec![
                Op::Plus(3),
                Op::Minus(3),
                Op::Left(3),
                Op::Right(3)
            ])
        );

        assert_eq!(
            parse_program("+++---<<<>>>[[]]"),
            Ok(vec![
                Op::Plus(3),
                Op::Minus(3),
                Op::Left(3),
                Op::Right(3),
                Op::LeftBracket(7),
                Op::LeftBracket(6),
                Op::RightBracket(5),
                Op::RightBracket(4)
            ])
        );
    }
}
