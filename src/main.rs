use constants::constants::Actions;
use crypt::crypt::action_on_file;

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
        let target_path = std::path::Path::new(&target);
        if target_path.is_file() {
            handle_file(&action, &target, &key);
        }
        else {
            println!("encountered dir: {}", target);
            handle_dir(&action, &target, &key);
        }
    }
}


/// execute action, on files only
fn handle_file(action: &Actions, file_path: &str, key: &str) {
    let err_option = action_on_file(&action, &file_path, &key);
    if err_option.is_some() {
        eprintln!("{}", err_option.unwrap());
        std::process::exit(1);
    }
}

/// execute action on all files in directory (recursively)
fn handle_dir(action: &Actions, dir_path: &str, key: &str) {
    for entry in std::fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();

        if entry.path().is_dir() {
            handle_dir(action, entry.path().to_str().unwrap(), key);
            continue;
        }
        handle_file(action, entry.path().to_str().unwrap(), key);
    }
}