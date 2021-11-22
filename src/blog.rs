
//use rocket::response::Redirect;

use rocket_dyn_templates::Template;//, tera::Tera, context};
use serde::Serialize;

use std::fs;
use std::path::PathBuf;

struct BlogIcon {
    pub html: String,
}

#[derive(Serialize)]
struct IconRows {
    pub rows: Vec<String>,
}



#[get("/")]
pub fn index() -> Template {
    let mut icons: Vec<BlogIcon> = Vec::new();
    for dir in fs::read_dir("site/blog/posts").unwrap() {
        match BlogIcon::load_from_dir(dir.unwrap().path()) {
            Some(icon) => { icons.push(icon); },
            None => {},
        }
    }
    let mut posts = IconRows { rows : Vec::new() };
    let mut row = String::new();
    for (i, icon) in icons.iter().enumerate() {
        if i != 0 && i % 3 == 0 {
            posts.rows.push(row.clone());
            row = String::new();
        }
        row += &icon.html;
    }
    posts.rows.push(row.clone());
    Template::render("blog/index", posts )
}


impl BlogIcon {
    pub fn load_from_dir(mut dir: PathBuf) -> Option<Self> {
        dir.push("icon.html");
        match fs::read_to_string(dir.as_path().to_str().unwrap()) {
            Ok(content) => Some(BlogIcon { html : content }),
            _ => None
        }
    }
}
