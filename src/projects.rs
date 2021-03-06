use super::utils::{Icon, IconList, Page};

use rocket_dyn_templates::Template;

use std::fs;
use std::path::PathBuf;

struct ProjectIcon {
    html: String,
    #[allow(dead_code)]
    ordering: u32,
}

impl Icon for ProjectIcon {
    fn load_from_dir(mut dir: PathBuf) -> Option<Self> {
        dir.push("ordering.txt");
        let ord = match fs::read_to_string(dir.as_path().to_str().unwrap()) {
            Ok(mut val) => {
                val.pop();
                match val.parse::<u32>() {
                Ok(v) => v,
                _ => 0,
            }
            },
            _ => 0,
        };
        dir.pop();
        dir.push("icon.html");
        match fs::read_to_string(dir.as_path().to_str().unwrap()) {
            Ok(content) => Some(ProjectIcon {
                html: content,
                ordering: ord,
            }),
            _ => None,
        }
    }

    fn get_html(&self) -> String {
        self.html.clone()
    }
}

#[get("/<path>/about.html")]
pub async fn about(path: String) -> Template {
    let mut page = Page::new(&format!("site/projects/listings/{}/about.html", path));
    page.project_color = String::from("w3-black");
    Template::render("base", page)
}

#[get("/")]
pub fn root() -> Template {
    let mut page = Page::new("site/projects/index.html");
    page.project_color = String::from("w3-black");
    let mut icons: IconList<ProjectIcon> = IconList::load_from_dir("site/projects/listings/");
    icons.icons.sort_by(|a, b| b.ordering.cmp(&a.ordering));
    page.body = icons.export_to_html(3);
    Template::render("base", page)
}
