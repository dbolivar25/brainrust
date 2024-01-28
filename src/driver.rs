use nom::error::convert_error;

use crate::parser::*;
use crate::vm::*;

pub struct Driver;

impl Driver {
    pub fn run(input: &str) {
        let ops = match parse_program(input) {
            Ok(ops) => ops,
            Err(e) => {
                e.map(|err| convert_error(input, err));
                return;
            }
        };

        if let Err(err) = VM::new().with_program(ops).build().run() {
            println!("Error: {}", err);
        }
    }
}
