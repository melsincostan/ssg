use chrono::{DateTime, Utc};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, Options};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use handlebars::Handlebars;
use image::imageops;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::format;
use std::fs::{self, DirEntry, ReadDir};
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;
use urlencoding::encode;

use crate::actions::build::infos::ARTICLES_DIRECTORY;

use self::infos::get_file_path;

#[path = "../infos.rs"]
mod infos;

#[path = "../checks.rs"]
mod checks;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    tagline: String,
    tags: Vec<String>,
    date: String,
    author: String,
    edited: bool,
    lang: String,
    edited_date: Option<String>,
}

#[derive(Serialize, Debug)]
struct Article {
    title: String,
    tagline: String,
    tags: String,
    date: String,
    author: String,
    lang: String,
    article: String,
    generated: String,
    stylesheet: String,
}

#[derive(Serialize, Debug)]
struct ArticleList {
    article_cards: Vec<String>,
    generated: String,
    stylesheet: String,
}

#[derive(Serialize, Debug)]
struct ArticleCard {
    article_link: String,
    title: String,
    tagline: String,
    tags: String,
    date: String,
}

#[derive(Serialize, Debug)]
struct Main {
    generated: String,
    stylesheet: String,
}

pub fn run() {
    let time = SystemTime::now();
    let timestamp: DateTime<Utc> = time.into();

    if !checks::check_folders() {
        println!("Not all folders are OK, quitting!");
        return;
    }

    if !checks::check_files() {
        println!("Not all files are OK, quitting!");
        return;
    }

    if fs::remove_dir_all(infos::STAGING_DIRECTORY).is_err() {
        println!("Could not clear the staging directory, trying to continue");
    }

    if fs::create_dir(infos::STAGING_DIRECTORY).is_err() {
        println!("Could not create the staging directory, quitting!");
        return;
    }

    if fs::create_dir(infos::get_staging_folder_path(infos::IMAGES_DIRECTORY)).is_err() {
        println!("Could not create the staging images directory, quitting!");
        return;
    }

    if fs::create_dir(infos::get_staging_folder_path(infos::ARTICLES_DIRECTORY)).is_err() {
        println!("Could not create the staging notes directory, quitting!");
        return;
    }

    let articles_template =
        fs::read_to_string(infos::get_file_path(infos::ARTICLE_TEMPLATE)).unwrap();
    let main_template = fs::read_to_string(infos::get_file_path(infos::MAINPAGE_TEMPLATE)).unwrap();
    let list_template =
        fs::read_to_string(infos::get_file_path(infos::ARTICLELIST_TEMPLATE)).unwrap();
    let card_template =
        fs::read_to_string(infos::get_file_path(infos::ARTICLECARD_TEMPLATE)).unwrap();
    let mut hbs = Handlebars::new();

    hbs.register_template_string("article", articles_template)
        .expect("Could not register articles template");
    hbs.register_template_string("main", main_template)
        .expect("Could not register main page template");
    hbs.register_template_string("list", list_template)
        .expect("Could not register list template");
    hbs.register_template_string("card", card_template)
        .expect("Could not register card template");

    let mut hasher = Sha256::new();
    hasher.update(
        fs::read_to_string(get_file_path(infos::STYLESHEET))
            .unwrap()
            .as_bytes(),
    );
    let stylesheet_output_name = format!("main.{:x}.css", hasher.finalize());

    let articles_wrapped = get_articles(stylesheet_output_name.clone());
    if articles_wrapped.is_none() {
        println!("Could not parse articles, quitting!");
        return;
    }

    let mut cards: Vec<String> = vec![];
    let articles = articles_wrapped.unwrap();

    for article in articles {
        let full_html = hbs
            .render("article", &article)
            .expect("Could not parse article into template");
        let article_filename = format!("{}-{}.html", article.date, article.title.to_lowercase());
        let path = Path::new(&infos::get_staging_folder_path(infos::ARTICLES_DIRECTORY))
            .join(article_filename.clone());
        let articleCard = hbs
            .render(
                "card",
                &ArticleCard {
                    article_link: article_filename,
                    title: article.title,
                    tagline: article.tagline,
                    date: article.date,
                    tags: article.tags,
                },
            )
            .expect("Could not render article card");
        cards.push(articleCard);
        fs::write(path, full_html);
    }

    let list_page_full = hbs
        .render(
            "list",
            &ArticleList {
                article_cards: cards,
                generated: timestamp.to_rfc3339(),
                stylesheet: format!("../{stylesheet_output_name}"),
            },
        )
        .expect("Could not render list page");

    let main_page_full = hbs
        .render(
            "main",
            &Main {
                generated: timestamp.to_rfc3339(),
                stylesheet: stylesheet_output_name.clone(),
            },
        )
        .expect("Could not render main page");

    let main_path = Path::new(&infos::STAGING_DIRECTORY).join("index.html");
    let list_path = infos::get_staging_file_path((infos::ARTICLES_DIRECTORY, "index.html"));
    fs::write(main_path, main_page_full);
    fs::write(list_path, list_page_full);

    // TODO: staging directory hardcoded in the tailwind config...
    // generate the css using npx.
    // Needs npx, will get latest tailwind from npm
    let css_output = Path::new(infos::STAGING_DIRECTORY).join(stylesheet_output_name);
    let css_output_path = css_output.to_str().unwrap();
    Command::new("npx")
        .arg("-y")
        .arg("tailwindcss")
        .arg("-c")
        .arg(infos::get_file_path(infos::TAILWIND_CONFIG))
        .arg("-i")
        .arg(infos::get_file_path(infos::STYLESHEET))
        .arg("-o")
        .arg(css_output_path)
        .arg("--minify")
        .status()
        .expect("Could not create tailwind CSS");
}

