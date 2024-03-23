use std::collections::HashMap;
use std::io::Error;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::arg_option_map;

use super::args::{CommandArgOption, CommandArgs, WEBSITE_ARG, USERNAME_ARG, PASSWORD_ARG};

pub fn create(raw_args: Peekable<IntoIter<String>>) -> Result<(), Error>{
    let arg_option_map = arg_option_map!(
        WEBSITE_ARG, true, true,
        USERNAME_ARG, true, true,
        PASSWORD_ARG, true, true
    );

    let args = CommandArgs::build(raw_args, arg_option_map)?;

    // TODO: execute with the args

    Ok(())
}
