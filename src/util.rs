use super::constant;

use std::path::PathBuf;

use rand::{distributions::Alphanumeric, Rng};

pub fn get_random_string(length: u32) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
}

pub fn generate_file_path(fdir: PathBuf, extension: &str) -> PathBuf {
    let rand_name = format!(
        "{}{}",
        get_random_string(constant::FILE_NAME_LEN),
        extension
    );
    return fdir.join(rand_name);
}
