use crypt::crypt::{action_on_file};

use crate::parse::parse::parse_args;

pub mod crypt;
pub mod parse;
pub mod constants;

fn main() {

    // SYNTAX: private ACTION TARGET1 TARGET2 ...
    let args_result = parse_args(std::env::args());
    if args_result.is_err() {
        eprintln!("{}", args_result.err().unwrap());
        std::process::exit(1);
    }

    let (action, targets) = args_result.unwrap();
    let key = rpassword::prompt_password("Enter password: ").unwrap();

    // execute action on targets (e.g. encrypt or decrypt)
    for target in targets {
        let err_option = action_on_file(&action, &target, &key);
        if err_option.is_some() {
            eprintln!("{}", err_option.unwrap());
            std::process::exit(1);
        }
    }
}
