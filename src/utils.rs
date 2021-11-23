
use rocket_dyn_templates::tera::Tera;
use serde::Serialize;

use std::fs;
use std::path::PathBuf;

pub fn customize_tera(tera: &mut Tera) {
    tera.autoescape_on(vec![]);
}

#[derive(Debug)]
pub struct Icon {
    pub html: String,
}

pub struct IconList {
    pub icons: Vec<Icon>
}

#[derive(Serialize)]
pub struct Page {
    pub home_color: String,
    pub bio_color: String,
    pub project_color: String,
    pub blog_color: String,
    pub body: String,
}


impl Icon {
    pub fn load_from_dir(mut dir: PathBuf) -> Option<Self> {
        dir.push("icon.html");
        match fs::read_to_string(dir.as_path().to_str().unwrap()) {
            Ok(content) => Some(Icon { html : content }),
            _ => None
        }
    }
}

impl IconList {
    pub fn load_from_dir(dir: &str) -> Self {
        let mut digest = IconList { icons: Vec::new()};
        for dir in fs::read_dir(dir).unwrap() {
            println!( "{:?}", dir );
            match Icon::load_from_dir(dir.unwrap().path()) {
                Some(icon) => { println!("{:?}", icon); digest.icons.push(icon); },
                None => {},
            }
        }
        digest
    }
    
    pub fn export_to_html(&self, count: usize) -> String {
        let grid_start = "<div class=\"w3-row-padding\">";
        let mut digest = String::from(grid_start);
        let grid_close = "</div>";
        let mut row = String::new();
        println!( "{:?}", self.icons );
        for (i, icon) in self.icons.iter().enumerate() {
            if i != 0 && i % count == 0 {
                digest += &(row.clone());
                digest += grid_close;
                digest += grid_start;
                row = String::new();
            }
            println!( "{}", icon.html );
            row += &icon.html;
        }
        digest += &(row.clone());
        digest += grid_close;
        println!( "{}", digest );
        digest
    }
}

impl Page {
    pub fn new() -> Self {
        Page {
            home_color: String::from("w3-hover-black"),
            bio_color: String::from("w3-hover-black"),
            project_color: String::from("w3-hover-black"),
            blog_color: String::from("w3-hover-black"),
            body: String::new(),
        }
    }
}
