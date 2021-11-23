#[macro_use] extern crate rocket;

mod statics;
mod blog;
mod projects;
mod bio;
mod utils;

use rocket::response::status::NotFound;
use rocket::fs::NamedFile;
use rocket_dyn_templates::Template; //, context};

#[get("/")]
async fn root() -> Result<NamedFile, NotFound<String>> {
    println!("Inside main::root");
    NamedFile::open("site/static/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::custom(|engines| { utils::customize_tera(&mut engines.tera); } ))
        .mount("/", routes![root])
        .mount("/static", routes![statics::find])
        .mount("/blog", routes![blog::index])
        .mount("/projects", routes![projects::index])
        .mount("/bio", routes![projects::index])
        //.register("/projects", catchers![projects::not_found])
        //.register("/blog", catchers![blog::not_found])
        //.register("/statics", catchers![statics::not_found])
}
