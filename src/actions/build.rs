use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, Options};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use image::imageops;
use serde::Deserialize;
use std::fmt::format;
use std::fs::{self, DirEntry, ReadDir};
use std::path::Path;
use std::str::Bytes;
use sha2::{Sha256, Digest};

use crate::actions::build::infos::ARTICLES_DIRECTORY;

#[path ="../infos.rs"]
mod infos;

#[path = "../checks.rs"]
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

    if fs::create_dir(infos::STAGING_DIRECTORY).is_err() {
        println!("Could not create the staging directory, quitting!");
    }

    if fs::create_dir(infos::get_staging_folder_path(infos::IMAGES_DIRECTORY)).is_err() {
        println!("Could not create the staging images directory, quitting!");
        return;
    }

    if fs::create_dir(infos::get_staging_folder_path(infos::ARTICLES_DIRECTORY)).is_err() {
        println!("Could not create the staging notes directory, quitting!");
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
    // let base_html = markdown_to_html(file_contents, options);
    // // this somehow seems better than going through an AST
    // let wrapped_tables = base_html.replace("<table>", "<div class=\"table_container\"><table>").replace("</table>", "</table></div>");
    // wrapped_tables
    let arena = Arena::new();
    let root = parse_document(&arena, file_contents, options);
    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::Image(ref mut img) => {
            let original_url = img.url.to_owned();
            let new_url = process_imgs(&original_url);
            std::mem::replace(&mut img.url, new_url);
        }
        _ => (),
    });

    let mut html = vec![];
    format_html(root, options, &mut html);
    let unwrapped_tables_html = String::from_utf8(html).unwrap();
    // maybe faster than going through the AST idk
    unwrapped_tables_html
        .replace("<table>", "<div class=\"table-container\">\n<table>")
        .replace("</table>", "</table>\n</div>")
}

// yoinked from the comrak docs :3c
fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

fn process_imgs(url: &str) -> String {
    println!("Processing image source: {}", url);
    if !url.starts_with(&format!("../{}", infos::IMAGES_DIRECTORY)) {
        println!("Doesn't seem to be a local image");
        url.to_string()
    } else {
        let md_path = Path::new(url).file_name().unwrap().to_str().unwrap();
        let builder_path = infos::get_file_path((infos::IMAGES_DIRECTORY, md_path));
        let mut hasher = Sha256::new();
        hasher.update(md_path.as_bytes());
        let out_filename = format!("{:x}.jpg", hasher.finalize());
        let staging_path = infos::get_staging_file_path((infos::IMAGES_DIRECTORY, &out_filename));

        if !Path::new(builder_path.as_str()).exists() {
            println!("Missing image {}", builder_path);
            return url.to_string();
        }

        if Path::new(&staging_path).exists() {
            println!("Image {} already exists in staging", staging_path);
            return html_image_path(&out_filename);
        }

        let res = image::open(builder_path.clone());
        if res.is_err() {
            println!("Could not open image {}", builder_path);
            return url.to_string();
        }
        let img = res.unwrap();
        img.resize(1920, 1080, imageops::Lanczos3);
        let saved = img.save(staging_path.clone());
        if saved.is_err() {
            println!("Could not save the resized image in {staging_path}");
            return url.to_string();
        }
        html_image_path(&out_filename)
    }
}

fn html_image_path(name: &str) -> String {
    format!("../images/{name}")
}
