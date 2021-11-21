
use rocket::response::content;

use std::fs;

use super::utils::{construct_html_from_base, IconGrid};


#[get("/")]
pub fn index() -> content::Html<String> {
    let posts = IconGrid::load( fs::read_dir("site/blog/posts").unwrap().into_iter()
                .map(|f| {f.unwrap().path().display().to_string()} ).collect() );
    content::Html( construct_html_from_base( posts.export_to_grid() ) )
}

/*
#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("tera/error/404", context! {
        uri: req.uri()
    })
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template("blog/index.html", r#"
        {% extends "tera/base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#).expect("valid Tera template");
}
*/
