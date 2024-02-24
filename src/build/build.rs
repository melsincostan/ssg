use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::fs::{self, DirEntry, ReadDir};
use comrak::{markdown_to_html, Options};

use crate::actions::build::infos::ARTICLES_DIRECTORY;

#[path = "../infos/infos.rs"]
mod infos;

#[path = "../checks/checks.rs"]
mod checks;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    tags: Vec<String>,
    date: String,
    author: String,
    edited: bool,
    edited_date: Option<String>,
}

pub fn run() {
    if !checks::check_folders() {
        println!("Not all folders are OK, quitting!");
        return;
    }

    if !checks::check_files() {
        println!("Not all files are OK, quitting!");
        return;
    }

    get_articles();
}

fn get_articles() {
    let matter_engine = Matter::<YAML>::new();
    let articles_dir_content = fs::read_dir(infos::get_folder_path(infos::ARTICLES_DIRECTORY));
    let dir_entries: ReadDir;
    if articles_dir_content.is_ok() {
        dir_entries = articles_dir_content.unwrap();
        for dir_entry in dir_entries {
            if dir_entry.is_ok() {
                process_article(dir_entry.unwrap(), &matter_engine);
            }
        }
    } else {
        println!("Could not read articles, quitting!");
        return;
    }
}

fn process_article(dir_entry: DirEntry, matter_engine: &Matter<YAML>) {
    let filename = dir_entry.file_name();
    let filetype = dir_entry.file_type();
    let mut options: Options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    if filetype.is_err() || filetype.unwrap().is_dir() {
        return;
    }
    println!("Article: {:?}", filename);
    let file_contents = fs::read_to_string(infos::get_file_path((
        ARTICLES_DIRECTORY,
        filename.to_str().unwrap(),
    )))
    .expect("Could not read article contents");
    let frontmatter = get_front_matter(&file_contents, matter_engine);
    let contents = process_contents(&file_contents, &options);
    println!("{}", contents);
}

fn get_front_matter(file_contents: &str, matter_engine: &Matter<YAML>) -> FrontMatter {
    matter_engine
        .parse_with_struct::<FrontMatter>(&file_contents)
        .expect("Could not parse front matter")
        .data
}

fn process_contents(file_contents: &str, options: &Options) -> String {
    markdown_to_html(file_contents, options)
}
