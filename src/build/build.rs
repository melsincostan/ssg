#[path = "../checks/checks.rs"]
mod checks;

pub fn run() {
    if !checks::check_folders() {
        println!("Not all folders are OK, quitting!");
        return;
    }

    if !checks::check_files() {
        println!("Not all files are OK, quitting!");
        return;
    }
}