fn get_articles(stylesheet: String) -> Option<Vec<Article>> {
    let matter_engine = Matter::<YAML>::new();
    let articles_dir_content = fs::read_dir(infos::get_folder_path(infos::ARTICLES_DIRECTORY));
    let dir_entries: ReadDir;
    let mut articles: Vec<Article> = vec![];
    let time = SystemTime::now();
    let timestamp: DateTime<Utc> = time.into();

    if articles_dir_content.is_ok() {
        dir_entries = articles_dir_content.unwrap();
        for dir_entry in dir_entries {
            if dir_entry.is_ok() {
                let res = process_article(dir_entry.unwrap(), &matter_engine);
                if res.is_some() {
                    let article_raw = res.unwrap();
                    let article = Article {
                        title: article_raw.0.title,
                        tagline: article_raw.0.tagline,
                        tags: article_raw.0.tags.join(", "),
                        author: article_raw.0.author,
                        date: format!(
                            "{}{}",
                            article_raw.0.date,
                            if article_raw.0.edited {
                                format!(" (Edited {})", article_raw.0.edited_date.unwrap())
                            } else {
                                "".to_string()
                            }
                        ),
                        lang: article_raw.0.lang,
                        article: article_raw.1,
                        generated: timestamp.to_rfc3339(),
                        stylesheet: format!("../{stylesheet}"),
                    };
                    articles.push(article);
                }
            }
        }
    } else {
        println!("Could not read articles, quitting!");
        return None;
    }
    articles.sort_by(|a, b| {
        // return the newest articles first. This assumes that dates are in YYYY-MM-DD format...
        b.date.cmp(&a.date)
    });
    Some(articles)
}

fn process_article(
    dir_entry: DirEntry,
    matter_engine: &Matter<YAML>,
) -> Option<(FrontMatter, String)> {
    let filename = dir_entry.file_name();
    let filetype = dir_entry.file_type();
    let mut options: Options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    if filetype.is_err() || filetype.unwrap().is_dir() {
        return None;
    }
    println!("Article: {:?}", filename);
    let file_contents = fs::read_to_string(infos::get_file_path((
        ARTICLES_DIRECTORY,
        filename.to_str().unwrap(),
    )))
    .expect("Could not read article contents");
    let frontmatter = get_front_matter(&file_contents, matter_engine);
    let contents = process_contents(&file_contents, &options);
    Some((frontmatter, contents))
}

fn get_front_matter(file_contents: &str, matter_engine: &Matter<YAML>) -> FrontMatter {
    matter_engine
        .parse_with_struct::<FrontMatter>(&file_contents)
        .expect("Could not parse front matter")
        .data
}

fn process_contents(file_contents: &str, options: &Options) -> String {
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
    format_html(root, options, &mut html).expect("Could not parse markdown");
    let unwrapped_tables_html = String::from_utf8(html).unwrap();
    // maybe faster than going through the AST idk probably not but algorithms
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
        img.resize(1920, 1080, imageops::Lanczos3); // this should nuke exif data as well, nice!
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
