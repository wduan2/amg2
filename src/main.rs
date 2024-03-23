use std::{env, process};
use std::io::{Error, ErrorKind};
use crate::command::handlers::create;

mod main_test;
mod macros;
mod command;

fn main() {
    let r = execute_with_args(env::args().collect());
    match r {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Failed to execute command due to {e}");
            process::exit(1)
        }
    }
}

fn execute_with_args(input: Vec<String>) -> Result<String, Error> {
    let mut raw_args = input.into_iter().peekable();
    // drop the first argument as it will always be the name of the program
    raw_args.next();
    return match raw_args.next() {
        Some(command) => {
            match command.as_str() {
                "create" => create(raw_args)?,
                _ => return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown command: {command}")))
            }
            Ok(format!("Command: '{command}' executed"))
        },
        _ => Ok(String::from("No command to specified"))
    }
}
