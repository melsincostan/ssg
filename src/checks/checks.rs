use std::path::Path;

#[path = "../infos/infos.rs"]
mod infos;

pub fn check_folders() -> bool {
    infos::folders()
        .iter()
        .map(|e| check_entity(e.to_string()))
        .all(|res| res)
}

pub fn check_files() -> bool {
    infos::files()
        .iter()
        .map(|e| check_entity(e.to_string()))
        .all(|res| res)
}

fn check_entity(name: String) -> bool {
    let result = Path::new(&name).exists();
    println!("./{}: {}", name, if result { "OK" } else { "ERR" });
    result
}
