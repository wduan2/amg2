use std::iter::Peekable;
use std::vec::IntoIter;

use super::args::{CommandArgOption, CommandArgName, CommandArgs, WEBSITE_ARG, USERNAME_ARG, PASSWORD_ARG};

pub fn create(args: Peekable<IntoIter<String>>) {
    let arg_options = vec!(
        CommandArgOption::new(CommandArgName::Website(WEBSITE_ARG), true, true),
        CommandArgOption::new(CommandArgName::Username(USERNAME_ARG), true, true),
        CommandArgOption::new(CommandArgName::Password(PASSWORD_ARG), true, true),
    );
    let args = CommandArgs::build(args, arg_options);

    // TODO: execute with the args

    ()
}
