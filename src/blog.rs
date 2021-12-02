use super::utils::{Icon, IconList, Page};

use rocket_dyn_templates::Template;

use std::fs;
use std::path::PathBuf;

struct BlogIcon {
    html: String,
    ordering: u32,
}

impl Icon for BlogIcon {
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
            Ok(content) => Some(BlogIcon {
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

#[get("/posts/<path>/post.html")]
pub async fn get_post(path: String) -> Template {
    let mut page = Page::new(&format!("site/blog/posts/{}/post.html", path));
    page.blog_color = String::from("w3-black");
    Template::render("base", page)
}

#[get("/")]
pub fn root() -> Template {
    let mut page = Page::new("site/blog/index.html");
    page.blog_color = String::from("w3-black");
    let icons: IconList<BlogIcon> = IconList::load_from_dir("site/blog/posts/");
    page.body = icons.export_to_html(3);
    Template::render("base", page)
}
