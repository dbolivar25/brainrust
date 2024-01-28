mod ops;
mod parser;
mod vm;
mod driver;

use driver::*;

fn main() {
    // Driver::run(include_str!("../examples/hello.bf"));
    Driver::run(include_str!("../examples/rot13.bf"));
}
