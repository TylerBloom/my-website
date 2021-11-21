
use rocket_dyn_templates::Template;
use serde::Serialize;

use std::fs;

#[derive(Serialize)]
struct BlogPosts {
    posts: Vec<String>,
}

#[get("/")]
pub fn index() -> Template {
    let posts = fs::read_dir("site/blog/posts").unwrap().into_iter()
                .map(|f| {f.unwrap().path().display().to_string()} ).collect();
    Template::render("blog/index", BlogPosts{ posts: posts } )
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
