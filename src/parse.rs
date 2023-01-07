pub mod parse {
    use std::{io::ErrorKind, io::Error};

    use crate::constants::constants::{Actions, INVALID_ARGUMENT_NUM_MSG, INVALID_ACTION_MSG, ENCRYPT_STR, DECRYPT_STR, HASH_STR};


    /// parse and check command arguments, then return actions and targets
    pub fn parse_args(args: std::env::Args) -> Result<(Actions, Vec<String>), Error> {

        if args.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidInput, INVALID_ARGUMENT_NUM_MSG));
        }

        let args_vec: Vec<String> = args.collect();

        // check action
        let action_string = &args_vec[1];
        let action;
        if action_string == ENCRYPT_STR {
            action = Actions::ENCRYPT;
        }
        else if action_string == DECRYPT_STR {
            action = Actions::DECRYPT;
        }
        else if action_string == HASH_STR {
            action = Actions::HASH;
        }
        else {
            return Err(Error::new(ErrorKind::InvalidInput, format!("{} '{}'", INVALID_ACTION_MSG, action_string)));
        }

        // get targets
        let targets = args_vec.iter().skip(2).map(|el| el.to_owned()).collect();

        return Ok((action, targets));
    }
}