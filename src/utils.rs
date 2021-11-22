use serde::Serialize;

use std::fs;
use std::path::PathBuf;

pub struct Icon {
    pub html: String,
}

pub struct IconList {
    pub icons: Vec<Icon>
}

#[derive(Serialize)]
pub struct IconRows {
    pub rows: Vec<String>,
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
            match Icon::load_from_dir(dir.unwrap().path()) {
                Some(icon) => { digest.icons.push(icon); },
                None => {},
            }
        }
        digest
    }
    
    pub fn export_to_rows(&self, count: usize) -> IconRows {
        let mut digest = IconRows { rows : Vec::new() };
        let mut row = String::new();
        for (i, icon) in self.icons.iter().enumerate() {
            if i != 0 && i % count == 0 {
                digest.rows.push(row.clone());
                row = String::new();
            }
            row += &icon.html;
        }
        digest.rows.push(row.clone());
        digest
    }
}

