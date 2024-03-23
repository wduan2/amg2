use std::collections::{HashMap};
use std::io::{Error, ErrorKind};
use std::iter::Peekable;
use std::vec::IntoIter;
use regex::Regex;

const ARG_PREFIX_REGEX: &str = r"^(--|-)";
pub const WEBSITE_ARG: &str = "--website";
pub const USERNAME_ARG: &str = "--username";
pub const OLD_USERNAME_ARG: &str = "--old-username";
pub const PASSWORD_ARG: &str = "--password";

pub struct CommandArgOption {
    name: String,
    required: bool,
    has_value: bool
}

impl CommandArgOption {
    pub fn new(name: &str, required: bool, has_value: bool) -> CommandArgOption {
        CommandArgOption { name: String::from(name), required, has_value }
    }
}

pub struct CommandArgs {
    arg_option_map: HashMap<String, CommandArgOption>,
    arg_map: HashMap<String, Option<String>>
}

impl CommandArgs {
    pub fn build(
        raw_args: Peekable<IntoIter<String>>,
        arg_option_map: HashMap<String, CommandArgOption>
    ) -> Result<CommandArgs, Error> {

        let arg_map = get_arg_map(raw_args);

        validate(&arg_map, &arg_option_map)?;

        return Ok(CommandArgs { arg_map, arg_option_map })
    }
}

fn get_arg_map(mut raw_args: Peekable<IntoIter<String>>) -> HashMap<String, Option<String>> {
    let arg_regex = Regex::new(ARG_PREFIX_REGEX).unwrap();
    let mut arg_map = HashMap::new();
    loop {
        if let Some(arg) = raw_args.next() {
            if let Some(next_arg) = raw_args.peek() {
                if arg_regex.is_match(next_arg) {
                    arg_map.insert(arg, None);
                } else {
                    arg_map.insert(arg, Some(raw_args.next().unwrap()));
                }
            } else {
                arg_map.insert(arg, None);
            }
        } else {
            break;
        }
    }
    return arg_map;
}

fn validate(
    arg_map: &HashMap<String, Option<String>>,
    arg_option_map: &HashMap<String, CommandArgOption>
) -> Result<(), Error> {
    let mut errors = Vec::new();
    for arg_type in arg_option_map.values() {
        if let Some(arg) = arg_map.get(&arg_type.name) {
            if arg.is_none() && arg_type.has_value {
                errors.push(format!("Missing value for argument {arg_name}", arg_name = arg_type.name));
            }
        } else if arg_type.required {
            errors.push(format!("Missing required argument: {arg_name}", arg_name = arg_type.name));
        }
    }

    for arg in arg_map.keys() {
        if !arg_option_map.contains_key(arg) {
            errors.push(format!("Unknown argument: {arg}"))
        }
    }

    if !errors.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, errors.join("\n")));
    }

    Ok(())
}
