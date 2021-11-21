
use std::fs;

use super::utils::{IconGrid, construct_html_from_base};

use rocket::response::content;

#[get("/")]
pub fn index() -> content::Html<String> {
    let projects = IconGrid::load( fs::read_dir("site/projects/listings").unwrap().into_iter()
                .map(|f| {f.unwrap().path().display().to_string()} ).collect() );
    content::Html( construct_html_from_base(projects.export_to_grid()) )
}

