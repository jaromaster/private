pub mod crypt {
    use std::io::{ErrorKind, Error};

    use hex_string::HexString;
    use sha3::{Sha3_256, Digest};

    use crate::constants::constants::Actions;


    /// execute given action on file (encrypt, decrypt)
    pub fn action_on_file(action: &Actions, path: &str, key: &str) -> Option<Error> {
        // read file
        let data_result = std::fs::read(path);
        if data_result.is_err() {
            return Some(Error::new(ErrorKind::InvalidInput, format!("could not read file '{}'", path)));
        }
        let data = data_result.unwrap();

        // execute action
        let result_data;
        match action {
            Actions::ENCRYPT => result_data = encrypt(key, &data),
            Actions::DECRYPT => result_data = decrypt(key, &data),
            Actions::HASH => {
                result_data = hash_sha3(&data);
                println!("{}: {}",  path, vec_to_hex_string(&result_data));
                return None;
            }
        };

        // write to file
        let write_result = std::fs::write(path, result_data);
        if write_result.is_err() {
            return Some(Error::new(ErrorKind::InvalidInput, format!("could not write to file '{}'", path)));
        }

        return None;
    }

    /// encrypt data and return as byte vector
    fn encrypt(key: &str, data: &Vec<u8>) -> Vec<u8> {
        let prepared_key = prepare_key(key);
        let f = fernet::Fernet::new(&prepared_key).unwrap();
        let encrypted_string = f.encrypt(data);
        let encrypted_data = encrypted_string.as_bytes();

        return encrypted_data.to_owned();
    }

    /// decrypt data and return it
    fn decrypt(key: &str, data: &Vec<u8>) -> Vec<u8> {
        let prepared_key = prepare_key(key);
        let f = fernet::Fernet::new(&prepared_key).unwrap();
        let decrypted_data = f.decrypt(&String::from_utf8(data.to_vec()).unwrap()).expect("could not decrypt data");

        return decrypted_data;
    }

    /// hash data using sha3 algorithm
    fn hash_sha3(data: &Vec<u8>) -> Vec<u8> {
        let mut sha3_hasher = Sha3_256::new();
        sha3_hasher.update(data);

        return sha3_hasher.finalize().to_vec();
    }

    /// convert byte vector to hex string
    fn vec_to_hex_string(data: &Vec<u8>) -> String {
        return HexString::from_bytes(data).as_string();
    }

    /// fill key until size 32 reached and encode as base64
    fn prepare_key(key: &str) -> String {
        let mut new_key = String::from(key);
        let fill_char = '.';

        while new_key.len() < 32 {
            new_key.push(fill_char);
        }

        return base64::encode(new_key);
    }

}