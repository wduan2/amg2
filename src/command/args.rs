use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CommandArgName {
    Website,
    Username,
    OldUsername,
    Password,
    Unknown(String)
}

impl CommandArgName {
    fn from_string(arg: String) -> CommandArgName {
        match arg.as_str() {
            "--website" => CommandArgName::Website,
            "--username" => CommandArgName::Username,
            "--old-username" => CommandArgName::OldUsername,
            "--password" => CommandArgName::Password,
            _ => CommandArgName::Unknown(arg)
        }
    }
}

impl fmt::Display for CommandArgName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandArgName::Website => write!(f, "--website"),
            CommandArgName::Username => write!(f, "--username"),
            CommandArgName::OldUsername => write!(f, "--old-username"),
            CommandArgName::Password => write!(f, "--password"),
            CommandArgName::Unknown(v) => write!(f, "{v}")
        }
    }
}

pub struct CommandArgOption {
    name: CommandArgName,
    required: bool,
    has_value: bool
}

impl CommandArgOption {
    pub fn new(name: CommandArgName, required: bool, has_value: bool) -> CommandArgOption {
        CommandArgOption { name, required, has_value }
    }
}

pub struct CommandArgs {
    arg_options: Vec<CommandArgOption>,
    arg_map: HashMap<CommandArgName, Option<String>>
}

impl CommandArgs {
    // by default, an iterator returns the borrowing view (a reference), however, since we
    // want to construct a struct that holds the data returns by iterator, the 'next()' method
    // implementation must return a value (e.g. clone the data under the hood)
    pub fn build(
        args: Peekable<IntoIter<String>>,
        arg_options: Vec<CommandArgOption>
    ) -> CommandArgs {

        let arg_map = get_arg_map(args);

        validate(&arg_map, &arg_options);

        return CommandArgs { arg_map, arg_options }
    }
}

fn get_arg_map(mut args: Peekable<IntoIter<String>>) -> HashMap<CommandArgName, Option<String>> {
    let mut arg_map = HashMap::new();
    loop {
        if let Some(arg) = args.next() {
            if let Some(next_arg) = args.peek() {
                if next_arg.starts_with("--") {
                    arg_map.insert(CommandArgName::from_string(arg), None);
                } else {
                    arg_map.insert(CommandArgName::from_string(arg), Some(args.next().unwrap()));
                }
            } else {
                arg_map.insert(CommandArgName::from_string(arg), None);
            }
        } else {
            break;
        }
    }
    return arg_map;
}

fn validate(
    arg_map: &HashMap<CommandArgName, Option<String>>,
    arg_options: &Vec<CommandArgOption>
) {
    let mut errors = Vec::new();
    for arg_type in arg_options.iter() {
        if let Some(arg) = arg_map.get(&arg_type.name) {
            if arg.is_none() && arg_type.has_value {
                errors.push(format!("Missing value for argument {arg_name}", arg_name = arg_type.name));
            }
        } else if arg_type.required {
            errors.push(format!("Missing required argument: {arg_name}", arg_name = arg_type.name));
        }
    }
    for arg in arg_map.keys() {
        if let CommandArgName::Unknown(v) = arg {
            errors.push(format!("Unknown argument: {v}"))
        }
    }

    if !errors.is_empty() {
        panic!("{error_message}", error_message = errors.join("\n"));
    }
}
