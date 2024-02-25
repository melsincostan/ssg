use std::fs;

#[path = "../infos.rs"]
mod infos;

pub fn run() {
    let err = fs::remove_dir_all(infos::BASE_DIRECTORY).err();
    if err.is_some() {
        println!("Couldn't delete main directory {}: {}", infos::BASE_DIRECTORY, err.unwrap().to_string());
    }
}
