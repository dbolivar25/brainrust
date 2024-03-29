mod driver;
mod ops;
mod parser;
mod vm;

use driver::*;

use anyhow::Result;
use clap::Parser;
use std::{fs, io::Write};

// argument parser
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Set the file to interpret
    #[arg(short, long, default_value = None)]
    file: Option<String>,
}

#[derive(Debug)]
struct App;

impl App {
    pub fn run_file_interpreter(file: String) -> Result<()> {
        let file_string = fs::read_to_string(file)?;

        Driver::run(&file_string);

        Ok(())
    }

    pub fn run_repl_interpreter() -> Result<()> {
        let mut input = String::new();
        let mut read_buffer = String::new();

        println!();
        loop {
            loop {
                print!("|>  ");
                std::io::stdout().flush()?;
                std::io::stdin().read_line(&mut read_buffer)?;

                input.push_str(&read_buffer);

                // allow multiline input by the user entering an empty line to end the input
                match read_buffer.trim() {
                    "" | "q" | "quit" => break,
                    _ => (),
                }

                read_buffer.clear();
            }

            match input.trim() {
                "q" | "quit" => break,
                input => Driver::run(input),
            }

            println!();

            input.clear();
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.file {
        Some(file) => App::run_file_interpreter(file)?,
        None => App::run_repl_interpreter()?,
    };

    Ok(())
}
