
use super::utils::IconList;

use rocket_dyn_templates::Template;//, tera::Tera, context};

#[get("/")]
pub fn index() -> Template {
    let icons = IconList::load_from_dir("site/projects/");
    Template::render("blog/index", icons.export_to_rows(3) )
}

