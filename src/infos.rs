use std::path::Path;

pub static BASE_DIRECTORY: &str = "site";

pub static STAGING_DIRECTORY: &str = "staging"; // TODO: staging directory hardcoded in the tailwind config...

pub static TEMPLATES_DIRECTORY: &str = "templates";
pub static STYLE_DIRECTORY: &str = "style";
pub static IMAGES_DIRECTORY: &str = "images";
pub static ARTICLES_DIRECTORY: &str = "notes";

pub static ARTICLE_TEMPLATE: (&str, &str) = (TEMPLATES_DIRECTORY, "article.hbs");
pub static MAINPAGE_TEMPLATE: (&str, &str) = (TEMPLATES_DIRECTORY, "main.hbs");
pub static ARTICLELIST_TEMPLATE: (&str, &str) = (TEMPLATES_DIRECTORY, "list.hbs");
pub static ARTICLECARD_TEMPLATE: (&str, &str) = (TEMPLATES_DIRECTORY, "card.hbs");

pub static TAILWIND_CONFIG: (&str, &str) = (STYLE_DIRECTORY, "tailwind.config.js");
pub static STYLESHEET: (&str, &str) = (STYLE_DIRECTORY, "style.css");

pub fn get_file_path(file: (&str, &str)) -> String {
    Path::new(BASE_DIRECTORY)
        .join(file.0)
        .join(file.1)
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_staging_file_path(file: (&str, &str)) -> String {
    Path::new(STAGING_DIRECTORY)
        .join(file.0)
        .join(file.1)
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_folder_path(folder: &str) -> String {
    Path::new(BASE_DIRECTORY)
        .join(folder)
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_staging_folder_path(folder: &str) -> String {
    Path::new(STAGING_DIRECTORY)
        .join(folder)
        .to_str()
        .unwrap()
        .to_string()
}

pub fn folders() -> Vec<String> {
    vec![
        TEMPLATES_DIRECTORY,
        STYLE_DIRECTORY,
        IMAGES_DIRECTORY,
        ARTICLES_DIRECTORY,
    ]
    .iter()
    .map(|e| get_folder_path(e))
    .collect()
}

pub fn files() -> Vec<String> {
    vec![
        MAINPAGE_TEMPLATE,
        ARTICLE_TEMPLATE,
        ARTICLECARD_TEMPLATE,
        ARTICLELIST_TEMPLATE,
        TAILWIND_CONFIG,
        STYLESHEET,
    ]
    .iter()
    .map(|e| get_file_path(*e))
    .collect()
}
