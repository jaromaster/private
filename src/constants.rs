pub mod constants {

    /// define all valid modes / actions
    pub enum Actions {
        ENCRYPT,
        DECRYPT,
        HASH
    }

    // messages
    pub const PROMPT_PASSWD_MSG: &str = "Enter password: ";
    pub const INVALID_ARGUMENT_NUM_MSG: &str = "invalid number of arguments";
    pub const INVALID_ACTION_MSG: &str = "invalid action";
    pub const FILE_WRITE_ERR_MSG: &str = "could not write to file";
    pub const FILE_READ_ERR_MSG: &str = "could not read file";
    pub const DECRYPT_ERR_MSG: &str = "could not decrypt data";

    // actions (validate input)
    pub const ENCRYPT_STR: &str = "encrypt";
    pub const DECRYPT_STR: &str = "decrypt";
    pub const HASH_STR: &str = "hash";
}