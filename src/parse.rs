pub mod parse {
    use std::{io::ErrorKind, io::Error};

    use crate::constants::constants::Actions;


    /// parse and check command arguments, then return actions and targets
    pub fn parse_args(args: std::env::Args) -> Result<(Actions, Vec<String>), Error> {

        if args.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid number of arguments"));
        }

        let args_vec: Vec<String> = args.collect();

        // check action
        let action_string = &args_vec[1];
        let action;
        if action_string == "encrypt" {
            action = Actions::ENCRYPT;
        }
        else if action_string == "decrypt" {
            action = Actions::DECRYPT;
        }
        else if action_string == "hash" {
            action = Actions::HASH;
        }
        else {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid action"));
        }

        // get targets
        let targets = args_vec.iter().skip(2).map(|el| el.to_owned()).collect();

        return Ok((action, targets));
    }
}