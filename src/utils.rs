use rocket_dyn_templates::tera::Tera;
use serde::Serialize;

use std::fs;
use std::path::PathBuf;

pub fn customize_tera(tera: &mut Tera) {
    tera.autoescape_on(vec![]);
}

pub trait Icon
where
    Self: Sized,
{
    fn load_from_dir(dir: PathBuf) -> Option<Self>;
    fn get_html(&self) -> String;
}

pub struct IconList<I>
where
    I: Icon,
{
    pub icons: Vec<I>,
}

#[derive(Serialize)]
pub struct Page {
    pub home_color: String,
    pub bio_color: String,
    pub project_color: String,
    pub blog_color: String,
    pub body: String,
}

impl<I> IconList<I>
where
    I: Icon,
{
    pub fn load_from_dir(dir: &str) -> Self {
        let mut digest = IconList { icons: Vec::new() };
        for dir in fs::read_dir(dir).unwrap() {
            match I::load_from_dir(dir.unwrap().path()) {
                Some(icon) => {
                    digest.icons.push(icon);
                }
                None => {}
            }
        }
        digest
    }

    pub fn export_to_html(&self, count: usize) -> String {
        let grid_start = "<div class=\"w3-row-padding\">";
        let mut digest = String::from(grid_start);
        let grid_close = "</div>";
        let mut row = String::new();
        for (i, icon) in self.icons.iter().enumerate() {
            if i != 0 && i % count == 0 {
                digest += &(row.clone());
                digest += grid_close;
                digest += grid_start;
                row = String::new();
            }
            row += &icon.get_html();
        }
        digest += &(row.clone());
        digest += grid_close;
        digest
    }
}

impl Page {
    pub fn new(index: &str) -> Self {
        let mut body = String::new();
        match fs::read_to_string(index) {
            Ok(content) => {
                body = String::from(content);
            }
            _ => {}
        }
        Page {
            home_color: String::from("w3-hover-black"),
            bio_color: String::from("w3-hover-black"),
            project_color: String::from("w3-hover-black"),
            blog_color: String::from("w3-hover-black"),
            body: body,
        }
    }
}
