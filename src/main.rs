use std::{env, process};
use std::error::Error;
use crate::command::handlers::create;

mod main_test;
mod macros;
mod command;

fn main() {
    let err = std::panic::catch_unwind(|| {
        let args: Vec<String> = env::args().collect();
        execute_with_args(args);
    });
}

fn execute_with_args(args: Vec<String>) {
    let mut iargs = args.iter().peekable();
    // skip the first argument as it will always be the name of the program
    iargs.next();
    if let Some(command) = iargs.next() {
        match command.as_str() {
            "create" => create(iargs),
            _ => panic!("Unknown command: {command}")
        }
    }
}
