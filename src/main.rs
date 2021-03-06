#[macro_use]
extern crate rocket;

mod bio;
mod blog;
mod home;
mod projects;
mod utils;

use rocket::fs::NamedFile;
use rocket_dyn_templates::Template;

use std::path::PathBuf;

#[get("/<path..>")]
pub async fn file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(path).await.ok()
}

#[get("/")]
async fn root() -> Template {
    home::root()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::custom(|engines| {
            utils::customize_tera(&mut engines.tera);
        }))
        .mount("/", routes![root, file])
        .mount("/home/", routes![home::root])
        .mount("/bio/", routes![bio::root])
        .mount("/projects/", routes![projects::root, projects::about])
        .mount("/blog/", routes![blog::root, blog::get_post])
    //.register("/projects", catchers![projects::not_found])
    //.register("/blog", catchers![blog::not_found])
    //.register("/statics", catchers![statics::not_found])
}
