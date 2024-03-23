use std::{env, process};
use std::error::Error;
use crate::command::handlers::create;

mod main_test;
mod macros;
mod command;

fn main() {
    let err = std::panic::catch_unwind(|| {
        let input: Vec<String> = env::args().collect();
        execute_with_args(input);
    });
}

fn execute_with_args(input: Vec<String>) {
    let mut raw_args = input.into_iter().peekable();
    // drop the first argument as it will always be the name of the program
    raw_args.next();
    if let Some(command) = raw_args.next() {
        match command.as_str() {
            "create" => create(raw_args),
            _ => panic!("Unknown command: {command}")
        }
    }
}
