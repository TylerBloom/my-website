
use super::utils::{IconList, Page};

use rocket_dyn_templates::Template;//, tera::Tera, context};

#[get("/")]
pub fn index() -> Template {
    let mut page = Page::new();
    page.bio_color = String::from("w3-black");
    let icons = IconList::load_from_dir("site/blog/posts/");
    page.body = icons.export_to_html(3);
    Template::render("index", page )
}

