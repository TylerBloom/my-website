#[macro_use] extern crate rocket;

mod statics;
mod blog;
//mod projects;

//use rocket_contrib::templates::Template;
use rocket::response::status::NotFound;
use rocket::fs::NamedFile;

#[get("/")]
async fn root() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("site/static/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
        .mount("/", routes![statics::find])
        //.mount("/blog", routes![blog::index])
        //.mount("/projects", routes![projects::index])
        //.register("/projects", catchers![projects::not_found])
        //.register("/blog", catchers![blog::not_found])
        //.register("/statics", catchers![statics::not_found])
}
