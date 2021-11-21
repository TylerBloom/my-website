#[macro_use] extern crate rocket;

mod statics;
mod blog;
mod projects;
mod utils;

use rocket::response::status::NotFound;
use rocket::fs::NamedFile;

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
        .mount("/", routes![root])
        .mount("/static", routes![statics::find])
        .mount("/blog", routes![blog::index])
        .mount("/projects", routes![projects::index])
        //.register("/projects", catchers![projects::not_found])
        //.register("/blog", catchers![blog::not_found])
        //.register("/statics", catchers![statics::not_found])
}
