
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
        dir.push("icon.html");
        match fs::read_to_string(dir.as_path().to_str().unwrap()) {
            Ok(content) => Some(BlogIcon { html : content, ordering: 0 }),
            _ => None
        }
    }
    
    fn get_html(&self) -> String {
        self.html.clone()
    }
}

#[get("/posts/<path>/post.html")]
pub async fn get_post(path: String) -> Template {
    println!( "{:?}", path );
    let mut page = Page::new(&format!("site/projects/listings/{}/about.html", path));
    page.blog_color = String::from("w3-black");
    let mut icons: IconList<BlogIcon> = IconList::load_from_dir("site/blog/posts/");
    icons.icons.sort_by(|a,b| a.ordering.cmp(&b.ordering));
    page.body = icons.export_to_html(3);
    Template::render("base", page )
}

#[get("/")]
pub fn root() -> Template {
    let mut page = Page::new("site/blog/index.html");
    page.blog_color = String::from("w3-black");
    let icons: IconList<BlogIcon> = IconList::load_from_dir("site/blog/posts/");
    page.body = icons.export_to_html(3);
    Template::render("base", page )
}

