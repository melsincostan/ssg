use std::fs;

#[path = "../infos.rs"]
mod infos;

pub fn run() {
    fs::create_dir(infos::BASE_DIRECTORY).expect("Could not create the main folder");
    infos::folders()
        .iter()
        .try_for_each(|e| fs::create_dir(e))
        .expect("could not create folders");
    infos::files().iter().for_each(|e| {
        fs::File::create(e).expect("could not create file");
    });
}
