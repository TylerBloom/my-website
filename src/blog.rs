
use super::utils::{IconList, Page};

use rocket_dyn_templates::Template;


#[get("/")]
pub fn root() -> Template {
    let mut page = Page::new("site/blog/index.html");
    page.blog_color = String::from("w3-black");
    let icons = IconList::load_from_dir("site/blog/posts/");
    page.body = icons.export_to_html(3);
    Template::render("base", page )
}

