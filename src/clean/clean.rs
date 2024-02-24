use std::fs;

#[path = "../infos/infos.rs"]
mod infos;

pub fn run() {
    infos::files()
        .iter()
        .for_each(|e| {
            let err = fs::remove_file(e).err();
            if err.is_some() {
                println!("Couldn't delete file {e}: {}", err.unwrap().to_string());
            }
        });
    infos::folders()
        .iter()
        .for_each(|e| {
            let err = fs::remove_dir(e).err();
            if err.is_some() {
                println!("Couldn't delete directory {e}: {}", err.unwrap().to_string());
            }
        });
    let err = fs::remove_dir(infos::BASE_DIRECTORY).err();
    if err.is_some() {
        println!("Couldn't delete main directory {}: {}", infos::BASE_DIRECTORY, err.unwrap().to_string());
    }
}
