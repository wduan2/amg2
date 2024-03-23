use std::iter::Peekable;
use std::vec::IntoIter;

use super::args::{CommandArgOption, CommandArgName, CommandArgs};

pub fn create(args: Peekable<IntoIter<String>>) {
    let arg_options = vec!(
        CommandArgOption::new(CommandArgName::Website, true, true),
        CommandArgOption::new(CommandArgName::Username, true, true),
        CommandArgOption::new(CommandArgName::Password, true, true),
    );
    let args = CommandArgs::build(args, arg_options);

    // TODO: execute with the args

    ()
}
