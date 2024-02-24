pub fn main_folder() -> String {
    "site".to_string()
}

pub fn folders() -> Vec<String> {
    unprefixed_folders()
        .iter()
        .map(|e| {
            let main_folder = main_folder();
            format!("{main_folder}/{e}")
        })
        .collect()
}

pub fn files() -> Vec<String> {
    unprefixed_files()
        .iter()
        .map(|e| {
            let main_folder = main_folder();
            format!("{main_folder}/{e}")
        })
        .collect()
}

fn unprefixed_folders() -> Vec<String> {
    let folders = vec!["templates", "style", "static", "articles"];
    folders.iter().map(|e| e.to_string()).collect()
}

fn unprefixed_files() -> Vec<String> {
    let files = vec![
        "templates/article.hbs",
        "templates/main.hbs",
        "templates/list.hbs",
        "style/tailwind.config.js",
        "style/package.json",
        "style/style.css",
    ];
    files.iter().map(|e| e.to_string()).collect()
}