use super::utils::Page;

use rocket_dyn_templates::Template;

#[get("/")]
pub fn root() -> Template {
    let mut page = Page::new("site/home/index.html");
    page.home_color = String::from("w3-black");
    Template::render("base", page)
}
